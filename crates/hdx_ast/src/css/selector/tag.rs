use hdx_atom::{atom, Atom, Atomizable};
use hdx_derive::Atomizable;
use hdx_lexer::Kind;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Tag {
	Html(HtmlTag),
	Svg(SvgTag),
	Mathml(MathmlTag),
	Custom(Atom),
	Unknown(Atom),
}

impl Tag {
	pub fn to_custom_element(atom: &Atom) -> Option<Self> {
		let lower = atom.to_ascii_lowercase();
		if matches!(
			lower,
			atom!("annotation-xml")
				| atom!("color-profile")
				| atom!("font-face")
				| atom!("font-face-src")
				| atom!("font-face-uri")
				| atom!("font-face-format")
				| atom!("font-face-name")
				| atom!("missing-glyph")
		) {
			return None;
		}

		let mut chars = lower.chars();
		if !matches!(chars.next(), Some('a'..='z')) {
			return None;
		}
		let mut has_dash = false;
		while let Some(char) = chars.next() {
			if char == '-' {
				has_dash = true;
				continue;
			}
			if !matches!(char,
				'.' |
				'_' |
				'0'..='9' |
				'a'..='z' |
				'\u{b7}' |
				'\u{c0}'..='\u{d6}' |
				'\u{d8}'..='\u{f6}' |
				'\u{f8}'..='\u{37d}' |
				'\u{37F}'..='\u{1fff}' |
				'\u{200c}'..='\u{200d}' |
				'\u{203f}'..='\u{2040}' |
				'\u{2070}'..='\u{218f}' |
				'\u{2c00}'..='\u{2fef}' |
				'\u{3001}'..='\u{d7ff}' |
				'\u{f900}'..='\u{fdcf}' |
				'\u{fdf0}'..='\u{fffd}' |
				'\u{10000}'..='\u{effff}'
			) {
				return None;
			}
		}
		if !has_dash {
			return None;
		}
		Some(Self::Custom(lower))
	}
}

impl Atomizable for Tag {
	fn from_atom(atom: &Atom) -> Option<Self> {
		if let Some(html) = HtmlTag::from_atom(atom) {
			return Some(Self::Html(html));
		}
		if let Some(svg) = SvgTag::from_atom(atom) {
			return Some(Self::Svg(svg));
		}
		if let Some(math) = MathmlTag::from_atom(atom) {
			return Some(Self::Mathml(math));
		}
		Self::to_custom_element(atom).or_else(|| Some(Self::Unknown(atom.clone())))
	}

	fn to_atom(&self) -> Atom {
		match self {
			Self::Html(a) => a.to_atom(),
			Self::Svg(a) => a.to_atom(),
			Self::Mathml(a) => a.to_atom(),
			Self::Custom(a) => a.clone(),
			Self::Unknown(a) => a.clone(),
		}
	}
}

