use css_parse::{Cursor, Kind, Span};
use miette::Diagnostic;
use thiserror::Error;

pub use css_parse::diagnostics::*;

#[derive(Debug, Error, Diagnostic)]
#[error("This cannot yet be parsed by the parser :(")]
#[diagnostic(code(css_parse::Unimplemented))]
#[help("This feature needs to be implemented within csskit. This file won't parse without it.")]
pub struct Unimplemented(#[label("Didn't recognise this bit")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule mut not have a 'prelude'.")]
#[diagnostic(code(css_parse::DisllowedAtRulePrelude))]
#[help("The 'prelude' is the bit between the @keyword and the {{")]
pub struct DisallowedAtRulePrelude(#[label("Remove this part")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule must not have a 'block'.")]
#[diagnostic(code(css_parse::DisllowedAtRuleBlock))]
#[help("The 'block' is the bit between the {{ and }}")]
pub struct DisallowedAtRuleBlock(#[label("Remove this part")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule must have a 'prelude'.")]
#[diagnostic(code(css_parse::MissingAtRulePrelude))]
#[help("The 'prelude' is the bit between the @ and the {{")]
pub struct MissingAtRulePrelude(#[label("Add content here")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule must have a 'block'.")]
#[diagnostic(code(css_parse::MissingAtRuleBlock))]
#[help("The 'block' is the bit between the {{ and }}")]
pub struct MissingAtRuleBlock(#[label("Add {{}} here")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected charset '{0}'. '{0}' isn't allowed here. This must be a valid IANA language code.")]
#[diagnostic(code(css_parse::UnexpectedCharset))]
#[help("Consider removing the rule or setting this to 'utf-8'")]
pub struct UnexpectedCharset(pub String, #[label("This charset code is not allowed here")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected literal '{0}'")]
#[diagnostic(code(css_parse::UnexpectedLiteral))]
#[help("Try removing the word here.")]
pub struct UnexpectedLiteral(pub String, #[label("??")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected identifier '{0}'. '{0}' isn't allowed here, but '{1}' is.")]
#[diagnostic(code(css_parse::UnexpectedIdentSuggest))]
#[help("Try changing this to '{1}'")]
pub struct UnexpectedIdentSuggest(pub String, pub String, #[label("This keyword is not allowed here")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected duplicate '{0}'")]
#[diagnostic(code(css_parse::UnexpectedDuplicateIdent))]
#[help("Try removing the word here.")]
pub struct UnexpectedDuplicateIdent(pub String, #[label("Remove this duplicate")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo selector ':{0}'")]
#[diagnostic(code(css_parse::UnexpectedPseudo))]
#[help("This isn't a valid psuedo selector for this rule.")]
pub struct UnexpectedPseudoClass(pub String, #[label("This psuedo selector")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo selector ':{0}'()")]
#[diagnostic(code(css_parse::UnexpectedPseudoClassFunction))]
#[help("This isn't a valid psuedo selector for this rule.")]
pub struct UnexpectedPseudoClassFunction(pub String, #[label("This psuedo selector")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo element '::{0}'")]
#[diagnostic(code(css_parse::UnexpectedPseudoElement))]
#[help("This isn't a valid psuedo selector for this rule.")]
pub struct UnexpectedPseudoElement(pub String, #[label("This psuedo selector")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo element '::{0}'")]
#[diagnostic(code(css_parse::UnexpectedPseudoElementFunction))]
#[help("This isn't a valid psuedo selector for this rule.")]
pub struct UnexpectedPseudoElementFunction(pub String, #[label("This psuedo selector")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("The dimension '{0}' wasn't recognised for this value type")]
#[diagnostic(code(css_parse::UnexpectedDimension))]
#[help(
	"This isn't a recognisable dimension for this value type. If it's a valid dimension, it might be that it cannot be used for this rule or in this position."
)]
pub struct UnexpectedDimension(pub String, #[label("This isn't recognised")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected at rule '@{0}'")]
#[diagnostic(code(css_parse::UnexpectedAtRule))]
#[help("This isn't a recognisable at-rule here. If the rule is valid, it might not be allowed here.")]
pub struct UnexpectedAtRule(pub String, #[label("This isn't recognised")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected function '{0}'()")]
#[diagnostic(code(css_parse::UnexpectedFunction))]
#[help("A function with this name wasn't expected in this position.")]
pub struct UnexpectedFunction(pub String, #[label("Here")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown Rule")]
#[diagnostic(code(css_parse::UnknownRule))]
#[help("This might be a mistake in the parser, please file an issue!")]
pub struct UnknownRule(#[label("Don't know how to interpret this")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown Value")]
#[diagnostic(code(css_parse::UnknownValue))]
#[help("This might be a mistake in the parser, please file an issue!")]
pub struct UnknownValue(#[label("Don't know how to interpret this")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown named color '{0}'")]
#[diagnostic(code(css_parse::UnknownColor))]
#[help("Replace this unknown color with a known named color or a valid color value.")]
pub struct UnknownColor(pub String, #[label("This isn't a known color")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected `{}` but found `{}`", self.0, Kind::from(self.1))]
#[diagnostic(code(css_parse::ExpectedToken))]
#[help("This is not correct CSS syntax.")]
pub struct ExpectedKind(pub Kind, #[label("`{}` expected", self.0)] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a dimension but found `{}`", Kind::from(self.0))]
#[diagnostic(code(css_parse::ExpectedDimension))]
#[help("This is not correct CSS syntax.")]
pub struct ExpectedDimension(#[label("dimension expected")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a function but found `{0}`")]
#[diagnostic(code(css_parse::ExpectedFunction))]
#[help("This is not correct CSS syntax.")]
pub struct ExpectedFunction(pub Kind, #[label("This token")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see {0}() but saw {1}()")]
#[diagnostic(code(css_parse::ExpectedFunctionOf))]
#[help("Try changing the {1}() to {0}()")]
pub struct ExpectedFunctionOf(pub String, pub String, #[label("This function")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an @ keyword but saw `{0}`")]
#[diagnostic(code(css_parse::ExpectedAtKeyword))]
#[help("This is not correct CSS syntax.")]
pub struct ExpectedAtKeyword(pub Kind, #[label("This at-keyword")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see @{0} but saw @{1}")]
#[diagnostic(code(css_parse::ExpectedAtKeywordOf))]
#[help("Try changing the @{1} to @{0}")]
pub struct ExpectedAtKeywordOf(pub String, pub String, #[label("This at-keyword")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see {0} but saw {1}")]
#[diagnostic(code(css_parse::ExpectedDelimOf))]
#[help("Try changing the {1} to {0}")]
pub struct ExpectedDelimOf(pub char, pub char, #[label("This delimiter")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid hexidecimal value for color: '{0}'")]
#[diagnostic(code(css_parse::BadHexColor))]
#[help("Hex colours must be 3, 4, 6 or 8 digits long.")]
pub struct BadHexColor(pub String, #[label("This is the wrong format")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This block uses an invalid selector, so the whole block would be discarded.")]
#[diagnostic(code(css_parse::NoSelector))]
#[help("Try adding a selector to this style rule")]
pub struct NoSelector(#[label("This selector isn't valid")] pub Span, pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This selector has two combinators next to each other, which is disallowed.")]
#[diagnostic(code(css_parse::AdjacentSelectorCombinators))]
#[help("Try removing one of the combinators or add a selector in between them")]
pub struct AdjacentSelectorCombinators(
	#[label("...because this combinator is right next to the previous one")] pub Span,
	#[label("This selector is invalid...")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("This selector has two types next to each other, which is disallowed.")]
#[diagnostic(code(css_parse::AdjacentSelectorTypes))]
#[help("Try removing one of the types or add a space inbetween")]
pub struct AdjacentSelectorTypes(
	#[label("...because this type is right next to the previous one.")] pub Span,
	#[label("This selector is invalid...")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("This value isn't allowed to be a raw number, it has to have a dimension.")]
#[diagnostic(code(css_parse::DisallowedValueWithoutDimension))]
#[help("Try adding a dimension, like '{0}'")]
pub struct DisallowedValueWithoutDimension(pub String, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("A math function isn't valid here.")]
#[diagnostic(code(css_parse::DisallowedMathFunction))]
#[help("var() and env() can be used but math functions like {0}() cannot.")]
pub struct DisallowedMathFunction(pub String, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an opening curly brace but saw `{}`", Kind::from(self.0))]
#[diagnostic(code(css_parse::ExpectedOpenCurly))]
#[help("This is not correct CSS syntax.")]
pub struct ExpectedOpenCurly(#[label("This value")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a number but saw `{0}`")]
#[diagnostic(code(css_parse::ExpectedNumber))]
#[help("This is not correct CSS syntax.")]
pub struct ExpectedNumber(pub Kind, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a signed number but saw `{0}`")]
#[diagnostic(code(css_parse::ExpectedSign))]
#[help("This number needs a + or a -.")]
pub struct ExpectedSign(pub f32, #[label("Add a + here")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an unsigned number but saw `{}`", self.0.token().value())]
#[diagnostic(code(css_parse::ExpectedUnsigned))]
#[help("This number cannot have a + or a -.")]
pub struct ExpectedUnsigned(#[label("This should be {}", self.0.token().value().abs())] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This number is out of bounds.")]
#[diagnostic(code(css_parse::NumberOutOfBounds))]
#[help("This needs to be a number between {1}.")]
pub struct NumberOutOfBounds(pub f32, pub String, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number cannot be negative.")]
#[diagnostic(code(css_parse::NumberNotNegative))]
#[help("This needs to be greater or equal to 0")]
pub struct NumberNotNegative(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number is too small.")]
#[diagnostic(code(css_parse::NumberTooSmall))]
#[help("This needs to be larger than {0}")]
pub struct NumberTooSmall(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number is too large.")]
#[diagnostic(code(css_parse::NumberTooLarge))]
#[help("This needs to be smaller than {0}")]
pub struct NumberTooLarge(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This value isn't allowed to have a fraction, it must be a whole number (integer).")]
#[diagnostic(code(css_parse::ExpectedInt))]
#[help("Try using {} instead", self.0.token().value().round())]
pub struct ExpectedInt(#[label("This value")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This value must have a fraction, it must be float.")]
#[diagnostic(code(css_parse::ExpectedFloat))]
#[help("Try using {} instead", self.0.token().value())]
pub struct ExpectedFloat(#[label("This value")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This number must be 0, got {} instead.", self.0.token().value())]
#[diagnostic(code(css_parse::ExpectedZero))]
#[help("Try replacing it with the literal 0 instead")]
pub struct ExpectedZero(#[label("This value")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This number must not be 0.")]
#[diagnostic(code(css_parse::ExpectedZero))]
#[help("Try replacing it with a positive or negative number")]
pub struct UnexpectedZero(#[label("This value")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("This media query tries to compare itself equal to two different numbers.")]
#[diagnostic(code(css_parse::UnexpectedMediaRangeComparisonEqualsTwice))]
#[help("Try deleting one.")]
pub struct UnexpectedMediaRangeComparisonEqualsTwice(#[label("This comparison")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Display 'list-item' can only be combined with 'flow' or 'flow-root'")]
#[diagnostic(code(css_parse::DisplayHasInvalidListItemCombo))]
#[help("{0} is not valid in combination with list-item, try changing it to 'flow' or 'flow-root'")]
pub struct DisplayHasInvalidListItemCombo(pub String, pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("hwb and hsl colors must have a hue as their first argument.")]
#[diagnostic(code(css_parse::ColorMustStartWithHue))]
#[help("Try adding a % to the first color component.")]
pub struct ColorMustStartWithHue(#[label("This component")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Only hwb and hsl colors have a hue as their first argument.")]
#[diagnostic(code(css_parse::ColorMustNotStartWithHue))]
#[help("Try removing the %")]
pub struct ColorMustNotStartWithHue(#[label("This component")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Colors should not use a hue as the middle color component")]
#[diagnostic(code(css_parse::ColorMustNotStartWithHue))]
#[help("Try removing the %")]
pub struct ColorMustNotHaveHueInMiddle(#[label("This component")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Colors using the legacy syntax must have commas between the components")]
#[diagnostic(code(css_parse::ColorLegacyMustIncludeComma))]
#[help("Try using the non-legacy syntax, without commas")]
pub struct ColorLegacyMustIncludeComma(#[label("Put a commma here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Colors using the legacy syntax must not use percentages, but absolute numbers")]
#[diagnostic(code(css_parse::ColorLegacyMustNotUsePercent))]
#[help("Try removing the %, or use the non-legacy syntax")]
pub struct ColorLegacyMustNotUsePercent(#[label("This should not be a percentage")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Hex colors can be 3, 4, 6, or 8 characters in length. This one is {0}")]
#[diagnostic(code(css_parse::ColorLegacyMustNotUsePercent))]
#[help("Try rewriting this to be 3, 4, 6 or 8 characters")]
pub struct ColorHexWrongLength(pub usize, #[label("This is not the right number of characters")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("{0} cannot be used as a keyframe name, as it's a reserved word.")]
#[diagnostic(code(css_parse::ReservedKeyframeName))]
#[help("")]
pub struct ReservedKeyframeName(pub String, #[label("Rename it, or try wrapping it in quotes")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("An @layer {{}} (block) rule cannot have multiple names.")]
#[diagnostic(code(css_parse::DisallowedLayerBlockWithMultipleNames))]
#[help("")]
pub struct DisallowedLayerBlockWithMultipleNames(#[label("Remove most (or all) of these names.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("!important cannot be used for this property")]
#[diagnostic(code(css_parse::DisallowedImportant))]
#[help("")]
pub struct DisallowedImportant(#[label("Remove this.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("{0} requires at least {1} arguments, but saw {2}")]
#[diagnostic(code(css_parse::DisallowedImportant))]
#[help("")]
pub struct NotEnoughArguments(
	pub String,
	pub usize,
	pub usize,
	#[label("Add another argument to this function.")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("'{0}' not a valid <counter-name>")]
#[diagnostic(code(css_parse::Unexpected))]
#[help("")]
pub struct InvalidCounterName(pub String, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This attr(name) is invalid.")]
#[diagnostic(code(css_parse::Unexpected))]
#[help("Try adding a | between each name.")]
pub struct InvalidAttrName(#[label("This value")] pub Span);
