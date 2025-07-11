include!(concat!(env!("OUT_DIR"), "/css_node_kind.rs"));
include!(concat!(env!("OUT_DIR"), "/css_apply_visit_methods.rs"));

use bumpalo::collections::Vec;
use css_parse::{CommaSeparated, syntax::BadDeclaration};

use crate::*;

macro_rules! visit_mut_trait {
	( $(
		$name: ident$(<$life:lifetime>)?($obj: ty),
	)+ ) => {
		pub trait VisitMut: Sized + Default {
			fn visit_bad_declaration<'a>(&mut self, _rule: &mut BadDeclaration<'a>) {}
			$(
				fn $name$(<$life>)?(&mut self, _rule: &mut $obj) {}
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
			fn visit_bad_declaration<'a>(&mut self, _rule: &BadDeclaration<'a>) {}
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
