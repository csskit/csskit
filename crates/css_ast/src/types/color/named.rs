use crate::CssAtomSet;
use css_parse::T;
use csskit_derives::*;

#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum NamedColor {
	#[atom(CssAtomSet::Aliceblue)]
	Aliceblue(T![Ident]),
	#[atom(CssAtomSet::Antiquewhite)]
	Antiquewhite(T![Ident]),
	#[atom(CssAtomSet::Aqua)]
	Aqua(T![Ident]),
	#[atom(CssAtomSet::Aquamarine)]
	Aquamarine(T![Ident]),
	#[atom(CssAtomSet::Azure)]
	Azure(T![Ident]),
	#[atom(CssAtomSet::Beige)]
	Beige(T![Ident]),
	#[atom(CssAtomSet::Bisque)]
	Bisque(T![Ident]),
	#[atom(CssAtomSet::Black)]
	Black(T![Ident]),
	#[atom(CssAtomSet::Blanchedalmond)]
	Blanchedalmond(T![Ident]),
	#[atom(CssAtomSet::Blue)]
	Blue(T![Ident]),
	#[atom(CssAtomSet::Blueviolet)]
	Blueviolet(T![Ident]),
	#[atom(CssAtomSet::Brown)]
	Brown(T![Ident]),
	#[atom(CssAtomSet::Burlywood)]
	Burlywood(T![Ident]),
	#[atom(CssAtomSet::Cadetblue)]
	Cadetblue(T![Ident]),
	#[atom(CssAtomSet::Chartreuse)]
	Chartreuse(T![Ident]),
	#[atom(CssAtomSet::Chocolate)]
	Chocolate(T![Ident]),
	#[atom(CssAtomSet::Coral)]
	Coral(T![Ident]),
	#[atom(CssAtomSet::Cornflowerblue)]
	Cornflowerblue(T![Ident]),
	#[atom(CssAtomSet::Cornsilk)]
	Cornsilk(T![Ident]),
	#[atom(CssAtomSet::Crimson)]
	Crimson(T![Ident]),
	#[atom(CssAtomSet::Cyan)]
	Cyan(T![Ident]),
	#[atom(CssAtomSet::Darkblue)]
	Darkblue(T![Ident]),
	#[atom(CssAtomSet::Darkcyan)]
	Darkcyan(T![Ident]),
	#[atom(CssAtomSet::Darkgoldenrod)]
	Darkgoldenrod(T![Ident]),
	#[atom(CssAtomSet::Darkgray)]
	Darkgray(T![Ident]),
	#[atom(CssAtomSet::Darkgreen)]
	Darkgreen(T![Ident]),
	#[atom(CssAtomSet::Darkgrey)]
	Darkgrey(T![Ident]),
	#[atom(CssAtomSet::Darkkhaki)]
	Darkkhaki(T![Ident]),
	#[atom(CssAtomSet::Darkmagenta)]
	Darkmagenta(T![Ident]),
	#[atom(CssAtomSet::Darkolivegreen)]
	Darkolivegreen(T![Ident]),
	#[atom(CssAtomSet::Darkorange)]
	Darkorange(T![Ident]),
	#[atom(CssAtomSet::Darkorchid)]
	Darkorchid(T![Ident]),
	#[atom(CssAtomSet::Darkred)]
	Darkred(T![Ident]),
	#[atom(CssAtomSet::Darksalmon)]
	Darksalmon(T![Ident]),
	#[atom(CssAtomSet::Darkseagreen)]
	Darkseagreen(T![Ident]),
	#[atom(CssAtomSet::Darkslateblue)]
	Darkslateblue(T![Ident]),
	#[atom(CssAtomSet::Darkslategray)]
	Darkslategray(T![Ident]),
	#[atom(CssAtomSet::Darkslategrey)]
	Darkslategrey(T![Ident]),
	#[atom(CssAtomSet::Darkturquoise)]
	Darkturquoise(T![Ident]),
	#[atom(CssAtomSet::Darkviolet)]
	Darkviolet(T![Ident]),
	#[atom(CssAtomSet::Deeppink)]
	Deeppink(T![Ident]),
	#[atom(CssAtomSet::Deepskyblue)]
	Deepskyblue(T![Ident]),
	#[atom(CssAtomSet::Dimgray)]
	Dimgray(T![Ident]),
	#[atom(CssAtomSet::Dimgrey)]
	Dimgrey(T![Ident]),
	#[atom(CssAtomSet::Dodgerblue)]
	Dodgerblue(T![Ident]),
	#[atom(CssAtomSet::Firebrick)]
	Firebrick(T![Ident]),
	#[atom(CssAtomSet::Floralwhite)]
	Floralwhite(T![Ident]),
	#[atom(CssAtomSet::Forestgreen)]
	Forestgreen(T![Ident]),
	#[atom(CssAtomSet::Fuchsia)]
	Fuchsia(T![Ident]),
	#[atom(CssAtomSet::Gainsboro)]
	Gainsboro(T![Ident]),
	#[atom(CssAtomSet::Ghostwhite)]
	Ghostwhite(T![Ident]),
	#[atom(CssAtomSet::Gold)]
	Gold(T![Ident]),
	#[atom(CssAtomSet::Goldenrod)]
	Goldenrod(T![Ident]),
	#[atom(CssAtomSet::Gray)]
	Gray(T![Ident]),
	#[atom(CssAtomSet::Green)]
	Green(T![Ident]),
	#[atom(CssAtomSet::Greenyellow)]
	Greenyellow(T![Ident]),
	#[atom(CssAtomSet::Grey)]
	Grey(T![Ident]),
	#[atom(CssAtomSet::Honeydew)]
	Honeydew(T![Ident]),
	#[atom(CssAtomSet::Hotpink)]
	Hotpink(T![Ident]),
	#[atom(CssAtomSet::Indianred)]
	Indianred(T![Ident]),
	#[atom(CssAtomSet::Indigo)]
	Indigo(T![Ident]),
	#[atom(CssAtomSet::Ivory)]
	Ivory(T![Ident]),
	#[atom(CssAtomSet::Khaki)]
	Khaki(T![Ident]),
	#[atom(CssAtomSet::Lavender)]
	Lavender(T![Ident]),
	#[atom(CssAtomSet::Lavenderblush)]
	Lavenderblush(T![Ident]),
	#[atom(CssAtomSet::Lawngreen)]
	Lawngreen(T![Ident]),
	#[atom(CssAtomSet::Lemonchiffon)]
	Lemonchiffon(T![Ident]),
	#[atom(CssAtomSet::Lightblue)]
	Lightblue(T![Ident]),
	#[atom(CssAtomSet::Lightcoral)]
	Lightcoral(T![Ident]),
	#[atom(CssAtomSet::Lightcyan)]
	Lightcyan(T![Ident]),
	#[atom(CssAtomSet::Lightgoldenrodyellow)]
	Lightgoldenrodyellow(T![Ident]),
	#[atom(CssAtomSet::Lightgray)]
	Lightgray(T![Ident]),
	#[atom(CssAtomSet::Lightgreen)]
	Lightgreen(T![Ident]),
	#[atom(CssAtomSet::Lightgrey)]
	Lightgrey(T![Ident]),
	#[atom(CssAtomSet::Lightpink)]
	Lightpink(T![Ident]),
	#[atom(CssAtomSet::Lightsalmon)]
	Lightsalmon(T![Ident]),
	#[atom(CssAtomSet::Lightseagreen)]
	Lightseagreen(T![Ident]),
	#[atom(CssAtomSet::Lightskyblue)]
	Lightskyblue(T![Ident]),
	#[atom(CssAtomSet::Lightslategray)]
	Lightslategray(T![Ident]),
	#[atom(CssAtomSet::Lightslategrey)]
	Lightslategrey(T![Ident]),
	#[atom(CssAtomSet::Lightsteelblue)]
	Lightsteelblue(T![Ident]),
	#[atom(CssAtomSet::Lightyellow)]
	Lightyellow(T![Ident]),
	#[atom(CssAtomSet::Lime)]
	Lime(T![Ident]),
	#[atom(CssAtomSet::Limegreen)]
	Limegreen(T![Ident]),
	#[atom(CssAtomSet::Linen)]
	Linen(T![Ident]),
	#[atom(CssAtomSet::Magenta)]
	Magenta(T![Ident]),
	#[atom(CssAtomSet::Maroon)]
	Maroon(T![Ident]),
	#[atom(CssAtomSet::Mediumaquamarine)]
	Mediumaquamarine(T![Ident]),
	#[atom(CssAtomSet::Mediumblue)]
	Mediumblue(T![Ident]),
	#[atom(CssAtomSet::Mediumorchid)]
	Mediumorchid(T![Ident]),
	#[atom(CssAtomSet::Mediumpurple)]
	Mediumpurple(T![Ident]),
	#[atom(CssAtomSet::Mediumseagreen)]
	Mediumseagreen(T![Ident]),
	#[atom(CssAtomSet::Mediumslateblue)]
	Mediumslateblue(T![Ident]),
	#[atom(CssAtomSet::Mediumspringgreen)]
	Mediumspringgreen(T![Ident]),
	#[atom(CssAtomSet::Mediumturquoise)]
	Mediumturquoise(T![Ident]),
	#[atom(CssAtomSet::Mediumvioletred)]
	Mediumvioletred(T![Ident]),
	#[atom(CssAtomSet::Midnightblue)]
	Midnightblue(T![Ident]),
	#[atom(CssAtomSet::Mintcream)]
	Mintcream(T![Ident]),
	#[atom(CssAtomSet::Mistyrose)]
	Mistyrose(T![Ident]),
	#[atom(CssAtomSet::Moccasin)]
	Moccasin(T![Ident]),
	#[atom(CssAtomSet::Navajowhite)]
	Navajowhite(T![Ident]),
	#[atom(CssAtomSet::Navy)]
	Navy(T![Ident]),
	#[atom(CssAtomSet::Oldlace)]
	Oldlace(T![Ident]),
	#[atom(CssAtomSet::Olive)]
	Olive(T![Ident]),
	#[atom(CssAtomSet::Olivedrab)]
	Olivedrab(T![Ident]),
	#[atom(CssAtomSet::Orange)]
	Orange(T![Ident]),
	#[atom(CssAtomSet::Orangered)]
	Orangered(T![Ident]),
	#[atom(CssAtomSet::Orchid)]
	Orchid(T![Ident]),
	#[atom(CssAtomSet::Palegoldenrod)]
	Palegoldenrod(T![Ident]),
	#[atom(CssAtomSet::Palegreen)]
	Palegreen(T![Ident]),
	#[atom(CssAtomSet::Paleturquoise)]
	Paleturquoise(T![Ident]),
	#[atom(CssAtomSet::Palevioletred)]
	Palevioletred(T![Ident]),
	#[atom(CssAtomSet::Papayawhip)]
	Papayawhip(T![Ident]),
	#[atom(CssAtomSet::Peachpuff)]
	Peachpuff(T![Ident]),
	#[atom(CssAtomSet::Peru)]
	Peru(T![Ident]),
	#[atom(CssAtomSet::Pink)]
	Pink(T![Ident]),
	#[atom(CssAtomSet::Plum)]
	Plum(T![Ident]),
	#[atom(CssAtomSet::Powderblue)]
	Powderblue(T![Ident]),
	#[atom(CssAtomSet::Purple)]
	Purple(T![Ident]),
	#[atom(CssAtomSet::Rebeccapurple)]
	Rebeccapurple(T![Ident]),
	#[atom(CssAtomSet::Red)]
	Red(T![Ident]),
	#[atom(CssAtomSet::Rosybrown)]
	Rosybrown(T![Ident]),
	#[atom(CssAtomSet::Royalblue)]
	Royalblue(T![Ident]),
	#[atom(CssAtomSet::Saddlebrown)]
	Saddlebrown(T![Ident]),
	#[atom(CssAtomSet::Salmon)]
	Salmon(T![Ident]),
	#[atom(CssAtomSet::Sandybrown)]
	Sandybrown(T![Ident]),
	#[atom(CssAtomSet::Seagreen)]
	Seagreen(T![Ident]),
	#[atom(CssAtomSet::Seashell)]
	Seashell(T![Ident]),
	#[atom(CssAtomSet::Sienna)]
	Sienna(T![Ident]),
	#[atom(CssAtomSet::Silver)]
	Silver(T![Ident]),
	#[atom(CssAtomSet::Skyblue)]
	Skyblue(T![Ident]),
	#[atom(CssAtomSet::Slateblue)]
	Slateblue(T![Ident]),
	#[atom(CssAtomSet::Slategray)]
	Slategray(T![Ident]),
	#[atom(CssAtomSet::Slategrey)]
	Slategrey(T![Ident]),
	#[atom(CssAtomSet::Snow)]
	Snow(T![Ident]),
	#[atom(CssAtomSet::Springgreen)]
	Springgreen(T![Ident]),
	#[atom(CssAtomSet::Steelblue)]
	Steelblue(T![Ident]),
	#[atom(CssAtomSet::Tan)]
	Tan(T![Ident]),
	#[atom(CssAtomSet::Teal)]
	Teal(T![Ident]),
	#[atom(CssAtomSet::Thistle)]
	Thistle(T![Ident]),
	#[atom(CssAtomSet::Tomato)]
	Tomato(T![Ident]),
	#[atom(CssAtomSet::Turquoise)]
	Turquoise(T![Ident]),
	#[atom(CssAtomSet::Violet)]
	Violet(T![Ident]),
	#[atom(CssAtomSet::Wheat)]
	Wheat(T![Ident]),
	#[atom(CssAtomSet::White)]
	White(T![Ident]),
	#[atom(CssAtomSet::Whitesmoke)]
	Whitesmoke(T![Ident]),
	#[atom(CssAtomSet::Yellow)]
	Yellow(T![Ident]),
	#[atom(CssAtomSet::Yellowgreen)]
	Yellowgreen(T![Ident]),
}

