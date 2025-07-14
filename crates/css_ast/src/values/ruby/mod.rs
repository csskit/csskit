#![allow(warnings)]
//! CSS Ruby Annotation Layout Module Level 1
//! https://drafts.csswg.org/css-ruby-1/

mod impls;
use impls::*;

// /// Represents the style value for `ruby-position` as defined in [css-ruby-1](https://drafts.csswg.org/css-ruby-1/#ruby-position).
// ///
// /// The ruby-position CSS property sets the position of a ruby annotation in relation to its base text. Annotations can display over, under, or interleaved with the base text.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ alternate || [ over | under ] ] | inter-character
// /// ```
// ///
// // https://drafts.csswg.org/css-ruby-1/#ruby-position
// #[value(" [ alternate || [ over | under ] ] | inter-character ")]
// #[initial("alternate")]
// #[applies_to("ruby annotation containers")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(newly)]
// #[versions(chrome:84,chrome_android:84,edge:84,firefox:38,firefox_android:38,safari:18.2,safari_ios:18.2)]
// pub enum RubyPositionStyleValue {}

/// Represents the style value for `ruby-merge` as defined in [css-ruby-1](https://drafts.csswg.org/css-ruby-1/#ruby-merge).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// separate | merge | auto
/// ```
///
// https://drafts.csswg.org/css-ruby-1/#ruby-merge
#[value(" separate | merge | auto ")]
#[initial("separate")]
#[applies_to("interlinear ruby annotation containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum RubyMergeStyleValue {}

/// Represents the style value for `ruby-align` as defined in [css-ruby-1](https://drafts.csswg.org/css-ruby-1/#ruby-align).
///
/// The ruby-align CSS property sets the spacing and alignment of ruby annotation text when it does not fill its available space.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// start | center | space-between | space-around
/// ```
///
// https://drafts.csswg.org/css-ruby-1/#ruby-align
#[value(" start | center | space-between | space-around ")]
#[initial("space-around")]
#[applies_to("ruby bases, ruby annotations, ruby base containers, ruby annotation containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:128,chrome_android:128,edge:128,firefox:38,firefox_android:38,safari:18.2,safari_ios:18.2)]
pub enum RubyAlignStyleValue {}

/// Represents the style value for `ruby-overhang` as defined in [css-ruby-1](https://drafts.csswg.org/css-ruby-1/#ruby-overhang).
///
/// The ruby-overhang CSS property sets whether ruby annotations may overlap adjacent text.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-ruby-1/#ruby-overhang
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("ruby annotation containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(safari:18.2,safari_ios:18.2)]
pub enum RubyOverhangStyleValue {}
