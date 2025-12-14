include!(concat!(env!("OUT_DIR"), "/css_node_kind.rs"));
include!(concat!(env!("OUT_DIR"), "/css_apply_visit_methods.rs"));
pub use apply_visit_methods;

use bumpalo::collections::Vec;
use css_parse::{
	Block, CommaSeparated, ComponentValues, Declaration, DeclarationGroup, DeclarationList, DeclarationOrBad,
	DeclarationValue, NoBlockAllowed, NodeMetadata, NodeWithMetadata, QualifiedRule, RuleList, syntax::BadDeclaration,
	token_macros,
};

use crate::*;

macro_rules! visit_mut_trait {
	( $(
		$name: ident$(<$($gen:tt),+>)?($obj: ty),
	)+ ) => {
		pub trait VisitMut: Sized {
			fn visit_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(&mut self, _rule: &mut Declaration<'a, T, CssMetadata>) {}
			fn visit_bad_declaration<'a>(&mut self, _rule: &mut BadDeclaration<'a>) {}
			fn visit_string(&mut self, _str: &mut token_macros::String) {}
			$(
				fn $name$(<$($gen),+>)?(&mut self, _rule: &mut $obj) {}
			)+
		}
	}
}
apply_visit_methods!(visit_mut_trait);

macro_rules! visit_trait {
	( $(
		$name: ident$(<$($gen:tt),+>)?($obj: ty),
	)+ ) => {
		pub trait Visit: Sized {
			fn visit_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(&mut self, _rule: &Declaration<'a, T, CssMetadata>) {}
			fn visit_bad_declaration<'a>(&mut self, _rule: &BadDeclaration<'a>) {}
			fn visit_string(&mut self, _str: &token_macros::String) {}
			$(
				fn $name$(<$($gen),+>)?(&mut self, _rule: &$obj) {}
			)+
		}
	}
}
apply_visit_methods!(visit_trait);

pub trait VisitableMut {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V);
}

pub trait Visitable {
	fn accept<V: Visit>(&self, v: &mut V);
}

impl<T> VisitableMut for Option<T>
where
	T: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		if let Some(node) = self {
			node.accept_mut(v)
		}
	}
}

impl Visitable for token_macros::Comma {
	fn accept<V: Visit>(&self, _: &mut V) {}
}

impl VisitableMut for token_macros::Comma {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::delim::Slash {
	fn accept<V: Visit>(&self, _: &mut V) {}
}

impl VisitableMut for token_macros::delim::Slash {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::Number {
	fn accept<V: Visit>(&self, _: &mut V) {}
}

impl VisitableMut for token_macros::Number {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::String {
	fn accept<V: Visit>(&self, v: &mut V) {
		v.visit_string(self);
	}
}

impl VisitableMut for token_macros::String {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_string(self);
	}
}

impl<T> Visitable for Option<T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		if let Some(node) = self {
			node.accept(v)
		}
	}
}

impl<'a, T, const MIN: usize> VisitableMut for CommaSeparated<'a, T, MIN>
where
	T: VisitableMut + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for (node, _) in self {
			node.accept_mut(v)
		}
	}
}

impl<'a, T, const MIN: usize> Visitable for CommaSeparated<'a, T, MIN>
where
	T: Visitable + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		for (node, _) in self {
			node.accept(v)
		}
	}
}

impl<'a, T> VisitableMut for Declaration<'a, T, CssMetadata>
where
	T: VisitableMut + DeclarationValue<'a, CssMetadata>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_declaration(self);
		self.value.accept_mut(v)
	}
}

impl<'a, T> Visitable for Declaration<'a, T, CssMetadata>
where
	T: Visitable + DeclarationValue<'a, CssMetadata>,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		v.visit_declaration::<T>(self);
		self.value.accept(v)
	}
}

impl<'a, T> VisitableMut for DeclarationList<'a, T, CssMetadata>
where
	T: VisitableMut + DeclarationValue<'a, CssMetadata>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for declaration in &mut self.declarations {
			declaration.accept_mut(v);
		}
	}
}

impl<'a, T> Visitable for DeclarationList<'a, T, CssMetadata>
where
	T: Visitable + DeclarationValue<'a, CssMetadata>,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		for declaration in &self.declarations {
			declaration.accept(v);
		}
	}
}

impl<'a, T, M> VisitableMut for RuleList<'a, T, M>
where
	T: VisitableMut + Parse<'a> + ToCursors + ToSpan + NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		self.rules.accept_mut(v);
	}
}

