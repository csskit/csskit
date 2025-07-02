mod impls;
use impls::*;

/*
 * https://drafts.csswg.org/css-grid-3/
 * CSS Grid Layout Module Level 3
 */

// // https://drafts.csswg.org/css-grid-3/#grid-template-columns
// #[value(" none | <track-list> | <auto-track-list> | subgrid <line-name-list>? ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the content area")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item in the computed track list (see § 7.2.5 computed value of a track listing and § 7.2.3.3 interpolation/combination of repeat()); discrete otherwise")]
// pub enum GridTemplateColumnsStyleValue {}

// // https://drafts.csswg.org/css-grid-3/#grid-template-rows
// #[value(" none | <track-list> | <auto-track-list> | subgrid <line-name-list>? ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the content area")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item in the computed track list (see § 7.2.5 computed value of a track listing and § 7.2.3.3 interpolation/combination of repeat()); discrete otherwise")]
// pub enum GridTemplateRowsStyleValue {}

// https://drafts.csswg.org/css-grid-3/#grid-template-areas
#[value(" none | <string>+ ")]
#[initial("none")]
#[applies_to("grid containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum GridTemplateAreasStyleValue<'a> {}

// // https://drafts.csswg.org/css-grid-3/#grid-template
// #[value(" none | [ <'grid-template-rows'> / <'grid-template-columns'> ] | [ <line-names>? <string> <track-size>? <line-names>? ]+ [ / <explicit-track-list> ]? ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum GridTemplateStyleValue {}

// // https://drafts.csswg.org/css-grid-3/#grid-auto-columns
// #[value(" <track-size>+ ")]
// #[initial("auto")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("see track sizing")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item; discrete otherwise")]
// pub struct GridAutoColumnsStyleValue<'a>;

// // https://drafts.csswg.org/css-grid-3/#grid-auto-rows
// #[value(" <track-size>+ ")]
// #[initial("auto")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("see track sizing")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item; discrete otherwise")]
// pub struct GridAutoRowsStyleValue<'a>;

// // https://drafts.csswg.org/css-grid-3/#grid-auto-flow
// #[value(" [ row | column ] || dense ")]
// #[initial("row")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum GridAutoFlowStyleValue {}

// // https://drafts.csswg.org/css-grid-3/#grid
// #[value(" <'grid-template'> | <'grid-template-rows'> / [ auto-flow && dense? ] <'grid-auto-columns'>? | [ auto-flow && dense? ] <'grid-auto-rows'>? / <'grid-template-columns'> ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum GridStyleValue {}

// // https://drafts.csswg.org/css-grid-3/#grid-row-start
// #[value(" <grid-line> ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridRowStartStyleValue;

// // https://drafts.csswg.org/css-grid-3/#grid-column-start
// #[value(" <grid-line> ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridColumnStartStyleValue;

// // https://drafts.csswg.org/css-grid-3/#grid-row-end
// #[value(" <grid-line> ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridRowEndStyleValue;

// // https://drafts.csswg.org/css-grid-3/#grid-column-end
// #[value(" <grid-line> ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridColumnEndStyleValue;

// // https://drafts.csswg.org/css-grid-3/#grid-row
// #[value(" <grid-line> [ / <grid-line> ]? ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridRowStyleValue;

// // https://drafts.csswg.org/css-grid-3/#grid-column
// #[value(" <grid-line> [ / <grid-line> ]? ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridColumnStyleValue;

// // https://drafts.csswg.org/css-grid-3/#grid-area
// #[value(" <grid-line> [ / <grid-line> ]{0,3} ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridAreaStyleValue;

// https://drafts.csswg.org/css-grid-3/#item-slack
#[value(" <length-percentage> | infinite ")]
#[initial("1em")]
#[applies_to("masonry containers")]
#[inherited("no")]
#[percentages("relative to the grid-axis content box size of the masonry container")]
#[canonical_order("per grammar")]
#[animation_type("as length")]
pub enum ItemSlackStyleValue {}

// https://drafts.csswg.org/css-grid-3/#item-direction
#[value(" auto | row | column | row-reverse | column-reverse ")]
#[initial("auto")]
#[applies_to("flex containers, grid containers, masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ItemDirectionStyleValue {}

// https://drafts.csswg.org/css-grid-3/#item-track
#[value(" auto | row | column | row-reverse | column-reverse ")]
#[initial("auto")]
#[applies_to("flex containers, grid containers, masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ItemTrackStyleValue {}

// // https://drafts.csswg.org/css-grid-3/#item-wrap
// #[value(" [ auto | nowrap | wrap ] || [ normal | reverse ] | wrap-reverse ")]
// #[initial("auto")]
// #[applies_to("flex containers, grid containers, masonry containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ItemWrapStyleValue {}

// // https://drafts.csswg.org/css-grid-3/#item-cross
// #[value(" [ auto | nowrap | wrap ] || [ normal | reverse ] | wrap-reverse ")]
// #[initial("auto")]
// #[applies_to("flex containers, grid containers, masonry containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ItemCrossStyleValue {}

// // https://drafts.csswg.org/css-grid-3/#item-pack
// #[value(" normal | dense || balance ")]
// #[initial("normal")]
// #[applies_to("flex containers, grid containers, masonry containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum ItemPackStyleValue {}

// // https://drafts.csswg.org/css-grid-3/#item-flow
// #[value(" <'item-direction'> || <'item-wrap'> || <'item-pack'> || <'item-slack'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct ItemFlowStyleValue;
