include!(concat!(env!("OUT_DIR"), "/css_apply_visit_methods.rs"));

use crate::*;
use bumpalo::collections::Vec;
use css_ast::{Declaration, Nth};
use css_lexer::ToSpan;
use css_parse::{
	AtRule, BadDeclaration, Block, CommaSeparated, ComponentValues, DeclarationList,
	DeclarationValue as DeclarationValueTrait, NoBlockAllowed, NoPreludeAllowed, Parse, Peek, QualifiedRule, RuleList,
	ToCursors, token_macros,
};

macro_rules! visit_mut_trait {
	( $(
		$name: ident$(<$life:lifetime>)?($obj: ty),
	)+ ) => {
		pub trait VisitMut: Sized + Default {
			fn visit_declaration<'a, T: DeclarationValueTrait<'a>>(&mut self, _: &mut Declaration<'a, T>) {}
			fn visit_bad_declaration<'a>(&mut self, _: &mut BadDeclaration<'a>) {}
			fn visit_tag(&mut self, _: &mut Tag) {}
			fn visit_nth(&mut self, _: &mut Nth) {}
			fn visit_attribute<'a>(&mut self, _: &Attribute) {}
			fn visit_combinator<'a>(&mut self, _: &Combinator) {}
			$(
				fn $name$(<$life>)?(&mut self, _: &mut $obj) {}
			)+
		}
	}
}
apply_visit_methods!(visit_mut_trait);

macro_rules! visit_trait {
	( $(
		$name: ident$(<$life:lifetime>)?($obj: ty),
	)+ ) => {
		pub trait Visit: Sized + Default {
			fn visit_declaration<'a, T: DeclarationValueTrait<'a>>(&self, _: &Declaration<'a, T>) {}
			fn visit_bad_declaration<'a>(&self, _: &BadDeclaration<'a>) {}
			fn visit_tag(&mut self, _: &Tag) {}
			fn visit_nth(&mut self, _: &Nth) {}
			fn visit_attribute<'a>(&mut self, _: &Attribute) {}
			fn visit_combinator<'a>(&mut self, _: &Combinator) {}
			$(
				fn $name$(<$life>)?(&mut self, _rule: &$obj) {}
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

impl<'a, T> VisitableMut for CommaSeparated<'a, T>
where
	T: VisitableMut + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for (node, _) in self {
			node.accept_mut(v)
		}
	}
}

impl<'a, T> Visitable for CommaSeparated<'a, T>
where
	T: Visitable + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		for (node, _) in self {
			node.accept(v)
		}
	}
}

impl<'a, T> VisitableMut for Declaration<'a, T>
where
	T: VisitableMut + DeclarationValueTrait<'a>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_declaration::<T>(self);
		self.value.accept_mut(v)
	}
}

impl<'a, T> Visitable for Declaration<'a, T>
where
	T: Visitable + DeclarationValueTrait<'a>,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		v.visit_declaration::<T>(self);
		self.value.accept(v)
	}
}

impl<'a, T> VisitableMut for DeclarationList<'a, T>
where
	T: VisitableMut + DeclarationValueTrait<'a>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for declaration in &mut self.declarations {
			declaration.accept_mut(v);
		}
	}
}

impl<'a, T> Visitable for DeclarationList<'a, T>
where
	T: Visitable + DeclarationValueTrait<'a>,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		for declaration in &self.declarations {
			declaration.accept(v);
		}
	}
}

impl<'a, AT, P, B> VisitableMut for AtRule<'a, AT, P, B>
where
	AT: Peek<'a> + Parse<'a> + Into<token_macros::AtKeyword>,
	P: VisitableMut + Parse<'a> + ToCursors + ToSpan,
	B: VisitableMut + Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		self.prelude.accept_mut(v);
		self.block.accept_mut(v);
	}
}

impl<'a, AT, P, B> Visitable for AtRule<'a, AT, P, B>
where
	AT: Peek<'a> + Parse<'a> + Into<token_macros::AtKeyword>,
	P: Visitable + Parse<'a> + ToCursors + ToSpan,
	B: Visitable + Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		self.prelude.accept(v);
		self.block.accept(v);
	}
}

impl<'a, T> VisitableMut for RuleList<'a, T>
where
	T: VisitableMut + Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		self.rules.accept_mut(v);
	}
}

impl<'a, T> Visitable for RuleList<'a, T>
where
	T: Visitable + Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		self.rules.accept(v);
	}
}

impl<'a, P, D, R> VisitableMut for QualifiedRule<'a, P, D, R>
where
	P: VisitableMut + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: VisitableMut + DeclarationValueTrait<'a>,
	R: VisitableMut + Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R>: Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		self.prelude.accept_mut(v);
		self.block.accept_mut(v);
	}
}

impl<'a, P, D, R> Visitable for QualifiedRule<'a, P, D, R>
where
	P: Visitable + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: Visitable + DeclarationValueTrait<'a>,
	R: Visitable + Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R>: Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		self.prelude.accept(v);
		self.block.accept(v);
	}
}

impl<'a, D, R> VisitableMut for Block<'a, D, R>
where
	D: VisitableMut + DeclarationValueTrait<'a>,
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

impl<'a, D, R> Visitable for Block<'a, D, R>
where
	D: Visitable + DeclarationValueTrait<'a>,
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

impl VisitableMut for NoPreludeAllowed {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for NoPreludeAllowed {
	fn accept<V: Visit>(&self, _: &mut V) {}
}

impl VisitableMut for NoBlockAllowed {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for NoBlockAllowed {
	fn accept<V: Visit>(&self, _: &mut V) {}
}

// Reusing some of the css_ast types, so need to apply _this_ Visitable/Mut to them:
impl<'a> VisitableMut for Nth {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_nth(self);
	}
}

impl<'a> Visitable for Nth {
	fn accept<V: Visit>(&self, v: &mut V) {
		v.visit_nth(self);
	}
}

impl<'a> VisitableMut for Attribute {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_attribute(self);
	}
}

impl<'a> Visitable for Attribute {
	fn accept<V: Visit>(&self, v: &mut V) {
		v.visit_attribute(self);
	}
}

impl<'a> VisitableMut for Combinator {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_combinator(self);
	}
}

impl<'a> Visitable for Combinator {
	fn accept<V: Visit>(&self, v: &mut V) {
		v.visit_combinator(self);
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
