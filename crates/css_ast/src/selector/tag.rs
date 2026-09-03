use super::prelude::*;

use css_parse::Result;

#[cfg(feature = "visitable")]
use crate::{
	PropertyKind,
	visit::{NodeId, QueryableNode},
};

#[node]
#[derive(
	Parse, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum Tag {
	Html(HtmlTag),
	HtmlNonStandard(HtmlNonStandardTag),
	HtmlNonConforming(HtmlNonConformingTag),
	Svg(SvgTag),
	Mathml(MathmlTag),
	CustomElement(CustomElementTag),
	Unknown(UnknownTag),
}

impl<'a> Peek<'a> for Tag {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);
}

impl css_parse::NodeWithMetadata<crate::CssMetadata> for Tag {
	fn metadata(&self) -> crate::CssMetadata {
		let mut metadata = crate::CssMetadata::default();

		match self {
			Tag::HtmlNonConforming(_) => {
				metadata.node_kinds |= crate::NodeKinds::Deprecated;
			}
			Tag::HtmlNonStandard(_) => {
				metadata.node_kinds |= crate::NodeKinds::Experimental;
			}
			Tag::CustomElement(_) => {
				metadata.node_kinds |= crate::NodeKinds::Custom;
			}
			Tag::Unknown(_) => {
				metadata.node_kinds |= crate::NodeKinds::Unknown;
			}
			_ => {}
		}

		metadata
	}
}

#[node]
#[derive(ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Custom)]
pub struct CustomElementTag(T![Ident]);

impl CustomElementTag {
	fn is_invalid(atom: CssAtomSet) -> bool {
		matches!(
			atom,
			CssAtomSet::AnnotationXml
				| CssAtomSet::ColorProfile
				| CssAtomSet::FontFace
				| CssAtomSet::FontFaceSrc
				| CssAtomSet::FontFaceUri
				| CssAtomSet::FontFaceFormat
				| CssAtomSet::FontFaceName
				| CssAtomSet::MissingGlyph
		)
	}
}

impl<'a> Peek<'a> for CustomElementTag {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let str = p.to_source_cursor(c).parse(p.alloc());
		if Self::is_invalid(p.to_atom(c)) {
			return false;
		}
		let mut chars = str.chars();
		if !matches!(chars.next(), Some('a'..='z')) {
			return false;
		}
		let mut has_dash = false;
		for char in chars {
			if char == '-' {
				has_dash = true;
				continue;
			}
			if !matches!(char,
				'.' |
				'_' |
				'0'..='9' |
				'a'..='z' |
				'\u{b7}' |
				'\u{c0}'..='\u{d6}' |
				'\u{d8}'..='\u{f6}' |
				'\u{f8}'..='\u{37d}' |
				'\u{37F}'..='\u{1fff}' |
				'\u{200c}'..='\u{200d}' |
				'\u{203f}'..='\u{2040}' |
				'\u{2070}'..='\u{218f}' |
				'\u{2c00}'..='\u{2fef}' |
				'\u{3001}'..='\u{d7ff}' |
				'\u{f900}'..='\u{fdcf}' |
				'\u{fdf0}'..='\u{fffd}' |
				'\u{10000}'..='\u{effff}'
			) {
				return false;
			}
		}
		has_dash
	}
}

impl<'a> Parse<'a> for CustomElementTag {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<Self>() {
			p.parse::<T![Ident]>().map(Self)
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
	}
}

