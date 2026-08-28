# csskit_arena

`csskit_arena` is a bump allocator for syntax trees and other short-lived object
graphs. It allocates quickly, releases all storage together, and can preserve
pointer offsets for zero-copy transfer across language bindings.

It's inspired by [bumpalo], and [oxc_allocator], and uses many of the same
techniques as these do. However, csskit_arena is designed to work with
[allocator_api2].

The allocator implements [`allocator_api2::alloc::Allocator`] for `&Arena`, so
it works with all allocator-aware collections from `allocator-api2`.

[bumpalo]: https://github.com/oxc-project/oxc/tree/main/crates/oxc_allocator/
[oxc_allocator]: https://github.com/oxc-project/oxc/tree/main/crates/oxc_allocator/
[allocator_api2]: https://github.com/oxc-project/oxc/tree/main/crates/oxc_allocator/
[`allocator_api2::alloc::Allocator`]: https://docs.rs/allocator-api2/latest/allocator_api2/alloc/trait.Allocator.html

## Limitations

This kind of memory management comes with some trade-offs, which may be
significant depending on your application:

- Total usable capacity is capped at [`MAX_BLOCK_SIZE`] (`2 GiB - 16` bytes).
- Individual allocations cannot be reclaimed.
- Borrowed arenas cannot grow.
- Adding a chunk loses the single-region raw-transfer guarantee.
- The arena is intended for groups of values with a shared lifetime, not
  independently managed long-lived allocations.

For short lived programs with small memory requirements - for example parsing
source code into an AST, none of these matter at all.

## Usage

```rust
use allocator_api2::vec::Vec;
use csskit_arena::Arena;

let arena = Arena::with_capacity(4096);
let mut values = Vec::new_in(&arena);
values.extend([10, 20, 30]);

assert_eq!(&*values, &[10, 20, 30]);
assert!(arena.used_bytes() >= 3 * size_of::<i32>());
```

Allocation advances a cursor, including any padding needed for alignment.
Individual deallocation is a no-op - memory is retained in the Arena until that
is dropped. Allocator-aware owners still drop their values normally, but the
underlying storage remains. The arena can be reset, which rewinds the cursor,
or dropped, which releases any Owned underlying storage to a pool for re-use.

## Features

The crate has no default features. The allocator itself is always available.

- `collections`: the arena-backed `Box`, `Vec`, and `String`, plus the `vec_in!`
  and `format_in!` macros.
- `serde`: serialization for those collections (implies `collections`).

### Why not the `allocator_api2` collections?

`allocator_api2::boxed::Box` and `allocator_api2::vec::Vec` work with `&Arena`
and are the right choice for general use, but the `collections` feature of this
crate provides the same types with some subtle but important differences:

- **Stable layout.** Every type is `#[repr(C)]`. This is important for bindings,
  as the layout can be read from other runtimes.
- **32-bit lengths.** `Vec` and `String` count elements in `u32`, halving the
  header size of an AST node. This makes sense given the Arena's limitation of
  ~2GB owned size, but this comes with the perhaps obvious limitation that
  these cannot grow above `u32::MAX` elements.
- **No deallocation.** Dropping a collection never calls `deallocate`, because
  the arena releases everything at once. `Vec` and `String` also never run
  element destructors, so `T` must not own resources outside the arena. `Box`
  does run the destructor of the value it holds.
- **Panic, not abort.** Exhausting the arena panics, which parsing can unwind
  from. The `allocator_api2` collections call `handle_alloc_error`, which
  aborts the process.
- **`Sized` only.** There is no unsizing coercion, so no `Box<dyn Trait>` and no
  `Box<[T]>`. Cloning is supported, since each value keeps its allocator.

## Raw transfer

On supported targets, the arena reserves one region beginning at a 4 GiB-aligned
address. Its usable size is limited to just under 2 GiB. Consequently, every
pointer inside that region has the same upper 32 bits, while its lower 32 bits
are the byte offset from the region's base:

```text
pointer = transfer base | 32-bit offset
```

A binding can transfer the arena buffer once, then encode interior pointers as
`u32` offsets rather than serialising the whole object graph.

