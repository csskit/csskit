//! A thread-local cache of arena reservations.
//!
//! Reserving address space is not free. A process that parses one document never notices, but a server or a CLI that
//! parses thousands pays it thousands of times, which is what makes a short-lived [`Arena`][crate::Arena] cost more
//! than the parse it serves.
//!
//! So a reservation is not given back to the OS when its arena is dropped: it is kept here, on the thread that made
//! it, and the next arena of the same size takes it. Nothing is shared between threads, thus no lock is needed, and
//! the pages stay resident, thus the next parse writes to memory it has already faulted in.
//!
//! A thread keeps at most [`CAPACITY`] reservations, and hands the pages of anything bigger than [`KEEP`] straight
//! back, so a thread that once parsed a huge document does not hold its memory for the life of the process.

use crate::vm::{self, Reservation};
use std::cell::RefCell;
use std::ptr::NonNull;

/// How many reservations one thread keeps.
const CAPACITY: usize = 4;

/// How many bytes of a returned reservation stay resident. Beyond this the pages go back to the OS.
const KEEP: usize = 4 * 1024 * 1024;

/// The reservations this thread is holding. Dropping it hands them all back, so a thread that ends does not leak
/// what it kept.
struct Cache {
	slots: [Option<Reservation>; CAPACITY],
}

impl Cache {
	const fn new() -> Self {
		Self { slots: [None; CAPACITY] }
	}

	#[cfg(test)]
	fn len(&self) -> usize {
		self.slots.iter().filter(|slot| slot.is_some()).count()
	}

	fn take_exact(&mut self, size: usize) -> Option<Reservation> {
		self.slots
			.iter_mut()
			.find(|slot| slot.as_ref().is_some_and(|reservation| reservation.size == size))
			.and_then(Option::take)
	}

	fn vacant(&mut self) -> Option<&mut Option<Reservation>> {
		self.slots.iter_mut().find(|slot| slot.is_none())
	}
}

impl Drop for Cache {
	fn drop(&mut self) {
		for slot in &mut self.slots {
			if let Some(reservation) = slot.take() {
				// SAFETY: a reservation in the cache belongs to no arena, thus nothing is allocated from it.
				unsafe { vm::release(reservation.reservation, reservation.reserved) };
			}
		}
	}
}

thread_local! {
	static FREE: RefCell<Cache> = const { RefCell::new(Cache::new()) };
}

/// A reservation of exactly `size` usable bytes, if this thread is holding one.
pub(crate) fn take(size: usize) -> Option<Reservation> {
	FREE.try_with(|free| {
		let mut free = free.try_borrow_mut().ok()?;
		free.take_exact(size)
	})
	.ok()
	.flatten()
}