/// <https://html.spec.whatwg.org/multipage/indices.html#elements-3>
#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum HtmlTag {
	#[atom(CssAtomSet::A)]
	A(T![Ident]),
	#[atom(CssAtomSet::Abbr)]
	Abbr(T![Ident]),
	#[atom(CssAtomSet::Address)]
	Address(T![Ident]),
	#[atom(CssAtomSet::Area)]
	Area(T![Ident]),
	#[atom(CssAtomSet::Article)]
	Article(T![Ident]),
	#[atom(CssAtomSet::Aside)]
	Aside(T![Ident]),
	#[atom(CssAtomSet::Audio)]
	Audio(T![Ident]),
	#[atom(CssAtomSet::B)]
	B(T![Ident]),
	#[atom(CssAtomSet::Base)]
	Base(T![Ident]),
	#[atom(CssAtomSet::Bdi)]
	Bdi(T![Ident]),
	#[atom(CssAtomSet::Bdo)]
	Bdo(T![Ident]),
	#[atom(CssAtomSet::Blockquote)]
	Blockquote(T![Ident]),
	#[atom(CssAtomSet::Body)]
	Body(T![Ident]),
	#[atom(CssAtomSet::Br)]
	Br(T![Ident]),
	#[atom(CssAtomSet::Button)]
	Button(T![Ident]),
	#[atom(CssAtomSet::Canvas)]
	Canvas(T![Ident]),
	#[atom(CssAtomSet::Caption)]
	Caption(T![Ident]),
	#[atom(CssAtomSet::Cite)]
	Cite(T![Ident]),
	#[atom(CssAtomSet::Code)]
	Code(T![Ident]),
	#[atom(CssAtomSet::Col)]
	Col(T![Ident]),
	#[atom(CssAtomSet::Colgroup)]
	Colgroup(T![Ident]),
	#[atom(CssAtomSet::Data)]
	Data(T![Ident]),
	#[atom(CssAtomSet::Datalist)]
	Datalist(T![Ident]),
	#[atom(CssAtomSet::Dd)]
	Dd(T![Ident]),
	#[atom(CssAtomSet::Del)]
	Del(T![Ident]),
	#[atom(CssAtomSet::Details)]
	Details(T![Ident]),
	#[atom(CssAtomSet::Dfn)]
	Dfn(T![Ident]),
	#[atom(CssAtomSet::Dialog)]
	Dialog(T![Ident]),
	#[atom(CssAtomSet::Div)]
	Div(T![Ident]),
	#[atom(CssAtomSet::Dl)]
	Dl(T![Ident]),
	#[atom(CssAtomSet::Dt)]
	Dt(T![Ident]),
	#[atom(CssAtomSet::Em)]
	Em(T![Ident]),
	#[atom(CssAtomSet::Embed)]
	Embed(T![Ident]),
	#[atom(CssAtomSet::Fieldset)]
	Fieldset(T![Ident]),
	#[atom(CssAtomSet::Figcaption)]
	Figcaption(T![Ident]),
	#[atom(CssAtomSet::Figure)]
	Figure(T![Ident]),
	#[atom(CssAtomSet::Footer)]
	Footer(T![Ident]),
	#[atom(CssAtomSet::Form)]
	Form(T![Ident]),
	#[atom(CssAtomSet::H1)]
	H1(T![Ident]),
	#[atom(CssAtomSet::H2)]
	H2(T![Ident]),
	#[atom(CssAtomSet::H3)]
	H3(T![Ident]),
	#[atom(CssAtomSet::H4)]
	H4(T![Ident]),
	#[atom(CssAtomSet::H5)]
	H5(T![Ident]),
	#[atom(CssAtomSet::H6)]
	H6(T![Ident]),
	#[atom(CssAtomSet::Head)]
	Head(T![Ident]),
	#[atom(CssAtomSet::Header)]
	Header(T![Ident]),
	#[atom(CssAtomSet::Hgroup)]
	Hgroup(T![Ident]),
	#[atom(CssAtomSet::Hr)]
	Hr(T![Ident]),
	#[atom(CssAtomSet::Html)]
	Html(T![Ident]),
	#[atom(CssAtomSet::I)]
	I(T![Ident]),
	#[atom(CssAtomSet::Iframe)]
	Iframe(T![Ident]),
	#[atom(CssAtomSet::Img)]
	Img(T![Ident]),
	#[atom(CssAtomSet::Input)]
	Input(T![Ident]),
	#[atom(CssAtomSet::Ins)]
	Ins(T![Ident]),
	#[atom(CssAtomSet::Kbd)]
	Kbd(T![Ident]),
	#[atom(CssAtomSet::Label)]
	Label(T![Ident]),
	#[atom(CssAtomSet::Legend)]
	Legend(T![Ident]),
	#[atom(CssAtomSet::Li)]
	Li(T![Ident]),
	#[atom(CssAtomSet::Link)]
	Link(T![Ident]),
	#[atom(CssAtomSet::Main)]
	Main(T![Ident]),
	#[atom(CssAtomSet::Map)]
	Map(T![Ident]),
	#[atom(CssAtomSet::Mark)]
	Mark(T![Ident]),
	#[atom(CssAtomSet::Menu)]
	Menu(T![Ident]),
	#[atom(CssAtomSet::Meta)]
	Meta(T![Ident]),
	#[atom(CssAtomSet::Meter)]
	Meter(T![Ident]),
	#[atom(CssAtomSet::Nav)]
	Nav(T![Ident]),
	#[atom(CssAtomSet::Noscript)]
	Noscript(T![Ident]),
	#[atom(CssAtomSet::Object)]
	Object(T![Ident]),
	#[atom(CssAtomSet::Ol)]
	Ol(T![Ident]),
	#[atom(CssAtomSet::Optgroup)]
	Optgroup(T![Ident]),
	#[atom(CssAtomSet::Option)]
	Option(T![Ident]),
	#[atom(CssAtomSet::Output)]
	Output(T![Ident]),
	#[atom(CssAtomSet::P)]
	P(T![Ident]),
	#[atom(CssAtomSet::Picture)]
	Picture(T![Ident]),
	#[atom(CssAtomSet::Pre)]
	Pre(T![Ident]),
	#[atom(CssAtomSet::Progress)]
	Progress(T![Ident]),
	#[atom(CssAtomSet::Q)]
	Q(T![Ident]),
	#[atom(CssAtomSet::Rp)]
	Rp(T![Ident]),
	#[atom(CssAtomSet::Rt)]
	Rt(T![Ident]),
	#[atom(CssAtomSet::Ruby)]
	Ruby(T![Ident]),
	#[atom(CssAtomSet::S)]
	S(T![Ident]),
	#[atom(CssAtomSet::Samp)]
	Samp(T![Ident]),
	#[atom(CssAtomSet::Script)]
	Script(T![Ident]),
	#[atom(CssAtomSet::Search)]
	Search(T![Ident]),
	#[atom(CssAtomSet::Section)]
	Section(T![Ident]),
	#[atom(CssAtomSet::Select)]
	Select(T![Ident]),
	#[atom(CssAtomSet::Slot)]
	Slot(T![Ident]),
	#[atom(CssAtomSet::Small)]
	Small(T![Ident]),
	#[atom(CssAtomSet::Source)]
	Source(T![Ident]),
	#[atom(CssAtomSet::Span)]
	Span(T![Ident]),
	#[atom(CssAtomSet::Strong)]
	Strong(T![Ident]),
	#[atom(CssAtomSet::Style)]
	Style(T![Ident]),
	#[atom(CssAtomSet::Sub)]
	Sub(T![Ident]),
	#[atom(CssAtomSet::Summary)]
	Summary(T![Ident]),
	#[atom(CssAtomSet::Sup)]
	Sup(T![Ident]),
	#[atom(CssAtomSet::Table)]
	Table(T![Ident]),
	#[atom(CssAtomSet::Tbody)]
	Tbody(T![Ident]),
	#[atom(CssAtomSet::Td)]
	Td(T![Ident]),
	#[atom(CssAtomSet::Template)]
	Template(T![Ident]),
	#[atom(CssAtomSet::Textarea)]
	Textarea(T![Ident]),
	#[atom(CssAtomSet::Tfoot)]
	Tfoot(T![Ident]),
	#[atom(CssAtomSet::Th)]
	Th(T![Ident]),
	#[atom(CssAtomSet::Thead)]
	Thead(T![Ident]),
	#[atom(CssAtomSet::Time)]
	Time(T![Ident]),
	#[atom(CssAtomSet::Title)]
	Title(T![Ident]),
	#[atom(CssAtomSet::Tr)]
	Tr(T![Ident]),
	#[atom(CssAtomSet::Track)]
	Track(T![Ident]),
	#[atom(CssAtomSet::U)]
	U(T![Ident]),
	#[atom(CssAtomSet::Ul)]
	Ul(T![Ident]),
	#[atom(CssAtomSet::Var)]
	Var(T![Ident]),
	#[atom(CssAtomSet::Video)]
	Video(T![Ident]),
	#[atom(CssAtomSet::Wbr)]
	Wbr(T![Ident]),
}

