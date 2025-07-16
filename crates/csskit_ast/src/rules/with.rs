use crate::NodeRule;
use css_parse::{AtRule, ComponentValues, RuleList, atkeyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

atkeyword_set!(struct AtWithKeyword "with");

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct WithRule<'a>(AtRule<'a, AtWithKeyword, ComponentValues<'a>, WithRuleBlock<'a>>);

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct WithRuleBlock<'a>(RuleList<'a, NodeRule<'a>>);
