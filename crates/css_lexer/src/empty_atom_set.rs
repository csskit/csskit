use derive_atom_set::AtomSet as DeriveAtomSet;

/// This enum represents an empty AtomSet. It can be used to Lex code when you're not interested in capturing known
/// keywords.
#[derive(Debug, Default, Copy, Clone, PartialEq, DeriveAtomSet)]
pub enum EmptyAtomSet {
	#[default]
	_None,
}

impl EmptyAtomSet {
	pub const ATOMS: EmptyAtomSet = EmptyAtomSet::_None;
}