impl<'a> Parse<'a> for Tag {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.next();
		match token.kind() {
			Kind::Ident => {
				let atom = parser.parse_atom(token);
				if let Some(tag) = Tag::from_atom(&atom) {
					Ok(tag)
				} else {
					unexpected_ident!(parser, atom)
				}
			}
			_ => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Tag {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.to_atom().write_css(sink)
	}
}

#[derive(Atomizable, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum HtmlTag {
	A,           // atom!("a")
	Abbr,        // atom!("abbr")
	Acronym,     // atom!("acronym")
	Address,     // atom!("address")
	Area,        // atom!("area")
	Article,     // atom!("article")
	Aside,       // atom!("aside")
	Audio,       // atom!("audio")
	B,           // atom!("b")
	Base,        // atom!("base")
	Bdi,         // atom!("bdi")
	Bdo,         // atom!("bdo")
	Big,         // atom!("big")
	Blockquote,  // atom!("blockquote")
	Body,        // atom!("body")
	Br,          // atom!("br")
	Button,      // atom!("button")
	Canvas,      // atom!("canvas")
	Caption,     // atom!("caption")
	Center,      // atom!("center")
	Cite,        // atom!("cite")
	Code,        // atom!("code")
	Col,         // atom!("col")
	Colgroup,    // atom!("colgroup")
	Data,        // atom!("data")
	Datalist,    // atom!("datalist")
	Dd,          // atom!("dd")
	Del,         // atom!("del")
	Details,     // atom!("details")
	Dfn,         // atom!("dfn")
	Dialog,      // atom!("dialog")
	Dir,         // atom!("dir")
	Div,         // atom!("div")
	Dl,          // atom!("dl")
	Dt,          // atom!("dt")
	Em,          // atom!("em")
	Embed,       // atom!("embed")
	Fencedframe, // atom!("fencedframe")
	Fieldset,    // atom!("fieldset")
	Figcaption,  // atom!("figcaption")
	Figure,      // atom!("figure")
	Font,        // atom!("font")
	Footer,      // atom!("footer")
	Form,        // atom!("form")
	Frame,       // atom!("frame")
	Frameset,    // atom!("frameset")
	Head,        // atom!("head")
	Header,      // atom!("header")
	Hgroup,      // atom!("hgroup")
	Hr,          // atom!("hr")
	Html,        // atom!("html")
	I,           // atom!("i")
	Iframe,      // atom!("iframe")
	Img,         // atom!("img")
	Input,       // atom!("input")
	Ins,         // atom!("ins")
	Kbd,         // atom!("kbd")
	Label,       // atom!("label")
	Legend,      // atom!("legend")
	Li,          // atom!("li")
	Link,        // atom!("link")
	Main,        // atom!("main")
	Map,         // atom!("map")
	Mark,        // atom!("mark")
	Marquee,     // atom!("marquee")
	Menu,        // atom!("menu")
	Menuitem,    // atom!("menuitem")
	Meta,        // atom!("meta")
	Meter,       // atom!("meter")
	Nav,         // atom!("nav")
	Nobr,        // atom!("nobr")
	Noembed,     // atom!("noembed")
	Noframes,    // atom!("noframes")
	Noscript,    // atom!("noscript")
	Object,      // atom!("object")
	Ol,          // atom!("ol")
	Optgroup,    // atom!("optgroup")
	Option,      // atom!("option")
	Output,      // atom!("output")
	P,           // atom!("p")
	Param,       // atom!("param")
	Picture,     // atom!("picture")
	Plaintext,   // atom!("plaintext")
	Portal,      // atom!("portal")
	Pre,         // atom!("pre")
	Progress,    // atom!("progress")
	Q,           // atom!("q")
	Rb,          // atom!("rb")
	Rp,          // atom!("rp")
	Rt,          // atom!("rt")
	Rtc,         // atom!("rtc")
	Ruby,        // atom!("ruby")
	S,           // atom!("s")
	Samp,        // atom!("samp")
	Script,      // atom!("script")
	Search,      // atom!("search")
	Section,     // atom!("section")
	Select,      // atom!("select")
	Slot,        // atom!("slot")
	Small,       // atom!("small")
	Source,      // atom!("source")
	Span,        // atom!("span")
	Strike,      // atom!("strike")
	Strong,      // atom!("strong")
	Style,       // atom!("style")
	Sub,         // atom!("sub")
	Summary,     // atom!("summary")
	Sup,         // atom!("sup")
	Table,       // atom!("table")
	Tbody,       // atom!("tbody")
	Td,          // atom!("td")
	Template,    // atom!("template")
	Textarea,    // atom!("textarea")
	Tfoot,       // atom!("tfoot")
	Th,          // atom!("th")
	Thead,       // atom!("thead")
	Time,        // atom!("time")
	Title,       // atom!("title")
	Tr,          // atom!("tr")
	Track,       // atom!("track")
	Tt,          // atom!("tt")
	U,           // atom!("u")
	Ul,          // atom!("ul")
	Var,         // atom!("var")
	Video,       // atom!("video")
	Wbr,         // atom!("wbr")
	Xmp,         // atom!("xmp")
}