/// <https://html.spec.whatwg.org/multipage/obsolete.html#non-conforming-features>
#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Deprecated)]
pub enum HtmlNonConformingTag {
	#[atom(CssAtomSet::Acronym)]
	Acronym(T![Ident]),
	#[atom(CssAtomSet::Applet)]
	Applet(T![Ident]),
	#[atom(CssAtomSet::Basefont)]
	Basefont(T![Ident]),
	#[atom(CssAtomSet::Bgsound)]
	Bgsound(T![Ident]),
	#[atom(CssAtomSet::Big)]
	Big(T![Ident]),
	#[atom(CssAtomSet::Blink)]
	Blink(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::Dir)]
	Dir(T![Ident]),
	#[atom(CssAtomSet::Font)]
	Font(T![Ident]),
	#[atom(CssAtomSet::Frame)]
	Frame(T![Ident]),
	#[atom(CssAtomSet::Frameset)]
	Frameset(T![Ident]),
	#[atom(CssAtomSet::Isindex)]
	Isindex(T![Ident]),
	#[atom(CssAtomSet::Keygen)]
	Keygen(T![Ident]),
	#[atom(CssAtomSet::Listing)]
	Listing(T![Ident]),
	#[atom(CssAtomSet::Marquee)]
	Marquee(T![Ident]),
	#[atom(CssAtomSet::Menuitem)]
	Menuitem(T![Ident]),
	#[atom(CssAtomSet::Multicol)]
	Multicol(T![Ident]),
	#[atom(CssAtomSet::Nextid)]
	Nextid(T![Ident]),
	#[atom(CssAtomSet::Nobr)]
	Nobr(T![Ident]),
	#[atom(CssAtomSet::Noembed)]
	Noembed(T![Ident]),
	#[atom(CssAtomSet::Noframes)]
	Noframes(T![Ident]),
	#[atom(CssAtomSet::Param)]
	Param(T![Ident]),
	#[atom(CssAtomSet::Plaintext)]
	Plaintext(T![Ident]),
	#[atom(CssAtomSet::Rb)]
	Rb(T![Ident]),
	#[atom(CssAtomSet::Rtc)]
	Rtc(T![Ident]),
	#[atom(CssAtomSet::Spacer)]
	Spacer(T![Ident]),
	#[atom(CssAtomSet::Strike)]
	Strike(T![Ident]),
	#[atom(CssAtomSet::Tt)]
	Tt(T![Ident]),
	#[atom(CssAtomSet::Xmp)]
	Xmp(T![Ident]),
}

