use crate::{Atomizable, Parsable, Writable};

#[derive(Parsable, Writable, Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum PointerEvents {
	Auto, // atom!("auto")
	None, // atom!("none")
}