impl<'a, T, M> Visitable for RuleList<'a, T, M>
where
	T: Visitable + Parse<'a> + ToCursors + ToSpan + NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		self.rules.accept(v);
	}
}

impl<'a, P, D, R> VisitableMut for QualifiedRule<'a, P, D, R, CssMetadata>
where
	P: VisitableMut + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: VisitableMut + DeclarationValue<'a, CssMetadata>,
	R: VisitableMut + Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R, CssMetadata>: Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		self.prelude.accept_mut(v);
		self.block.accept_mut(v);
	}
}

impl<'a, P, D, R> Visitable for QualifiedRule<'a, P, D, R, CssMetadata>
where
	P: Visitable + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: Visitable + DeclarationValue<'a, CssMetadata>,
	R: Visitable + Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R, CssMetadata>: Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		self.prelude.accept(v);
		self.block.accept(v);
	}
}

impl<'a, D, R> VisitableMut for Block<'a, D, R, CssMetadata>
where
	D: VisitableMut + DeclarationValue<'a, CssMetadata>,
	R: VisitableMut + Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for declaration in &mut self.declarations {
			declaration.accept_mut(v);
		}
		for rule in &mut self.rules {
			rule.accept_mut(v);
		}
	}
}

impl<'a, D, R> Visitable for Block<'a, D, R, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata>,
	R: Visitable + Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		for declaration in &self.declarations {
			declaration.accept(v);
		}
		for rule in &self.rules {
			rule.accept(v);
		}
	}
}

impl<'a, T> VisitableMut for Vec<'a, T>
where
	T: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for node in self {
			node.accept_mut(v);
		}
	}
}

impl<'a, T> Visitable for Vec<'a, T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		for node in self {
			node.accept(v)
		}
	}
}

impl<'a> VisitableMut for BadDeclaration<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_bad_declaration(self);
	}
}

impl<'a> Visitable for BadDeclaration<'a> {
	fn accept<V: Visit>(&self, v: &mut V) {
		v.visit_bad_declaration(self);
	}
}

impl<'a> VisitableMut for ComponentValues<'a> {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl<'a> Visitable for ComponentValues<'a> {
	fn accept<V: Visit>(&self, _: &mut V) {}
}

impl<D, M> VisitableMut for NoBlockAllowed<D, M> {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl<D, M> Visitable for NoBlockAllowed<D, M> {
	fn accept<V: Visit>(&self, _: &mut V) {}
}

impl<'a, D> VisitableMut for DeclarationGroup<'a, D, CssMetadata>
where
	D: VisitableMut + DeclarationValue<'a, CssMetadata>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for declaration in &mut self.declarations {
			declaration.accept_mut(v)
		}
	}
}

impl<'a, D> Visitable for DeclarationGroup<'a, D, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata>,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		for declaration in &self.declarations {
			declaration.accept(v)
		}
	}
}

impl<'a, D> VisitableMut for DeclarationOrBad<'a, D, CssMetadata>
where
	D: VisitableMut + DeclarationValue<'a, CssMetadata>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::Declaration(d) => d.accept_mut(v),
			Self::Bad(b) => b.accept_mut(v),
		}
	}
}

impl<'a, D> Visitable for DeclarationOrBad<'a, D, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata>,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		match self {
			Self::Declaration(d) => d.accept(v),
			Self::Bad(b) => b.accept(v),
		}
	}
}

macro_rules! impl_tuple_mut {
    ($($T:ident),*) => {
				impl<$($T),*> VisitableMut for ($($T),*)
        where
            $($T: VisitableMut,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused)]
						fn accept_mut<VI: VisitMut>(&mut self, v: &mut VI) {
                let ($($T),*) = self;
                $($T.accept_mut(v);)*
            }
        }
    };
}

impl_tuple_mut!(T, U);
impl_tuple_mut!(T, U, V);
impl_tuple_mut!(T, U, V, W);

macro_rules! impl_tuple {
    ($($T:ident),*) => {
				impl<$($T),*> Visitable for ($($T),*)
        where
            $($T: Visitable,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused)]
						fn accept<VI: Visit>(&self, v: &mut VI) {
                let ($($T),*) = self;
                $($T.accept(v);)*
            }
        }
    };
}
impl_tuple!(T, U);
impl_tuple!(T, U, V);
impl_tuple!(T, U, V, W);