#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Experimental)]
pub enum HtmlNonStandardTag {
	// https://wicg.github.io/fenced-frame/#the-fencedframe-element
	#[atom(CssAtomSet::Fencedframe)]
	Fencedframe(T![Ident]),
	// https://wicg.github.io/portals/#the-portal-element
	#[atom(CssAtomSet::Portal)]
	Portal(T![Ident]),
	// https://wicg.github.io/PEPC/permission-element.html#the-permission-element
	#[atom(CssAtomSet::Permission)]
	Permission(T![Ident]),
	// https://open-ui.org/components/customizableselect/
	#[atom(CssAtomSet::Selectedcontent)]
	Selectedcontent(T![Ident]),
}

/// <https://svgwg.org/svg2-draft/eltindex.html>
#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SvgTag {
	#[atom(CssAtomSet::A)]
	A(T![Ident]),
	#[atom(CssAtomSet::Animate)]
	Animate(T![Ident]),
	#[atom(CssAtomSet::Animatemotion)]
	Animatemotion(T![Ident]),
	#[atom(CssAtomSet::Animatetransform)]
	Animatetransform(T![Ident]),
	#[atom(CssAtomSet::Circle)]
	Circle(T![Ident]),
	#[atom(CssAtomSet::Clippath)]
	Clippath(T![Ident]),
	#[atom(CssAtomSet::Defs)]
	Defs(T![Ident]),
	#[atom(CssAtomSet::Desc)]
	Desc(T![Ident]),
	#[atom(CssAtomSet::Discard)]
	Discard(T![Ident]),
	#[atom(CssAtomSet::Ellipse)]
	Ellipse(T![Ident]),
	#[atom(CssAtomSet::Feblend)]
	Feblend(T![Ident]),
	#[atom(CssAtomSet::Fecolormatrix)]
	Fecolormatrix(T![Ident]),
	#[atom(CssAtomSet::Fecomponenttransfer)]
	Fecomponenttransfer(T![Ident]),
	#[atom(CssAtomSet::Fecomposite)]
	Fecomposite(T![Ident]),
	#[atom(CssAtomSet::Feconvolvematrix)]
	Feconvolvematrix(T![Ident]),
	#[atom(CssAtomSet::Fediffuselighting)]
	Fediffuselighting(T![Ident]),
	#[atom(CssAtomSet::Fedisplacementmap)]
	Fedisplacementmap(T![Ident]),
	#[atom(CssAtomSet::Fedistantlight)]
	Fedistantlight(T![Ident]),
	#[atom(CssAtomSet::Fedropshadow)]
	Fedropshadow(T![Ident]),
	#[atom(CssAtomSet::Feflood)]
	Feflood(T![Ident]),
	#[atom(CssAtomSet::Fefunca)]
	Fefunca(T![Ident]),
	#[atom(CssAtomSet::Fefuncb)]
	Fefuncb(T![Ident]),
	#[atom(CssAtomSet::Fefuncg)]
	Fefuncg(T![Ident]),
	#[atom(CssAtomSet::Fefuncr)]
	Fefuncr(T![Ident]),
	#[atom(CssAtomSet::Fegaussianblur)]
	Fegaussianblur(T![Ident]),
	#[atom(CssAtomSet::Feimage)]
	Feimage(T![Ident]),
	#[atom(CssAtomSet::Femerge)]
	Femerge(T![Ident]),
	#[atom(CssAtomSet::Femergenode)]
	Femergenode(T![Ident]),
	#[atom(CssAtomSet::Femorphology)]
	Femorphology(T![Ident]),
	#[atom(CssAtomSet::Feoffset)]
	Feoffset(T![Ident]),
	#[atom(CssAtomSet::Fepointlight)]
	Fepointlight(T![Ident]),
	#[atom(CssAtomSet::Fespecularlighting)]
	Fespecularlighting(T![Ident]),
	#[atom(CssAtomSet::Fespotlight)]
	Fespotlight(T![Ident]),
	#[atom(CssAtomSet::Fetile)]
	Fetile(T![Ident]),
	#[atom(CssAtomSet::Feturbulence)]
	Feturbulence(T![Ident]),
	#[atom(CssAtomSet::Filter)]
	Filter(T![Ident]),
	#[atom(CssAtomSet::Foreignobject)]
	Foreignobject(T![Ident]),
	#[atom(CssAtomSet::G)]
	G(T![Ident]),
	#[atom(CssAtomSet::Image)]
	Image(T![Ident]),
	#[atom(CssAtomSet::Line)]
	Line(T![Ident]),
	#[atom(CssAtomSet::Lineargradient)]
	Lineargradient(T![Ident]),
	#[atom(CssAtomSet::Marker)]
	Marker(T![Ident]),
	#[atom(CssAtomSet::Mask)]
	Mask(T![Ident]),
	#[atom(CssAtomSet::Metadata)]
	Metadata(T![Ident]),
	#[atom(CssAtomSet::Mpath)]
	Mpath(T![Ident]),
	#[atom(CssAtomSet::Path)]
	Path(T![Ident]),
	#[atom(CssAtomSet::Pattern)]
	Pattern(T![Ident]),
	#[atom(CssAtomSet::Polygon)]
	Polygon(T![Ident]),
	#[atom(CssAtomSet::Polyline)]
	Polyline(T![Ident]),
	#[atom(CssAtomSet::Radialgradient)]
	Radialgradient(T![Ident]),
	#[atom(CssAtomSet::Rect)]
	Rect(T![Ident]),
	#[atom(CssAtomSet::Script)]
	Script(T![Ident]),
	#[atom(CssAtomSet::Set)]
	Set(T![Ident]),
	#[atom(CssAtomSet::Stop)]
	Stop(T![Ident]),
	#[atom(CssAtomSet::Style)]
	Style(T![Ident]),
	#[atom(CssAtomSet::Svg)]
	Svg(T![Ident]),
	#[atom(CssAtomSet::Switch)]
	Switch(T![Ident]),
	#[atom(CssAtomSet::Symbol)]
	Symbol(T![Ident]),
	#[atom(CssAtomSet::Text)]
	Text(T![Ident]),
	#[atom(CssAtomSet::Textpath)]
	Textpath(T![Ident]),
	#[atom(CssAtomSet::Title)]
	Title(T![Ident]),
	#[atom(CssAtomSet::Tspan)]
	Tspan(T![Ident]),
	#[atom(CssAtomSet::Use)]
	Use(T![Ident]),
	#[atom(CssAtomSet::View)]
	View(T![Ident]),
}