Always query `Arena::transfer_base()` before using this representation:

```rust
use csskit_arena::Arena;

let arena = Arena::new();
match arena.transfer_base() {
    Some(base) => {
        // Interior pointers may be encoded by their low 32 bits.
        assert_eq!(base & u32::MAX as usize, 0);
    }
    None => {
        // Use full pointers or another transfer format.
    }
}
```

`transfer_base()` returns `None` when no single aligned region covers every
allocation, including after a growable arena adds another chunk. Treat its
result as the authority rather than assuming support from the target platform.

## Ownership modes

| Constructor                               | Backing storage | Can grow? | Intended use                             |
| ----------------------------------------- | --------------- | --------- | ---------------------------------------- |
| `Arena::new()` / `default()`              | Arena-owned     | Yes       | Unknown final size                       |
| `Arena::with_capacity(size)`              | Arena-owned     | Yes       | Input provides a useful size hint        |
| `unsafe Arena::from_raw_parts(ptr, size)` | Caller-owned    | No        | A binding already owns a suitable buffer |

`from_raw_parts` is unsafe. Its pointer must identify a live, writable region
which outlives the arena, is aligned to [`BLOCK_ALIGN`], is no larger than
[`MAX_BLOCK_SIZE`], and is not handed to another allocator. The arena never
frees borrowed memory. Exhausting borrowed storage returns `AllocError` rather
than adding a chunk.

[`BLOCK_ALIGN`]: https://docs.rs/csskit_arena/latest/csskit_arena/constant.BLOCK_ALIGN.html
[`MAX_BLOCK_SIZE`]: https://docs.rs/csskit_arena/latest/csskit_arena/constant.MAX_BLOCK_SIZE.html

## Reuse

`reset()` releases every allocation together, rewinds the cursor, and retains
the first chunk for reuse:

```rust
use allocator_api2::vec::Vec;
use csskit_arena::Arena;

let mut arena = Arena::with_capacity(4096);

{
    let mut values = Vec::new_in(&arena);
    values.extend(0..100);
    assert!(arena.used_bytes() > 0);
}

arena.reset();
assert_eq!(arena.used_bytes(), 0);
```

The method requires `&mut Arena`, preventing safe references into the arena from
remaining live across the reset. Raw pointers and binding-layer offsets must
also be discarded: all previous allocations become invalid.

Dropping or resetting an arena does not discover and run destructors for values
still stored in it. Drop allocator-aware owners before reset when their elements
manage resources.

## Allocation behaviour

`&Arena` implements the allocator operations as follows:

- `allocate` aligns the current cursor and advances it by the requested size;
- zero-sized allocations receive an aligned pointer inside the current region
  without moving the cursor;
- `deallocate` does nothing;
- `grow` extends the latest allocation in place when possible, otherwise it
  allocates new storage and copies the old bytes;
- exhausting an owned chunk adds a larger chunk while the arena remains below
  [`MAX_BLOCK_SIZE`].

`used_bytes()` reports bytes handed out across all chunks. `capacity()` reports
total usable capacity across those chunks. Alignment padding can make used
storage larger than the sum of requested object sizes.

## Platform behaviour

On 64-bit Unix and Windows, the allocator attempts to reserve aligned virtual
address space. Reservation does not commit this actual reservation, instead:

- Unix relies on demand paging;
- Windows commits pages incrementally as allocations reach them.

Released reservations are cached in a small thread-local pool. Reusing them
avoids repeated virtual-memory system calls, which makes it cheap to drop and
re-allocate (useful for hot methods which might need an Arena).

Where aligned virtual-memory reservation is unavailable, the arena falls back
to growable chunks from the global allocator. On 32-bit targets, pointers are
already representable as 32-bit offsets from zero. On 64-bit targets without a
usable reservation, `transfer_base()` returns `None`.

The 4 GiB-aligned reservation technique is adapted from [oxc_allocator].

## Part of csskit

This crate is part of [csskit], a comprehensive CSS tool chain.

[csskit]: https://csskit.rs/

## License

MIT
