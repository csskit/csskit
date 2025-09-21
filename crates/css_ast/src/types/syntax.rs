#![allow(unused)]
use super::prelude::*;

// https://drafts.csswg.org/css-values-5/#css-syntax
// <syntax> = '*' | <syntax-component> [ <syntax-combinator> <syntax-component> ]* | <syntax-string>
// <syntax-component> = <syntax-single-component> <syntax-multiplier>?
//                    | '<' transform-list '>'
// <syntax-single-component> = '<' <syntax-type-name> '>' | <ident>
// <syntax-type-name> = angle | color | custom-ident | image | integer
//                    | length | length-percentage | number
//                    | percentage | resolution | string | time
//                    | url | transform-function
// <syntax-combinator> = '|'
// <syntax-multiplier> = [ '#' | '+' ]
//
// <syntax-string> = <string>
pub type Syntax = crate::Todo;
