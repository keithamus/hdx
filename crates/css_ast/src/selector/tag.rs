use css_lexer::{Cursor, Span};
use css_parse::{keyword_set, Build, Parser, Peek, T};
use hdx_proc_macro::visit;

use crate::{Visit, Visitable};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum Tag {
	Html(HtmlTag),
	HtmlNonConforming(HtmlNonConformingTag),
	HtmlNonStandard(HtmlNonStandardTag),
	Svg(SvgTag),
	Mathml(MathmlTag),
	CustomElement(CustomElementTag),
	Unknown(UnknownTag),
}

impl<'a> Peek<'a> for Tag {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c)
	}
}

impl<'a> Build<'a> for Tag {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if HtmlTag::peek(p, c) {
			Self::Html(HtmlTag::build(p, c))
		} else if SvgTag::peek(p, c) {
			Self::Svg(SvgTag::build(p, c))
		} else if MathmlTag::peek(p, c) {
			Self::Mathml(MathmlTag::build(p, c))
		} else if CustomElementTag::peek(p, c) {
			Self::CustomElement(CustomElementTag::build(p, c))
		} else if HtmlNonConformingTag::peek(p, c) {
			Self::HtmlNonConforming(HtmlNonConformingTag::build(p, c))
		} else if HtmlNonStandardTag::peek(p, c) {
			Self::HtmlNonStandard(HtmlNonStandardTag::build(p, c))
		} else {
			Self::Unknown(UnknownTag::build(p, c))
		}
	}
}

impl From<Tag> for Cursor {
	fn from(value: Tag) -> Self {
		match value {
			Tag::Html(c) => c.into(),
			Tag::HtmlNonConforming(c) => c.into(),
			Tag::HtmlNonStandard(c) => c.into(),
			Tag::Svg(c) => c.into(),
			Tag::Mathml(c) => c.into(),
			Tag::CustomElement(c) => c.into(),
			Tag::Unknown(c) => c.into(),
		}
	}
}

impl From<Tag> for Span {
	fn from(value: Tag) -> Self {
		let c: Cursor = value.into();
		c.into()
	}
}

impl<'a> Visitable<'a> for Tag {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_tag(self);
		match self {
			Self::Html(c) => Visitable::accept(c, v),
			Self::HtmlNonConforming(c) => Visitable::accept(c, v),
			Self::HtmlNonStandard(c) => Visitable::accept(c, v),
			Self::Svg(c) => Visitable::accept(c, v),
			Self::Mathml(c) => Visitable::accept(c, v),
			Self::CustomElement(c) => Visitable::accept(c, v),
			Self::Unknown(c) => Visitable::accept(c, v),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CustomElementTag(T![Ident]);

impl CustomElementTag {
	const INVALID: phf::Map<&'static str, bool> = phf::phf_map! {
		"annotation-xml" => true,
		"color-profile" => true,
		"font-face" => true,
		"font-face-src" => true,
		"font-face-uri" => true,
		"font-face-format" => true,
		"font-face-name" => true,
		"missing-glyph" => true,
	};
}

impl<'a> Peek<'a> for CustomElementTag {
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		let str = p.parse_str_lower(c);
		if *Self::INVALID.get(str).unwrap_or(&false) {
			return false;
		}
		let mut chars = str.chars();
		if !matches!(chars.next(), Some('a'..='z')) {
			return false;
		}
		let mut has_dash = false;
		for char in chars {
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
				return false;
			}
		}
		has_dash
	}
}

impl<'a> Build<'a> for CustomElementTag {
	fn build(p: &Parser<'a>, c: css_lexer::Cursor) -> Self {
		Self(<T![Ident]>::build(p, c))
	}
}

impl From<CustomElementTag> for Cursor {
	fn from(value: CustomElementTag) -> Self {
		value.0.into()
	}
}

impl From<CustomElementTag> for Span {
	fn from(value: CustomElementTag) -> Self {
		let c: Cursor = value.into();
		c.into()
	}
}

impl<'a> Visitable<'a> for CustomElementTag {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_custom_element_tag(self);
	}
}