/// <https://w3c.github.io/mathml-core/#mathml-elements>
#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MathmlTag {
	#[atom(CssAtomSet::Annotation)]
	Annotation(T![Ident]),
	#[atom(CssAtomSet::AnnotationXml)]
	AnnotationXml(T![Ident]),
	#[atom(CssAtomSet::Maction)]
	Maction(T![Ident]),
	#[atom(CssAtomSet::Maligngroup)]
	Maligngroup(T![Ident]),
	#[atom(CssAtomSet::Malignmark)]
	Malignmark(T![Ident]),
	#[atom(CssAtomSet::Math)]
	Math(T![Ident]),
	#[atom(CssAtomSet::Menclose)]
	Menclose(T![Ident]),
	#[atom(CssAtomSet::Merror)]
	Merror(T![Ident]),
	#[atom(CssAtomSet::Mfenced)]
	Mfenced(T![Ident]),
	#[atom(CssAtomSet::Mfrac)]
	Mfrac(T![Ident]),
	#[atom(CssAtomSet::Mglyph)]
	Mglyph(T![Ident]),
	#[atom(CssAtomSet::Mi)]
	Mi(T![Ident]),
	#[atom(CssAtomSet::Mlabeledtr)]
	Mlabeledtr(T![Ident]),
	#[atom(CssAtomSet::Mlongdiv)]
	Mlongdiv(T![Ident]),
	#[atom(CssAtomSet::Mmultiscripts)]
	Mmultiscripts(T![Ident]),
	#[atom(CssAtomSet::Mn)]
	Mn(T![Ident]),
	#[atom(CssAtomSet::Mo)]
	Mo(T![Ident]),
	#[atom(CssAtomSet::Mover)]
	Mover(T![Ident]),
	#[atom(CssAtomSet::Mpadded)]
	Mpadded(T![Ident]),
	#[atom(CssAtomSet::Mphantom)]
	Mphantom(T![Ident]),
	#[atom(CssAtomSet::Mprescripts)]
	Mprescripts(T![Ident]),
	#[atom(CssAtomSet::Mroot)]
	Mroot(T![Ident]),
	#[atom(CssAtomSet::Mrow)]
	Mrow(T![Ident]),
	#[atom(CssAtomSet::Ms)]
	Ms(T![Ident]),
	#[atom(CssAtomSet::Mscarries)]
	Mscarries(T![Ident]),
	#[atom(CssAtomSet::Mscarry)]
	Mscarry(T![Ident]),
	#[atom(CssAtomSet::Msgroup)]
	Msgroup(T![Ident]),
	#[atom(CssAtomSet::Msline)]
	Msline(T![Ident]),
	#[atom(CssAtomSet::Mspace)]
	Mspace(T![Ident]),
	#[atom(CssAtomSet::Msqrt)]
	Msqrt(T![Ident]),
	#[atom(CssAtomSet::Msrow)]
	Msrow(T![Ident]),
	#[atom(CssAtomSet::Mstack)]
	Mstack(T![Ident]),
	#[atom(CssAtomSet::Mstyle)]
	Mstyle(T![Ident]),
	#[atom(CssAtomSet::Msub)]
	Msub(T![Ident]),
	#[atom(CssAtomSet::Msubsup)]
	Msubsup(T![Ident]),
	#[atom(CssAtomSet::Msup)]
	Msup(T![Ident]),
	#[atom(CssAtomSet::Mtable)]
	Mtable(T![Ident]),
	#[atom(CssAtomSet::Mtd)]
	Mtd(T![Ident]),
	#[atom(CssAtomSet::Mtext)]
	Mtext(T![Ident]),
	#[atom(CssAtomSet::Mtr)]
	Mtr(T![Ident]),
	#[atom(CssAtomSet::Munder)]
	Munder(T![Ident]),
	#[atom(CssAtomSet::Munderover)]
	Munderover(T![Ident]),
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Semantics)]
	Semantics(T![Ident]),
}

#[node]
#[derive(
	ToCursors, Parse, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self), queryable(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Unknown, property_kinds = Name)]
pub struct UnknownTag(T![Ident]);

#[cfg(feature = "visitable")]
impl QueryableNode for UnknownTag {
	const NODE_ID: NodeId = NodeId::UnknownTag;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.0.into()),
			_ => None,
		}
	}
}

impl<'a> Peek<'a> for UnknownTag {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Tag, "div");
	}
}
