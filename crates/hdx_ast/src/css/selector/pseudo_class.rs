use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::{moz::MozPseudoClass, ms::MsPseudoClass, o::OPseudoClass, webkit::WebkitPseudoClass};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PseudoClass {
	Active(T![:], T![Ident]),
	AnyLink(T![:], T![Ident]),
	Autofill(T![:], T![Ident]),
	Blank(T![:], T![Ident]),
	Checked(T![:], T![Ident]),
	Current(T![:], T![Ident]),
	Default(T![:], T![Ident]),
	Defined(T![:], T![Ident]),
	Disabled(T![:], T![Ident]),
	Empty(T![:], T![Ident]),
	Enabled(T![:], T![Ident]),
	First(T![:], T![Ident]),
	FirstChild(T![:], T![Ident]),
	FirstOfType(T![:], T![Ident]),
	Fullscreen(T![:], T![Ident]),
	Future(T![:], T![Ident]),
	Focus(T![:], T![Ident]),
	FocusVisible(T![:], T![Ident]),
	FocusWithin(T![:], T![Ident]),
	Host(T![:], T![Ident]),
	Hover(T![:], T![Ident]),
	Indeterminate(T![:], T![Ident]),
	InRange(T![:], T![Ident]),
	Invalid(T![:], T![Ident]),
	LastChild(T![:], T![Ident]),
	LastOfType(T![:], T![Ident]),
	Left(T![:], T![Ident]),
	Link(T![:], T![Ident]),
	LocalLink(T![:], T![Ident]),
	Modal(T![:], T![Ident]),
	OnlyChild(T![:], T![Ident]),
	OnlyOfType(T![:], T![Ident]),
	Optional(T![:], T![Ident]),
	OutOfRange(T![:], T![Ident]),
	Past(T![:], T![Ident]),
	PictureInPicture(T![:], T![Ident]),
	PlaceholderShown(T![:], T![Ident]),
	PopoverOpen(T![:], T![Ident]),
	Paused(T![:], T![Ident]),
	Playing(T![:], T![Ident]),
	ReadOnly(T![:], T![Ident]),
	ReadWrite(T![:], T![Ident]),
	Required(T![:], T![Ident]),
	Right(T![:], T![Ident]),
	Root(T![:], T![Ident]),
	Scope(T![:], T![Ident]),
	Target(T![:], T![Ident]),
	TargetWithin(T![:], T![Ident]),
	Valid(T![:], T![Ident]),
	Visited(T![:], T![Ident]),
	Webkit(WebkitPseudoClass),
	Moz(MozPseudoClass),
	Ms(MsPseudoClass),
	O(OPseudoClass),
}

impl<'a> Parse<'a> for PseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		let colon = p.parse::<T![:]>()?;
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("active") => Self::Active(colon, ident),
			atom!("any-link") => Self::Active(colon, ident),
			atom!("autofill") => Self::Active(colon, ident),
			atom!("blank") => Self::Active(colon, ident),
			atom!("checked") => Self::Active(colon, ident),
			atom!("current") => Self::Active(colon, ident),
			atom!("default") => Self::Active(colon, ident),
			atom!("defined") => Self::Active(colon, ident),
			atom!("disabled") => Self::Active(colon, ident),
			atom!("empty") => Self::Active(colon, ident),
			atom!("enabled") => Self::Active(colon, ident),
			atom!("first") => Self::Active(colon, ident),
			atom!("first-child") => Self::Active(colon, ident),
			atom!("first-of-type") => Self::Active(colon, ident),
			atom!("fullscreen") => Self::Active(colon, ident),
			atom!("future") => Self::Active(colon, ident),
			atom!("focus") => Self::Active(colon, ident),
			atom!("focus-visible") => Self::Active(colon, ident),
			atom!("focus-within") => Self::Active(colon, ident),
			atom!("host") => Self::Active(colon, ident),
			atom!("hover") => Self::Active(colon, ident),
			atom!("indeterminate") => Self::Active(colon, ident),
			atom!("in-range") => Self::Active(colon, ident),
			atom!("invalid") => Self::Active(colon, ident),
			atom!("last-child") => Self::Active(colon, ident),
			atom!("last-of-type") => Self::Active(colon, ident),
			atom!("left") => Self::Active(colon, ident),
			atom!("link") => Self::Active(colon, ident),
			atom!("local-link") => Self::Active(colon, ident),
			atom!("modal") => Self::Active(colon, ident),
			atom!("only-child") => Self::Active(colon, ident),
			atom!("only-of-type") => Self::Active(colon, ident),
			atom!("optional") => Self::Active(colon, ident),
			atom!("out-of-range") => Self::Active(colon, ident),
			atom!("past") => Self::Active(colon, ident),
			atom!("picture-in-picture") => Self::Active(colon, ident),
			atom!("placeholder-shown") => Self::Active(colon, ident),
			atom!("popover-open") => Self::Active(colon, ident),
			atom!("paused") => Self::Active(colon, ident),
			atom!("playing") => Self::Active(colon, ident),
			atom!("read-only") => Self::Active(colon, ident),
			atom!("read-write") => Self::Active(colon, ident),
			atom!("required") => Self::Active(colon, ident),
			atom!("right") => Self::Active(colon, ident),
			atom!("root") => Self::Active(colon, ident),
			atom!("scope") => Self::Active(colon, ident),
			atom!("target") => Self::Active(colon, ident),
			atom!("target-within") => Self::Active(colon, ident),
			atom!("valid") => Self::Active(colon, ident),
			atom!("visited") => Self::Active(colon, ident),
			atom => {
				p.rewind(checkpoint);
				if let Ok(psuedo) = p.try_parse::<WebkitPseudoClass>() {
					return Ok(Self::Webkit(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<MozPseudoClass>() {
					return Ok(Self::Moz(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<MsPseudoClass>() {
					return Ok(Self::Ms(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<OPseudoClass>() {
					return Ok(Self::O(psuedo));
				}
				Err(diagnostics::UnexpectedPseudoClass(atom, c.into()))?
			}
		})
	}
}

impl<'a> ToCursors for PseudoClass {
	fn to_cursors(&self, s: &mut impl hdx_parser::CursorSink) {
		match self {
			Self::Active(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::AnyLink(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Autofill(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Blank(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Checked(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Current(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Default(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Defined(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Disabled(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Empty(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Enabled(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::First(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FirstChild(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FirstOfType(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Fullscreen(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Future(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Focus(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FocusVisible(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FocusWithin(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Host(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Hover(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Indeterminate(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::InRange(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Invalid(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::LastChild(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::LastOfType(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Left(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Link(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::LocalLink(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Modal(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::OnlyChild(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::OnlyOfType(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Optional(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::OutOfRange(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Past(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::PictureInPicture(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::PlaceholderShown(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::PopoverOpen(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Paused(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Playing(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::ReadOnly(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::ReadWrite(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Required(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Right(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Root(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Scope(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Target(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::TargetWithin(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Valid(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Visited(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Webkit(c) => ToCursors::to_cursors(c, s),
			Self::Moz(c) => ToCursors::to_cursors(c, s),
			Self::Ms(c) => ToCursors::to_cursors(c, s),
			Self::O(c) => ToCursors::to_cursors(c, s),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PseudoClass, 28);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PseudoClass, ":target");
		assert_parse!(PseudoClass, ":scope");
		assert_parse!(PseudoClass, ":valid");
	}
}
