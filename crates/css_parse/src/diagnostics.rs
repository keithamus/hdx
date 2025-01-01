use css_lexer::{Cursor, Kind, Span};
use miette::{self, Diagnostic};
use thiserror::{self, Error};

#[derive(Debug, Error, Diagnostic)]
#[error("The token at {0} cannot yet be parsed by the parser :(")]
#[diagnostic(
	help("This feature needs to be implemented within hdx. This file won't parse without it."),
	code(css_parse::Unimplemented)
)]
pub struct Unimplemented(#[label("Didn't recognise this bit")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule mut not have a 'prelude'.")]
#[diagnostic(help("The 'prelude' is the bit between the @keyword and the {{"), code(css_parse::DisllowedAtRulePrelude))]
pub struct DisallowedAtRulePrelude(#[label("Remove this part")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule must not have a 'block'.")]
#[diagnostic(help("The 'block' is the bit between the {{ and }}"), code(css_parse::DisllowedAtRuleBlock))]
pub struct DisallowedAtRuleBlock(#[label("Remove this part")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule must have a 'prelude'.")]
#[diagnostic(help("The 'prelude' is the bit between the @ and the {{"), code(css_parse::MissingAtRulePrelude))]
pub struct MissingAtRulePrelude(#[label("Add content here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This at-rule must have a 'block'.")]
#[diagnostic(help("The 'block' is the bit between the {{ and }}"), code(css_parse::MissingAtRuleBlock))]
pub struct MissingAtRuleBlock(#[label("Add {{}} here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This declaration wasn't understood, and so was disregarded.")]
#[diagnostic(help("The declaration contains invalid syntax, and will be ignored."), code(css_parse::BadDeclaration))]
pub struct BadDeclaration(#[label("This is not valid syntax for a declaration.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::Unexpected))]
pub struct Unexpected(pub Kind, #[label("This wasn't expected here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected charset '{0}'. '{0}' isn't allowed here. This must be a valid IANA language code.")]
#[diagnostic(help("Consider removing the rule or setting this to 'utf-8'"), code(css_parse::UnexpectedCharset))]
pub struct UnexpectedCharset(pub String, #[label("This charset code is not allowed here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected identifier '{0}'")]
#[diagnostic(help("Try removing the word here."), code(css_parse::UnexpectedIdent))]
pub struct UnexpectedIdent(pub String, #[label("??")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected identifier '{0}'. '{0}' isn't allowed here, but '{1}' is.")]
#[diagnostic(help("Try changing this to '{1}'"), code(css_parse::UnexpectedIdentSuggest))]
pub struct UnexpectedIdentSuggest(pub String, pub String, #[label("This keyword is not allowed here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected duplicate '{0}'")]
#[diagnostic(help("Try removing the word here."), code(css_parse::UnexpectedDuplicateIdent))]
pub struct UnexpectedDuplicateIdent(pub String, #[label("Remove this duplicate")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected delimeter '{0}'")]
#[diagnostic(help("Try removing the the character."), code(css_parse::UnexpectedDelim))]
pub struct UnexpectedDelim(pub char, #[label("This character wasn't understood")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo selector ':{0}'")]
#[diagnostic(help("This isn't a valid psuedo selector for this rule."), code(css_parse::UnexpectedPseudo))]
pub struct UnexpectedPseudoClass(pub String, #[label("This psuedo selector")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo selector ':{0}'()")]
#[diagnostic(help("This isn't a valid psuedo selector for this rule."), code(css_parse::UnexpectedPseudoClassFunction))]
pub struct UnexpectedPseudoClassFunction(pub String, #[label("This psuedo selector")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo element '::{0}'")]
#[diagnostic(help("This isn't a valid psuedo selector for this rule."), code(css_parse::UnexpectedPseudoElement))]
pub struct UnexpectedPseudoElement(pub String, #[label("This psuedo selector")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected pseudo element '::{0}'")]
#[diagnostic(
	help("This isn't a valid psuedo selector for this rule."),
	code(css_parse::UnexpectedPseudoElementFunction)
)]
pub struct UnexpectedPseudoElementFunction(pub String, #[label("This psuedo selector")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected tag name ':{0}'")]
#[diagnostic(help("This isn't a valid tag name."), code(css_parse::UnexpectedTag))]
pub struct UnexpectedTag(pub String, #[label("This tag")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected ID selector ':{0}'")]
#[diagnostic(help("This isn't a valid ID."), code(css_parse::UnexpectedId))]
pub struct UnexpectedId(pub String, #[label("This ID")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("The dimension '{0}' wasn't recognised for this value type")]
#[diagnostic(
	help(
		"This isn't a recognisable dimension for this value type. If it's a valid dimension, it might be that it cannot be used for this rule or in this position."
	),
	code(css_parse::UnexpectedDimension)
)]
pub struct UnexpectedDimension(pub String, #[label("This isn't recognised")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected at rule '@{0}'")]
#[diagnostic(
	help("This isn't a recognisable at-rule here. If the rule is valid, it might not be allowed here."),
	code(css_parse::UnexpectedAtRule)
)]
pub struct UnexpectedAtRule(pub String, #[label("This isn't recognised")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected function '{0}'()")]
#[diagnostic(help("This function wasn't expected in this position."), code(css_parse::UnexpectedFunction))]
pub struct UnexpectedFunction(pub String, #[label("??")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown Rule")]
#[diagnostic(help("This might be a mistake in the parser, please file an issue!"), code(css_parse::UnknownRule))]
pub struct UnknownRule(#[label("Don't know how to interpret this")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Ignored property due to parse error.")]
#[diagnostic(
	help("This property is going to be ignored because it doesn't look valid. If it is valid, please file an issue!"),
	code(css_parse::UnknownDeclaration)
)]
pub struct UnknownDeclaration(#[label("This property was ignored.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown Value")]
#[diagnostic(help("This might be a mistake in the parser, please file an issue!"), code(css_parse::UnknownValue))]
pub struct UnknownValue(#[label("Don't know how to interpret this")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown named color '{0}'")]
#[diagnostic(
	help("Replace this unknown color with a known named color or a valid color value."),
	code(css_parse::UnknownColor)
)]
pub struct UnknownColor(pub String, #[label("This isn't a known color")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected this to be the end of the file, but there was more content.")]
#[diagnostic(
	help("This is likely a problem with the parser. Please submit a bug report!"),
	code(css_parse::ExpectedEnd)
)]
pub struct ExpectedEnd(#[label("All of this extra content was ignored.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected more content but reached the end of the file.")]
#[diagnostic(help("Perhaps this file isn't finished yet?"), code(css_parse::UnexpectedEnd))]
pub struct UnexpectedEnd();

#[derive(Debug, Error, Diagnostic)]
#[error("Expected more content before this curly brace.")]
#[diagnostic(help("This needed more content here"), code(css_parse::UnexpectedCloseCurly))]
pub struct UnexpectedCloseCurly(pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected `{0}` but found `{1}` {2}")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::ExpectedToken))]
pub struct ExpectedKind(pub Kind, pub Kind, #[label("`{0}` expected")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a dimension but found `{1}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::ExpectedDimension))]
pub struct ExpectedDimension(pub Cursor, #[label("dimension expected")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an identifier but found `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::ExpectedIdent))]
pub struct ExpectedIdent(pub Kind, #[label("This should be `{0}`")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an identifier but not `{0}`")]
#[diagnostic(help("This is wrong. Maybe it is misspelled?"), code(css_parse::ExpectedOtherIdent))]
pub struct ExpectedOtherIdent(pub String, #[label("This cannot be `{0}`")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected the identifier `{0}` but found `{1}`")]
#[diagnostic(help("Try changing `{1}` to `{0}`."), code(css_parse::ExpectedIdentOf))]
pub struct ExpectedIdentOf(pub &'static str, pub String, #[label("This should be `{0}`")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a function but found `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::ExpectedFunction))]
pub struct ExpectedFunction(pub Kind, #[label("This token")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see {0}() but saw {1}()")]
#[diagnostic(help("Try changing the {1}() to {0}()"), code(css_parse::ExpectedFunctionOf))]
pub struct ExpectedFunctionOf(pub String, pub String, #[label("This function")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an @ keyword but saw `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::ExpectedAtKeyword))]
pub struct ExpectedAtKeyword(pub Kind, #[label("This at-keyword")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see @{0} but saw @{1}")]
#[diagnostic(help("Try changing the @{1} to @{0}"), code(css_parse::ExpectedAtKeywordOf))]
pub struct ExpectedAtKeywordOf(pub String, pub String, #[label("This at-keyword")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a delimiter but saw `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::ExpectedDelim))]
pub struct ExpectedDelim(pub Kind, #[label("This at-keyword")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected to see {0} but saw {1}")]
#[diagnostic(help("Try changing the {1} to {0}"), code(css_parse::ExpectedDelimOf))]
pub struct ExpectedDelimOf(pub char, pub char, #[label("This delimiter")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid hexidecimal value for color: '{0}'")]
#[diagnostic(help("Hex colours must be 3, 4, 6 or 8 digits long."), code(css_parse::BadHexColor))]
pub struct BadHexColor(pub String, #[label("This is the wrong format")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This block uses an invalid selector, so the whole block would be discarded.")]
#[diagnostic(help("Try adding a selector to this style rule"), code(css_parse::NoSelector))]
pub struct NoSelector(#[label("This selector isn't valid")] pub Span, pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This selector has two combinators next to each other, which is disallowed.")]
#[diagnostic(
	help("Try removing one of the combinators or add a selector in between them"),
	code(css_parse::AdjacentSelectorCombinators)
)]
pub struct AdjacentSelectorCombinators(
	#[label("...because this combinator is right next to the previous one")] pub Span,
	#[label("This selector is invalid...")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("This selector has two types next to each other, which is disallowed.")]
#[diagnostic(help("Try removing one of the types or add a space inbetween"), code(css_parse::AdjacentSelectorTypes))]
pub struct AdjacentSelectorTypes(
	#[label("...because this type is right next to the previous one.")] pub Span,
	#[label("This selector is invalid...")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("This value isn't allowed to be a raw number, it has to have a dimension.")]
#[diagnostic(help("Try adding a dimension, like '{0}'"), code(css_parse::DisallowedValueWithoutDimension))]
pub struct DisallowedValueWithoutDimension(pub String, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("A math function isn't valid here.")]
#[diagnostic(
	help("var() and env() can be used but math functions like {0}() cannot."),
	code(css_parse::DisallowedMathFunction)
)]
pub struct DisallowedMathFunction(pub String, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an opening curly brace but saw `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::ExpectedOpenCurly))]
pub struct ExpectedOpenCurly(pub Kind, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a number but saw `{0}`")]
#[diagnostic(help("This is not correct CSS syntax."), code(css_parse::ExpectedNumber))]
pub struct ExpectedNumber(pub Kind, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a signed number but saw `{0}`")]
#[diagnostic(help("This number needs a + or a -."), code(css_parse::ExpectedSign))]
pub struct ExpectedSign(pub f32, #[label("Add a + here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an unsigned number but saw `{0}`")]
#[diagnostic(help("This number cannot have a + or a -."), code(css_parse::ExpectedUnsigned))]
pub struct ExpectedUnsigned(pub f32, #[label("Remove the sign")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number is out of bounds.")]
#[diagnostic(help("This needs to be a number between {1}."), code(css_parse::NumberOutOfBounds))]
pub struct NumberOutOfBounds(pub f32, pub String, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number cannot be negative.")]
#[diagnostic(help("This needs to be greater or equal to 0"), code(css_parse::NumberNotNegative))]
pub struct NumberNotNegative(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number is too small.")]
#[diagnostic(help("This needs to be larger than {0}"), code(css_parse::NumberTooSmall))]
pub struct NumberTooSmall(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number is too large.")]
#[diagnostic(help("This needs to be smaller than {0}"), code(css_parse::NumberTooLarge))]
pub struct NumberTooLarge(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This value isn't allowed to have a fraction, it must be a whole number (integer).")]
#[diagnostic(help("Try using {0} instead"), code(css_parse::ExpectedInt))]
pub struct ExpectedInt(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This value must have a fraction, it must be float.")]
#[diagnostic(help("Try using {0} instead"), code(css_parse::ExpectedFloat))]
pub struct ExpectedFloat(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This number must be 0, got {0} instead.")]
#[diagnostic(help("Try replacing it with the literal 0 instead"), code(css_parse::ExpectedZero))]
pub struct ExpectedZero(pub f32, #[label("This value")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("This media query tries to compare itself equal to two different numbers.")]
#[diagnostic(help("Try deleting one."), code(css_parse::UnexpectedMediaRangeComparisonEqualsTwice))]
pub struct UnexpectedMediaRangeComparisonEqualsTwice(#[label("This comparison")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Display 'list-item' can only be combined with 'flow' or 'flow-root'")]
#[diagnostic(
	help("{0} is not valid in combination with list-item, try changing it to 'flow' or 'flow-root'"),
	code(css_parse::DisplayHasInvalidListItemCombo)
)]
pub struct DisplayHasInvalidListItemCombo(pub String, pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("hwb and hsl colors must have a hue as their first argument.")]
#[diagnostic(help("Try adding a % to the first color component."), code(css_parse::ColorMustStartWithHue))]
pub struct ColorMustStartWithHue(#[label("This component")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Only hwb and hsl colors have a hue as their first argument.")]
#[diagnostic(help("Try removing the %"), code(css_parse::ColorMustNotStartWithHue))]
pub struct ColorMustNotStartWithHue(#[label("This component")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Colors should not use a hue as the middle color component")]
#[diagnostic(help("Try removing the %"), code(css_parse::ColorMustNotStartWithHue))]
pub struct ColorMustNotHaveHueInMiddle(#[label("This component")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Colors using the legacy syntax must have commas between the components")]
#[diagnostic(help("Try using the non-legacy syntax, without commas"), code(css_parse::ColorLegacyMustIncludeComma))]
pub struct ColorLegacyMustIncludeComma(#[label("Put a commma here")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Colors using the legacy syntax must not use percentages, but absolute numbers")]
#[diagnostic(help("Try removing the %, or use the non-legacy syntax"), code(css_parse::ColorLegacyMustNotUsePercent))]
pub struct ColorLegacyMustNotUsePercent(#[label("This should not be a percentage")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Hex colors can be 3, 4, 6, or 8 characters in length. This one is {0}")]
#[diagnostic(help("Try rewriting this to be 3, 4, 6 or 8 characters"), code(css_parse::ColorLegacyMustNotUsePercent))]
pub struct ColorHexWrongLength(pub usize, #[label("This is not the right number of characters")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("{0} cannot be used as a keyframe name, as it's a reserved word.")]
#[diagnostic(help(""), code(css_parse::ReservedKeyframeName))]
pub struct ReservedKeyframeName(pub String, #[label("Rename it, or try wrapping it in quotes")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("An @layer {{}} (block) rule cannot have multiple names.")]
#[diagnostic(help(""), code(css_parse::DisallowedLayerBlockWithMultipleNames))]
pub struct DisallowedLayerBlockWithMultipleNames(#[label("Remove most (or all) of these names.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("!important cannot be used for this property")]
#[diagnostic(help(""), code(css_parse::DisallowedImportant))]
pub struct DisallowedImportant(#[label("Remove this.")] pub Span);