// https://html.spec.whatwg.org/multipage/indices.html#elements-3
#[visit]
keyword_set!(HtmlTag {
	A: "a",
	Abbr: "abbr",
	Address: "address",
	Area: "area",
	Article: "article",
	Aside: "aside",
	Audio: "audio",
	B: "b",
	Base: "base",
	Bdi: "bdi",
	Bdo: "bdo",
	Big: "big",
	Blockquote: "blockquote",
	Body: "body",
	Br: "br",
	Button: "button",
	Canvas: "canvas",
	Caption: "caption",
	Center: "center",
	Cite: "cite",
	Code: "code",
	Col: "col",
	Colgroup: "colgroup",
	Data: "data",
	Datalist: "datalist",
	Dd: "dd",
	Del: "del",
	Details: "details",
	Dfn: "dfn",
	Dialog: "dialog",
	Dir: "dir",
	Div: "div",
	Dl: "dl",
	Dt: "dt",
	Em: "em",
	Embed: "embed",
	Fieldset: "fieldset",
	Figcaption: "figcaption",
	Figure: "figure",
	Font: "font",
	Footer: "footer",
	Form: "form",
	Frame: "frame",
	Frameset: "frameset",
	H1: "h1",
	H2: "h2",
	H3: "h3",
	H4: "h4",
	H5: "h5",
	H6: "h6",
	Head: "head",
	Header: "header",
	Hgroup: "hgroup",
	Hr: "hr",
	Html: "html",
	I: "i",
	Iframe: "iframe",
	Img: "img",
	Input: "input",
	Ins: "ins",
	Kbd: "kbd",
	Label: "label",
	Legend: "legend",
	Li: "li",
	Link: "link",
	Main: "main",
	Map: "map",
	Mark: "mark",
	Marquee: "marquee",
	Menu: "menu",
	Menuitem: "menuitem",
	Meta: "meta",
	Meter: "meter",
	Nav: "nav",
	Nobr: "nobr",
	Noembed: "noembed",
	Noframes: "noframes",
	Noscript: "noscript",
	Object: "object",
	Ol: "ol",
	Optgroup: "optgroup",
	Option: "option",
	Output: "output",
	P: "p",
	Param: "param",
	Picture: "picture",
	Plaintext: "plaintext",
	Pre: "pre",
	Progress: "progress",
	Q: "q",
	Rb: "rb",
	Rp: "rp",
	Rt: "rt",
	Rtc: "rtc",
	Ruby: "ruby",
	S: "s",
	Samp: "samp",
	Script: "script",
	Search: "search",
	Section: "section",
	Select: "select",
	Slot: "slot",
	Small: "small",
	Source: "source",
	Span: "span",
	Strike: "strike",
	Strong: "strong",
	Style: "style",
	Sub: "sub",
	Summary: "summary",
	Sup: "sup",
	Table: "table",
	Tbody: "tbody",
	Td: "td",
	Template: "template",
	Textarea: "textarea",
	Tfoot: "tfoot",
	Th: "th",
	Thead: "thead",
	Time: "time",
	Title: "title",
	Tr: "tr",
	Track: "track",
	Tt: "tt",
	U: "u",
	Ul: "ul",
	Var: "var",
	Video: "video",
	Wbr: "wbr",
	Xmp: "xmp",
});

impl<'a> Visitable<'a> for HtmlTag {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_html_tag(self);
	}
}

// https://html.spec.whatwg.org/multipage/obsolete.html#non-conforming-features
#[visit]
keyword_set!(HtmlNonConformingTag {
	Acronym: "acronym",
	Applet: "applet",
	Basefont: "basefont",
	Bgsound: "bgsound",
	Big: "big",
	Blink: "blink",
	Center: "center",
	Dir: "dir",
	Font: "font",
	Frame: "frame",
	Frameset: "frameset",
	Isindex: "isindex",
	Keygen: "keygen",
	Listing: "listing",
	Marquee: "marquee",
	Menuitem: "menuitem",
	Multicol: "multicol",
	Nextid: "nextid",
	Nobr: "nobr",
	Noembed: "noembed",
	Noframes: "noframes",
	Param: "param",
	Plaintext: "plaintext",
	Rb: "rb",
	Rtc: "rtc",
	Spacer: "spacer",
	Strike: "strike",
	Tt: "tt",
	Xmp: "xmp",
});

impl<'a> Visitable<'a> for HtmlNonConformingTag {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_html_non_conforming_tag(self);
	}
}

