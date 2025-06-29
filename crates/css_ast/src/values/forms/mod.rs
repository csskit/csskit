mod impls;
use impls::*;

/*
 * https://drafts.csswg.org/css-forms-1/
 * CSS Form Control Styling Level 1
 */

// https://drafts.csswg.org/css-forms-1/#field-sizing
#[value(" fixed | content ")]
#[initial("fixed")]
#[applies_to("elements with default preferred size")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FieldSizingStyleValue {}

// https://drafts.csswg.org/css-forms-1/#slider-orientation
#[value(" auto | left-to-right | right-to-left | top-to-bottom | bottom-to-top ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum SliderOrientationStyleValue {}

// https://drafts.csswg.org/css-forms-1/#input-security
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("sensitive text inputs")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum InputSecurityStyleValue {}
