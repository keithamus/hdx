use super::prelude::*;

use crate::{CrossFadeFunction, ElementFunction, Gradient, ImageFunction, ImageSetFunction, Url};
use css_parse::Box;

/// <https://drafts.csswg.org/css-images-4/#typedef-image>
///
/// ```text
/// <image> = <url> | <image()> | <image-set()> | <cross-fade()> | <element()> | <gradient>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Image<'a> {
	Url(Url),
	#[atom(CssAtomSet::Image)]
	ImageFunction(Box<'a, ImageFunction<'a>>),
	#[atom(CssAtomSet::ImageSet)]
	ImageSetFunction(Box<'a, ImageSetFunction<'a>>),
	#[atom(CssAtomSet::CrossFade)]
	CrossFadeFunction(Box<'a, CrossFadeFunction<'a>>),
	#[atom(CssAtomSet::Element)]
	ElementFunction(Box<'a, ElementFunction>),
	Gradient(Gradient<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Image, "url('foo')");
		assert_parse!(CssAtomSet::ATOMS, Image, "url(\"foo\")");
		assert_parse!(CssAtomSet::ATOMS, Image, "url(foo)");
		assert_parse!(CssAtomSet::ATOMS, Image, "image(url(foo),red)");
		assert_parse!(CssAtomSet::ATOMS, Image, "image-set(url(a)2x)");
		assert_parse!(CssAtomSet::ATOMS, Image, "cross-fade(url(a)20%,url(b))");
		assert_parse!(CssAtomSet::ATOMS, Image, "element(#foo)");
		assert_parse!(CssAtomSet::ATOMS, Image, "linear-gradient(red,blue)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("url('foo')", Image, Url);
		assert_visits!("linear-gradient(red, blue)", Image, Gradient, LinearGradientFunction, Color, Color);
		assert_visits!("element(#foo)", Image, ElementFunction, Id);
		assert_visits!("image-set(url(a)2x)", Image, ImageSetFunction);
		assert_visits!("cross-fade(url(a))", Image, CrossFadeFunction, Image, Url);
	}
}