#[cfg(feature = "chromashift")]
impl super::ToChromashift for NamedColor {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::{Color, Named};
		Some(Color::Named(match self {
			NamedColor::Aliceblue(_) => Named::Aliceblue,
			NamedColor::Antiquewhite(_) => Named::Antiquewhite,
			NamedColor::Aqua(_) => Named::Aqua,
			NamedColor::Aquamarine(_) => Named::Aquamarine,
			NamedColor::Azure(_) => Named::Azure,
			NamedColor::Beige(_) => Named::Beige,
			NamedColor::Bisque(_) => Named::Bisque,
			NamedColor::Black(_) => Named::Black,
			NamedColor::Blanchedalmond(_) => Named::Blanchedalmond,
			NamedColor::Blue(_) => Named::Blue,
			NamedColor::Blueviolet(_) => Named::Blueviolet,
			NamedColor::Brown(_) => Named::Brown,
			NamedColor::Burlywood(_) => Named::Burlywood,
			NamedColor::Cadetblue(_) => Named::Cadetblue,
			NamedColor::Chartreuse(_) => Named::Chartreuse,
			NamedColor::Chocolate(_) => Named::Chocolate,
			NamedColor::Coral(_) => Named::Coral,
			NamedColor::Cornflowerblue(_) => Named::Cornflowerblue,
			NamedColor::Cornsilk(_) => Named::Cornsilk,
			NamedColor::Crimson(_) => Named::Crimson,
			NamedColor::Cyan(_) => Named::Cyan,
			NamedColor::Darkblue(_) => Named::Darkblue,
			NamedColor::Darkcyan(_) => Named::Darkcyan,
			NamedColor::Darkgoldenrod(_) => Named::Darkgoldenrod,
			NamedColor::Darkgray(_) => Named::Darkgray,
			NamedColor::Darkgrey(_) => Named::Darkgrey,
			NamedColor::Darkgreen(_) => Named::Darkgreen,
			NamedColor::Darkkhaki(_) => Named::Darkkhaki,
			NamedColor::Darkmagenta(_) => Named::Darkmagenta,
			NamedColor::Darkolivegreen(_) => Named::Darkolivegreen,
			NamedColor::Darkorange(_) => Named::Darkorange,
			NamedColor::Darkorchid(_) => Named::Darkorchid,
			NamedColor::Darkred(_) => Named::Darkred,
			NamedColor::Darksalmon(_) => Named::Darksalmon,
			NamedColor::Darkseagreen(_) => Named::Darkseagreen,
			NamedColor::Darkslateblue(_) => Named::Darkslateblue,
			NamedColor::Darkslategray(_) => Named::Darkslategray,
			NamedColor::Darkslategrey(_) => Named::Darkslategrey,
			NamedColor::Darkturquoise(_) => Named::Darkturquoise,
			NamedColor::Darkviolet(_) => Named::Darkviolet,
			NamedColor::Deeppink(_) => Named::Deeppink,
			NamedColor::Deepskyblue(_) => Named::Deepskyblue,
			NamedColor::Dimgray(_) => Named::Dimgrey,
			NamedColor::Dimgrey(_) => Named::Dimgray,
			NamedColor::Dodgerblue(_) => Named::Dodgerblue,
			NamedColor::Firebrick(_) => Named::Firebrick,
			NamedColor::Floralwhite(_) => Named::Floralwhite,
			NamedColor::Forestgreen(_) => Named::Forestgreen,
			NamedColor::Fuchsia(_) => Named::Fuchsia,
			NamedColor::Gainsboro(_) => Named::Gainsboro,
			NamedColor::Ghostwhite(_) => Named::Ghostwhite,
			NamedColor::Gold(_) => Named::Gold,
			NamedColor::Goldenrod(_) => Named::Goldenrod,
			NamedColor::Gray(_) => Named::Gray,
			NamedColor::Grey(_) => Named::Grey,
			NamedColor::Green(_) => Named::Green,
			NamedColor::Greenyellow(_) => Named::Greenyellow,
			NamedColor::Honeydew(_) => Named::Honeydew,
			NamedColor::Hotpink(_) => Named::Hotpink,
			NamedColor::Indianred(_) => Named::Indianred,
			NamedColor::Indigo(_) => Named::Indigo,
			NamedColor::Ivory(_) => Named::Ivory,
			NamedColor::Khaki(_) => Named::Khaki,
			NamedColor::Lavender(_) => Named::Lavender,
			NamedColor::Lavenderblush(_) => Named::Lavenderblush,
			NamedColor::Lawngreen(_) => Named::Lawngreen,
			NamedColor::Lemonchiffon(_) => Named::Lemonchiffon,
			NamedColor::Lightblue(_) => Named::Lightblue,
			NamedColor::Lightcoral(_) => Named::Lightcoral,
			NamedColor::Lightcyan(_) => Named::Lightcyan,
			NamedColor::Lightgoldenrodyellow(_) => Named::Lightgoldenrodyellow,
			NamedColor::Lightgray(_) | NamedColor::Lightgrey(_) => Named::Lightgray,
			NamedColor::Lightgreen(_) => Named::Lightgreen,
			NamedColor::Lightpink(_) => Named::Lightpink,
			NamedColor::Lightsalmon(_) => Named::Lightsalmon,
			NamedColor::Lightseagreen(_) => Named::Lightseagreen,
			NamedColor::Lightskyblue(_) => Named::Lightskyblue,
			NamedColor::Lightslategray(_) => Named::Lightslategray,
			NamedColor::Lightslategrey(_) => Named::Lightslategrey,
			NamedColor::Lightsteelblue(_) => Named::Lightsteelblue,
			NamedColor::Lightyellow(_) => Named::Lightyellow,
			NamedColor::Lime(_) => Named::Lime,
			NamedColor::Limegreen(_) => Named::Limegreen,
			NamedColor::Linen(_) => Named::Linen,
			NamedColor::Magenta(_) => Named::Magenta,
			NamedColor::Maroon(_) => Named::Maroon,
			NamedColor::Mediumaquamarine(_) => Named::Mediumaquamarine,
			NamedColor::Mediumblue(_) => Named::Mediumblue,
			NamedColor::Mediumorchid(_) => Named::Mediumorchid,
			NamedColor::Mediumpurple(_) => Named::Mediumpurple,
			NamedColor::Mediumseagreen(_) => Named::Mediumseagreen,
			NamedColor::Mediumslateblue(_) => Named::Mediumslateblue,
			NamedColor::Mediumspringgreen(_) => Named::Mediumspringgreen,
			NamedColor::Mediumturquoise(_) => Named::Mediumturquoise,
			NamedColor::Mediumvioletred(_) => Named::Mediumvioletred,
			NamedColor::Midnightblue(_) => Named::Midnightblue,
			NamedColor::Mintcream(_) => Named::Mintcream,
			NamedColor::Mistyrose(_) => Named::Mistyrose,
			NamedColor::Moccasin(_) => Named::Moccasin,
			NamedColor::Navajowhite(_) => Named::Navajowhite,
			NamedColor::Navy(_) => Named::Navy,
			NamedColor::Oldlace(_) => Named::Oldlace,
			NamedColor::Olive(_) => Named::Olive,
			NamedColor::Olivedrab(_) => Named::Olivedrab,
			NamedColor::Orange(_) => Named::Orange,
			NamedColor::Orangered(_) => Named::Orangered,
			NamedColor::Orchid(_) => Named::Orchid,
			NamedColor::Palegoldenrod(_) => Named::Palegoldenrod,
			NamedColor::Palegreen(_) => Named::Palegreen,
			NamedColor::Paleturquoise(_) => Named::Paleturquoise,
			NamedColor::Palevioletred(_) => Named::Palevioletred,
			NamedColor::Papayawhip(_) => Named::Papayawhip,
			NamedColor::Peachpuff(_) => Named::Peachpuff,
			NamedColor::Peru(_) => Named::Peru,
			NamedColor::Pink(_) => Named::Pink,
			NamedColor::Plum(_) => Named::Plum,
			NamedColor::Powderblue(_) => Named::Powderblue,
			NamedColor::Purple(_) => Named::Purple,
			NamedColor::Rebeccapurple(_) => Named::Rebeccapurple,
			NamedColor::Red(_) => Named::Red,
			NamedColor::Rosybrown(_) => Named::Rosybrown,
			NamedColor::Royalblue(_) => Named::Royalblue,
			NamedColor::Saddlebrown(_) => Named::Saddlebrown,
			NamedColor::Salmon(_) => Named::Salmon,
			NamedColor::Sandybrown(_) => Named::Sandybrown,
			NamedColor::Seagreen(_) => Named::Seagreen,
			NamedColor::Seashell(_) => Named::Seashell,
			NamedColor::Sienna(_) => Named::Sienna,
			NamedColor::Silver(_) => Named::Silver,
			NamedColor::Skyblue(_) => Named::Skyblue,
			NamedColor::Slateblue(_) => Named::Slateblue,
			NamedColor::Slategray(_) => Named::Slategray,
			NamedColor::Slategrey(_) => Named::Slategrey,
			NamedColor::Snow(_) => Named::Snow,
			NamedColor::Springgreen(_) => Named::Springgreen,
			NamedColor::Steelblue(_) => Named::Steelblue,
			NamedColor::Tan(_) => Named::Tan,
			NamedColor::Teal(_) => Named::Teal,
			NamedColor::Thistle(_) => Named::Thistle,
			NamedColor::Tomato(_) => Named::Tomato,
			NamedColor::Turquoise(_) => Named::Turquoise,
			NamedColor::Violet(_) => Named::Violet,
			NamedColor::Wheat(_) => Named::Wheat,
			NamedColor::White(_) => Named::White,
			NamedColor::Whitesmoke(_) => Named::Whitesmoke,
			NamedColor::Yellow(_) => Named::Yellow,
			NamedColor::Yellowgreen(_) => Named::Yellowgreen,
		}))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NamedColor>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, NamedColor, "yellow");
		assert_parse!(CssAtomSet::ATOMS, NamedColor, "tomato");
		assert_parse!(CssAtomSet::ATOMS, NamedColor, "tan");
	}
}