#[visit]
keyword_set!(HtmlNonStandardTag {
	// https://wicg.github.io/fenced-frame/#the-fencedframe-element
	Fencedframe: "fencedframe",
	// https://wicg.github.io/portals/#the-portal-element
	Portal: "portal",
	// https://wicg.github.io/PEPC/permission-element.html#the-permission-element
	Permission: "permission",
	// https://open-ui.org/components/customizableselect/
	Selectedcontent: "selectedcontent",
});

impl<'a> Visitable<'a> for HtmlNonStandardTag {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_html_non_standard_tag(self);
	}
}

// https://svgwg.org/svg2-draft/eltindex.html
#[visit]
keyword_set!(SvgTag {
	A: "a",
	Animate: "animate",
	Animatemotion: "animatemotion",
	Animatetransform: "animatetransform",
	Circle: "circle",
	Clippath: "clippath",
	Defs: "defs",
	Desc: "desc",
	Discard: "discard",
	Ellipse: "ellipse",
	Feblend: "feblend",
	Fecolormatrix: "fecolormatrix",
	Fecomponenttransfer: "fecomponenttransfer",
	Fecomposite: "fecomposite",
	Feconvolvematrix: "feconvolvematrix",
	Fediffuselighting: "fediffuselighting",
	Fedisplacementmap: "fedisplacementmap",
	Fedistantlight: "fedistantlight",
	Fedropshadow: "fedropshadow",
	Feflood: "feflood",
	Fefunca: "fefunca",
	Fefuncb: "fefuncb",
	Fefuncg: "fefuncg",
	Fefuncr: "fefuncr",
	Fegaussianblur: "fegaussianblur",
	Feimage: "feimage",
	Femerge: "femerge",
	Femergenode: "femergenode",
	Femorphology: "femorphology",
	Feoffset: "feoffset",
	Fepointlight: "fepointlight",
	Fespecularlighting: "fespecularlighting",
	Fespotlight: "fespotlight",
	Fetile: "fetile",
	Feturbulence: "feturbulence",
	Filter: "filter",
	Foreignobject: "foreignobject",
	G: "g",
	Image: "image",
	Line: "line",
	Lineargradient: "lineargradient",
	Marker: "marker",
	Mask: "mask",
	Metadata: "metadata",
	Mpath: "mpath",
	Path: "path",
	Pattern: "pattern",
	Polygon: "polygon",
	Polyline: "polyline",
	Radialgradient: "radialgradient",
	Rect: "rect",
	Script: "script",
	Set: "set",
	Stop: "stop",
	Style: "style",
	Svg: "svg",
	Switch: "switch",
	Symbol: "symbol",
	Text: "text",
	Textpath: "textpath",
	Title: "title",
	Tspan: "tspan",
	Use: "use",
	View: "view",
});

impl<'a> Visitable<'a> for SvgTag {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_svg_tag(self);
	}
}