#[derive(Atomizable, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SvgTag {
	A,                   // atom!("a")
	Animate,             // atom!("animate")
	Animatemotion,       // atom!("animatemotion")
	Animatetransform,    // atom!("animatetransform")
	Circle,              // atom!("circle")
	Clippath,            // atom!("clippath")
	Cursor,              // atom!("cursor")
	Defs,                // atom!("defs")
	Desc,                // atom!("desc")
	Ellipse,             // atom!("ellipse")
	Feblend,             // atom!("feblend")
	Fecolormatrix,       // atom!("fecolormatrix")
	Fecomponenttransfer, // atom!("fecomponenttransfer")
	Fecomposite,         // atom!("fecomposite")
	Feconvolvematrix,    // atom!("feconvolvematrix")
	Fediffuselighting,   // atom!("fediffuselighting")
	Fedisplacementmap,   // atom!("fedisplacementmap")
	Fedistantlight,      // atom!("fedistantlight")
	Fedropshadow,        // atom!("fedropshadow")
	Feflood,             // atom!("feflood")
	Fefunca,             // atom!("fefunca")
	Fefuncb,             // atom!("fefuncb")
	Fefuncg,             // atom!("fefuncg")
	Fefuncr,             // atom!("fefuncr")
	Fegaussianblur,      // atom!("fegaussianblur")
	Feimage,             // atom!("feimage")
	Femerge,             // atom!("femerge")
	Femergenode,         // atom!("femergenode")
	Femorphology,        // atom!("femorphology")
	Feoffset,            // atom!("feoffset")
	Fepointlight,        // atom!("fepointlight")
	Fespecularlighting,  // atom!("fespecularlighting")
	Fespotlight,         // atom!("fespotlight")
	Fetile,              // atom!("fetile")
	Feturbulence,        // atom!("feturbulence")
	Filter,              // atom!("filter")
	Font,                // atom!("font")
	FontFace,            // atom!("font-face")
	FontFaceFormat,      // atom!("font-face-format")
	FontFaceName,        // atom!("font-face-name")
	FontFaceSrc,         // atom!("font-face-src")
	FontFaceUri,         // atom!("font-face-uri")
	Foreignobject,       // atom!("foreignobject")
	G,                   // atom!("g")
	Glyph,               // atom!("glyph")
	Glyphref,            // atom!("glyphref")
	Hkern,               // atom!("hkern")
	Image,               // atom!("image")
	Line,                // atom!("line")
	LinearGradient,      // atom!("linearGradient")
	Marker,              // atom!("marker")
	Mask,                // atom!("mask")
	Metadata,            // atom!("metadata")
	MissingGlyph,        // atom!("missing-glyph")
	Mpath,               // atom!("mpath")
	Path,                // atom!("path")
	Pattern,             // atom!("pattern")
	Polygon,             // atom!("polygon")
	Polyline,            // atom!("polyline")
	RadialGradient,      // atom!("radialGradient")
	Rect,                // atom!("rect")
	Script,              // atom!("script")
	Set,                 // atom!("set")
	Stop,                // atom!("stop")
	Style,               // atom!("style")
	Svg,                 // atom!("svg")
	Switch,              // atom!("switch")
	Symbol,              // atom!("symbol")
	Text,                // atom!("text")
	Textpath,            // atom!("textpath")
	Title,               // atom!("title")
	Tref,                // atom!("tref")
	Tspan,               // atom!("tspan")
	Use,                 // atom!("use")
	View,                // atom!("view")
	Vkern,               // atom!("vkern")
}

#[derive(Atomizable, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum MathmlTag {
	Maction,       // atom!("maction")
	Math,          // atom!("math")
	Menclose,      // atom!("menclose")
	Merror,        // atom!("merror")
	Mfenced,       // atom!("mfenced")
	Mfrac,         // atom!("mfrac")
	Mi,            // atom!("mi")
	Mmultiscripts, // atom!("mmultiscripts")
	Mn,            // atom!("mn")
	Mo,            // atom!("mo")
	Mover,         // atom!("mover")
	Mpadded,       // atom!("mpadded")
	Mphantom,      // atom!("mphantom")
	Mroot,         // atom!("mroot")
	Mrow,          // atom!("mrow")
	Ms,            // atom!("ms")
	Mspace,        // atom!("mspace")
	Msqrt,         // atom!("msqrt")
	Mstyle,        // atom!("mstyle")
	Msub,          // atom!("msub")
	Msubsup,       // atom!("msubsup")
	Msup,          // atom!("msup")
	Mtable,        // atom!("mtable")
	Mtd,           // atom!("mtd")
	Mtext,         // atom!("mtext")
	Mtr,           // atom!("mtr")
	Munder,        // atom!("munder")
	Munderover,    // atom!("munderover")
	Semantics,     // atom!("semantics")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Tag, 16);
		assert_size!(HtmlTag, 1);
		assert_size!(SvgTag, 1);
		assert_size!(MathmlTag, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Tag, "div");
	}
}