/// Keep `reservation` for the next arena, and say whether it was kept. `used` is how many bytes of it were handed
/// out, which is how much of it is resident.
///
/// # Safety
/// Nothing allocated from the reservation may still be in use, and the caller must release the reservation itself
/// where this gives back `false`.
pub(crate) unsafe fn give(reservation: Reservation, used: usize) -> bool {
	FREE.try_with(|free| {
		let Ok(mut free) = free.try_borrow_mut() else {
			return false;
		};
		let Some(slot) = free.vacant() else {
			return false;
		};
		if let Some(tail) = used.checked_sub(KEEP).filter(|tail| *tail > 0) {
			// Only what a document of ordinary size would not have touched goes back: the first [`KEEP`] bytes stay
			// resident, so the next parse writes to pages it already has, and one huge document does not leave its
			// memory held for the life of the thread.
			// SAFETY: the caller guarantees nothing allocated from the reservation is live, and `used` is within it.
			let tail_base = unsafe { NonNull::new_unchecked(reservation.base.as_ptr().add(KEEP)) };
			// SAFETY: as above.
			unsafe { vm::discard(tail_base, tail) };
		}
		*slot = Some(reservation);
		true
	})
	.unwrap_or(false)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{Arena, MAX_BLOCK_SIZE};

	fn isolated(test: impl FnOnce() + Send + 'static) {
		std::thread::spawn(test).join().expect("the test thread panicked");
	}

	#[test]
	fn an_arena_reuses_the_region_the_last_one_left() {
		if !vm::RESERVABLE {
			return;
		}
		isolated(|| {
			let first = {
				let arena = Arena::new();
				arena.base_ptr()
			};
			let arena = Arena::new();
			assert_eq!(arena.base_ptr(), first, "the second arena took the first one's reservation");
			assert_eq!(arena.capacity(), MAX_BLOCK_SIZE);
			assert_eq!(arena.used_bytes(), 0, "a reused arena starts empty");
		});
	}

	#[test]
	fn a_reused_region_still_allocates() {
		if !vm::RESERVABLE {
			return;
		}
		isolated(|| {
			let written = {
				let arena = Arena::new();
				let mut vec = allocator_api2::vec::Vec::new_in(&arena);
				vec.extend_from_slice(&[7u8; 4096]);
				vec.iter().map(|byte| *byte as usize).sum::<usize>()
			};
			let arena = Arena::new();
			let mut vec = allocator_api2::vec::Vec::new_in(&arena);
			vec.extend_from_slice(&[9u8; 4096]);
			assert_eq!(vec.iter().map(|byte| *byte as usize).sum::<usize>(), 4096 * 9);
			assert_eq!(written, 4096 * 7);
		});
	}

	#[test]
	fn a_thread_keeps_no_more_than_the_cap() {
		if !vm::RESERVABLE {
			return;
		}
		isolated(|| {
			let arenas: Vec<Arena> = (0..CAPACITY + 2).map(|_| Arena::new()).collect();
			drop(arenas);
			let kept = FREE.with(|free| free.borrow().len());
			assert_eq!(kept, CAPACITY, "a thread keeps the cap and no more");
		});
	}

	#[test]
	fn a_size_the_pool_does_not_hold_is_reserved_fresh() {
		if !vm::RESERVABLE {
			return;
		}
		isolated(|| {
			drop(Arena::new());
			assert!(take(MAX_BLOCK_SIZE - 4096).is_none(), "only an exact size matches");
			assert!(take(MAX_BLOCK_SIZE).is_some(), "the full size is the one that was kept");
		});
	}

	#[test]
	fn a_reset_does_not_hide_what_the_arena_touched() {
		if !vm::RESERVABLE {
			return;
		}
		isolated(|| {
			let mut arena = Arena::new();
			{
				let mut vec = allocator_api2::vec::Vec::new_in(&arena);
				vec.extend_from_slice(&[1u8; KEEP * 2]);
				assert_eq!(vec.len(), KEEP * 2);
			}
			arena.reset();
			assert_eq!(arena.used_bytes(), 0, "a reset rewinds the cursor");
			assert!(arena.resident_bytes() >= KEEP * 2, "a reset does not un-touch the pages behind the cursor");
		});
	}

	#[test]
	fn a_region_whose_tail_was_discarded_still_allocates() {
		if !vm::RESERVABLE {
			return;
		}
		isolated(|| {
			{
				let arena = Arena::new();
				let mut vec = allocator_api2::vec::Vec::new_in(&arena);
				vec.extend_from_slice(&[3u8; KEEP * 2]);
				assert_eq!(vec.len(), KEEP * 2);
			}
			assert_eq!(FREE.with(|free| free.borrow().len()), 1, "the reservation was kept");
			let arena = Arena::new();
			let mut vec = allocator_api2::vec::Vec::new_in(&arena);
			vec.extend_from_slice(&[5u8; KEEP * 2]);
			assert!(vec.iter().all(|byte| *byte == 5), "writes across the discarded boundary hold");
			assert_eq!(vec.len(), KEEP * 2);
		});
	}
}