// https://w3c.github.io/mathml/#mmlindex_elements
#[visit]
keyword_set!(MathmlTag {
	Abs: "abs",
	And: "and",
	Annotation: "annotation",
	AnnotationXml: "annotation-xml",
	Apply: "apply",
	Approx: "approx",
	Arg: "arg",
	Bind: "bind",
	Bvar: "bvar",
	Card: "card",
	Cartesianproduct: "cartesianproduct",
	Cbytes: "cbytes",
	Ceiling: "ceiling",
	Cerror: "cerror",
	Ci: "ci",
	Cn: "cn",
	Codomain: "codomain",
	Compose: "compose",
	Condition: "condition",
	Conjugate: "conjugate",
	Cs: "cs",
	Csymbol: "csymbol",
	Curl: "curl",
	Declare: "declare",
	Degree: "degree",
	Determinant: "determinant",
	Diff: "diff",
	Divergence: "divergence",
	Divide: "divide",
	Domain: "domain",
	Domainofapplication: "domainofapplication",
	Emptyset: "emptyset",
	Eq: "eq",
	Equivalent: "equivalent",
	Exists: "exists",
	Exp: "exp",
	Factorial: "factorial",
	Factorof: "factorof",
	Floor: "floor",
	Fn: "fn",
	Forall: "forall",
	Gcd: "gcd",
	Geq: "geq",
	Grad: "grad",
	Gt: "gt",
	Ident: "ident",
	Image: "image",
	Imaginary: "imaginary",
	Img: "img",
	Implies: "implies",
	In: "in",
	Int: "int",
	Intersect: "intersect",
	Interval: "interval",
	Inverse: "inverse",
	Lambda: "lambda",
	Laplacian: "laplacian",
	Lcm: "lcm",
	Leq: "leq",
	Limit: "limit",
	List: "list",
	Ln: "ln",
	Log: "log",
	Logbase: "logbase",
	Lowlimit: "lowlimit",
	Lt: "lt",
	Maction: "maction",
	Maligngroup: "maligngroup",
	Malignmark: "malignmark",
	Math: "math",
	Matrix: "matrix",
	Matrixrow: "matrixrow",
	Max: "max",
	Mean: "mean",
	Median: "median",
	Menclose: "menclose",
	Merror: "merror",
	Mfenced: "mfenced",
	Mfrac: "mfrac",
	Mfraction: "mfraction",
	Mglyph: "mglyph",
	Mi: "mi",
	Min: "min",
	Minus: "minus",
	Mlabeledtr: "mlabeledtr",
	Mlongdiv: "mlongdiv",
	Mmultiscripts: "mmultiscripts",
	Mn: "mn",
	Mo: "mo",
	Mode: "mode",
	Moment: "moment",
	Momentabout: "momentabout",
	Mover: "mover",
	Mpadded: "mpadded",
	Mphantom: "mphantom",
	Mprescripts: "mprescripts",
	Mroot: "mroot",
	Mrow: "mrow",
	Ms: "ms",
	Mscarries: "mscarries",
	Mscarry: "mscarry",
	Msgroup: "msgroup",
	Msline: "msline",
	Mspace: "mspace",
	Msqrt: "msqrt",
	Msrow: "msrow",
	Mstack: "mstack",
	Mstyle: "mstyle",
	Msub: "msub",
	Msubsup: "msubsup",
	Msup: "msup",
	Mtable: "mtable",
	Mtd: "mtd",
	Mtext: "mtext",
	Mtr: "mtr",
	Munder: "munder",
	Munderover: "munderover",
	Neq: "neq",
	None: "none",
	Not: "not",
	Notin: "notin",
	Notprsubset: "notprsubset",
	Notsubset: "notsubset",
	Or: "or",
	Otherwise: "otherwise",
	Outerproduct: "outerproduct",
	Partialdiff: "partialdiff",
	Piece: "piece",
	Piecewise: "piecewise",
	Plus: "plus",
	Power: "power",
	Product: "product",
	Prsubset: "prsubset",
	Quotient: "quotient",
	Real: "real",
	Reln: "reln",
	Rem: "rem",
	Root: "root",
	Scalarproduct: "scalarproduct",
	Sdev: "sdev",
	Selector: "selector",
	Semantics: "semantics",
	Sep: "sep",
	Set: "set",
	Setdiff: "setdiff",
	Share: "share",
	Sin: "sin",
	Subset: "subset",
	Sum: "sum",
	Tendsto: "tendsto",
	Times: "times",
	Transpose: "transpose",
	Union: "union",
	Uplimit: "uplimit",
	Variance: "variance",
	Vector: "vector",
	Vectorproduct: "vectorproduct",
	Xo: "xo",
});

impl<'a> Visitable<'a> for MathmlTag {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_mathml_tag(self);
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct UnknownTag(T![Ident]);

impl<'a> Peek<'a> for UnknownTag {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c)
	}
}

impl<'a> Build<'a> for UnknownTag {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![Ident]>::build(p, c))
	}
}

impl From<UnknownTag> for Span {
	fn from(value: UnknownTag) -> Self {
		let c: Cursor = value.into();
		c.into()
	}
}

impl From<UnknownTag> for Cursor {
	fn from(value: UnknownTag) -> Self {
		value.0.into()
	}
}

impl<'a> Visitable<'a> for UnknownTag {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_unknown_tag(self);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Tag>(), 20);
		assert_eq!(std::mem::size_of::<HtmlTag>(), 16);
		assert_eq!(std::mem::size_of::<SvgTag>(), 16);
		assert_eq!(std::mem::size_of::<MathmlTag>(), 16);
		assert_eq!(std::mem::size_of::<CustomElementTag>(), 12);
		assert_eq!(std::mem::size_of::<HtmlNonConformingTag>(), 16);
		assert_eq!(std::mem::size_of::<HtmlNonStandardTag>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Tag, "div");
	}
}
