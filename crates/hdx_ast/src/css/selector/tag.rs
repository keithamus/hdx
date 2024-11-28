use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Tag {
	Html(HtmlTag),
	HtmlNonConforming(HtmlNonConformingTag),
	HtmlNonStandard(HtmlNonStandardTag),
	Svg(SvgTag),
	Mathml(MathmlTag),
	CustomElement(CustomElementTag),
	Unknown(T![Ident]),
}

impl<'a> Is<'a> for Tag {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::is(p, c)
	}
}

impl<'a> Build<'a> for Tag {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if HtmlTag::is(p, c) {
			Self::Html(HtmlTag::build(p, c))
		} else if SvgTag::is(p, c) {
			Self::Svg(SvgTag::build(p, c))
		} else if MathmlTag::is(p, c) {
			Self::Mathml(MathmlTag::build(p, c))
		} else if CustomElementTag::is(p, c) {
			Self::CustomElement(CustomElementTag::build(p, c))
		} else if HtmlNonConformingTag::is(p, c) {
			Self::HtmlNonConforming(HtmlNonConformingTag::build(p, c))
		} else if HtmlNonStandardTag::is(p, c) {
			Self::HtmlNonStandard(HtmlNonStandardTag::build(p, c))
		} else {
			Self::Unknown(<T![Ident]>::build(p, c))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CustomElementTag(T![Ident]);

impl<'a> Is<'a> for CustomElementTag {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		let atom = p.parse_atom_lower(c);
		if matches!(
			atom,
			atom!("annotation-xml")
				| atom!("color-profile")
				| atom!("font-face")
				| atom!("font-face-src")
				| atom!("font-face-uri")
				| atom!("font-face-format")
				| atom!("font-face-name")
				| atom!("missing-glyph")
		) {
			return false;
		}
		let mut chars = atom.chars();
		if !matches!(chars.next(), Some('a'..='z')) {
			return false;
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
				return false;
			}
		}
		return has_dash;
	}
}

impl<'a> Build<'a> for CustomElementTag {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		Self(<T![Ident]>::build(p, c))
	}
}

impl From<CustomElementTag> for Cursor {
	fn from(value: CustomElementTag) -> Self {
		value.0.into()
	}
}

// https://html.spec.whatwg.org/multipage/indices.html#elements-3
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum HtmlTag {
	A(T![Ident]),
	Abbr(T![Ident]),
	Address(T![Ident]),
	Area(T![Ident]),
	Article(T![Ident]),
	Aside(T![Ident]),
	Audio(T![Ident]),
	B(T![Ident]),
	Base(T![Ident]),
	Bdi(T![Ident]),
	Bdo(T![Ident]),
	Big(T![Ident]),
	Blockquote(T![Ident]),
	Body(T![Ident]),
	Br(T![Ident]),
	Button(T![Ident]),
	Canvas(T![Ident]),
	Caption(T![Ident]),
	Center(T![Ident]),
	Cite(T![Ident]),
	Code(T![Ident]),
	Col(T![Ident]),
	Colgroup(T![Ident]),
	Data(T![Ident]),
	Datalist(T![Ident]),
	Dd(T![Ident]),
	Del(T![Ident]),
	Details(T![Ident]),
	Dfn(T![Ident]),
	Dialog(T![Ident]),
	Dir(T![Ident]),
	Div(T![Ident]),
	Dl(T![Ident]),
	Dt(T![Ident]),
	Em(T![Ident]),
	Embed(T![Ident]),
	Fencedframe(T![Ident]),
	Fieldset(T![Ident]),
	Figcaption(T![Ident]),
	Figure(T![Ident]),
	Font(T![Ident]),
	Footer(T![Ident]),
	Form(T![Ident]),
	Frame(T![Ident]),
	Frameset(T![Ident]),
	H1(T![Ident]),
	H2(T![Ident]),
	H3(T![Ident]),
	H4(T![Ident]),
	H5(T![Ident]),
	H6(T![Ident]),
	Head(T![Ident]),
	Header(T![Ident]),
	Hgroup(T![Ident]),
	Hr(T![Ident]),
	Html(T![Ident]),
	I(T![Ident]),
	Iframe(T![Ident]),
	Img(T![Ident]),
	Input(T![Ident]),
	Ins(T![Ident]),
	Kbd(T![Ident]),
	Label(T![Ident]),
	Legend(T![Ident]),
	Li(T![Ident]),
	Link(T![Ident]),
	Main(T![Ident]),
	Map(T![Ident]),
	Mark(T![Ident]),
	Marquee(T![Ident]),
	Menu(T![Ident]),
	Menuitem(T![Ident]),
	Meta(T![Ident]),
	Meter(T![Ident]),
	Nav(T![Ident]),
	Nobr(T![Ident]),
	Noembed(T![Ident]),
	Noframes(T![Ident]),
	Noscript(T![Ident]),
	Object(T![Ident]),
	Ol(T![Ident]),
	Optgroup(T![Ident]),
	Option(T![Ident]),
	Output(T![Ident]),
	P(T![Ident]),
	Param(T![Ident]),
	Picture(T![Ident]),
	Plaintext(T![Ident]),
	Portal(T![Ident]),
	Pre(T![Ident]),
	Progress(T![Ident]),
	Q(T![Ident]),
	Rb(T![Ident]),
	Rp(T![Ident]),
	Rt(T![Ident]),
	Rtc(T![Ident]),
	Ruby(T![Ident]),
	S(T![Ident]),
	Samp(T![Ident]),
	Script(T![Ident]),
	Search(T![Ident]),
	Section(T![Ident]),
	Select(T![Ident]),
	Slot(T![Ident]),
	Small(T![Ident]),
	Source(T![Ident]),
	Span(T![Ident]),
	Strike(T![Ident]),
	Strong(T![Ident]),
	Style(T![Ident]),
	Sub(T![Ident]),
	Summary(T![Ident]),
	Sup(T![Ident]),
	Table(T![Ident]),
	Tbody(T![Ident]),
	Td(T![Ident]),
	Template(T![Ident]),
	Textarea(T![Ident]),
	Tfoot(T![Ident]),
	Th(T![Ident]),
	Thead(T![Ident]),
	Time(T![Ident]),
	Title(T![Ident]),
	Tr(T![Ident]),
	Track(T![Ident]),
	Tt(T![Ident]),
	U(T![Ident]),
	Ul(T![Ident]),
	Var(T![Ident]),
	Video(T![Ident]),
	Wbr(T![Ident]),
	Xmp(T![Ident]),
}

impl<'a> Is<'a> for HtmlTag {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		matches!(
			p.parse_atom_lower(c),
			atom!("a")
				| atom!("abbr")
				| atom!("address")
				| atom!("area")
				| atom!("article")
				| atom!("aside")
				| atom!("audio")
				| atom!("b") | atom!("base")
				| atom!("bdi")
				| atom!("bdo")
				| atom!("blockquote")
				| atom!("body")
				| atom!("br")
				| atom!("button")
				| atom!("canvas")
				| atom!("caption")
				| atom!("cite")
				| atom!("code")
				| atom!("col")
				| atom!("colgroup")
				| atom!("data")
				| atom!("datalist")
				| atom!("dd")
				| atom!("del")
				| atom!("details")
				| atom!("dfn")
				| atom!("dialog")
				| atom!("div")
				| atom!("dl")
				| atom!("dt")
				| atom!("em")
				| atom!("embed")
				| atom!("fieldset")
				| atom!("figcaption")
				| atom!("figure")
				| atom!("footer")
				| atom!("form")
				| atom!("h1")
				| atom!("h2")
				| atom!("h3")
				| atom!("h4")
				| atom!("h5")
				| atom!("h6")
				| atom!("head")
				| atom!("header")
				| atom!("hgroup")
				| atom!("hr")
				| atom!("html")
				| atom!("i") | atom!("iframe")
				| atom!("img")
				| atom!("input")
				| atom!("ins")
				| atom!("kbd")
				| atom!("label")
				| atom!("legend")
				| atom!("li")
				| atom!("link")
				| atom!("main")
				| atom!("map")
				| atom!("mark")
				| atom!("menu")
				| atom!("meta")
				| atom!("meter")
				| atom!("nav")
				| atom!("noscript")
				| atom!("object")
				| atom!("ol")
				| atom!("optgroup")
				| atom!("option")
				| atom!("output")
				| atom!("p") | atom!("picture")
				| atom!("pre")
				| atom!("progress")
				| atom!("q") | atom!("rp")
				| atom!("rt")
				| atom!("ruby")
				| atom!("s") | atom!("samp")
				| atom!("script")
				| atom!("search")
				| atom!("section")
				| atom!("select")
				| atom!("slot")
				| atom!("small")
				| atom!("source")
				| atom!("span")
				| atom!("strong")
				| atom!("style")
				| atom!("sub")
				| atom!("summary")
				| atom!("sup")
				| atom!("table")
				| atom!("tbody")
				| atom!("td")
				| atom!("template")
				| atom!("textarea")
				| atom!("tfoot")
				| atom!("th")
				| atom!("thead")
				| atom!("time")
				| atom!("title")
				| atom!("tr")
				| atom!("track")
				| atom!("u") | atom!("ul")
				| atom!("var")
				| atom!("video")
				| atom!("wbr")
		)
	}
}

impl<'a> Build<'a> for HtmlTag {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("a") => Self::A(<T![Ident]>::build(p, c)),
			atom!("abbr") => Self::Abbr(<T![Ident]>::build(p, c)),
			atom!("address") => Self::Address(<T![Ident]>::build(p, c)),
			atom!("area") => Self::Area(<T![Ident]>::build(p, c)),
			atom!("article") => Self::Article(<T![Ident]>::build(p, c)),
			atom!("aside") => Self::Aside(<T![Ident]>::build(p, c)),
			atom!("audio") => Self::Audio(<T![Ident]>::build(p, c)),
			atom!("b") => Self::B(<T![Ident]>::build(p, c)),
			atom!("base") => Self::Base(<T![Ident]>::build(p, c)),
			atom!("bdi") => Self::Bdi(<T![Ident]>::build(p, c)),
			atom!("bdo") => Self::Bdo(<T![Ident]>::build(p, c)),
			atom!("blockquote") => Self::Blockquote(<T![Ident]>::build(p, c)),
			atom!("body") => Self::Body(<T![Ident]>::build(p, c)),
			atom!("br") => Self::Br(<T![Ident]>::build(p, c)),
			atom!("button") => Self::Button(<T![Ident]>::build(p, c)),
			atom!("canvas") => Self::Canvas(<T![Ident]>::build(p, c)),
			atom!("caption") => Self::Caption(<T![Ident]>::build(p, c)),
			atom!("center") => Self::Center(<T![Ident]>::build(p, c)),
			atom!("cite") => Self::Cite(<T![Ident]>::build(p, c)),
			atom!("code") => Self::Code(<T![Ident]>::build(p, c)),
			atom!("col") => Self::Col(<T![Ident]>::build(p, c)),
			atom!("colgroup") => Self::Colgroup(<T![Ident]>::build(p, c)),
			atom!("data") => Self::Data(<T![Ident]>::build(p, c)),
			atom!("datalist") => Self::Datalist(<T![Ident]>::build(p, c)),
			atom!("dd") => Self::Dd(<T![Ident]>::build(p, c)),
			atom!("del") => Self::Del(<T![Ident]>::build(p, c)),
			atom!("details") => Self::Details(<T![Ident]>::build(p, c)),
			atom!("dfn") => Self::Dfn(<T![Ident]>::build(p, c)),
			atom!("dialog") => Self::Dialog(<T![Ident]>::build(p, c)),
			atom!("div") => Self::Div(<T![Ident]>::build(p, c)),
			atom!("dl") => Self::Dl(<T![Ident]>::build(p, c)),
			atom!("dt") => Self::Dt(<T![Ident]>::build(p, c)),
			atom!("em") => Self::Em(<T![Ident]>::build(p, c)),
			atom!("embed") => Self::Embed(<T![Ident]>::build(p, c)),
			atom!("fieldset") => Self::Fieldset(<T![Ident]>::build(p, c)),
			atom!("figcaption") => Self::Figcaption(<T![Ident]>::build(p, c)),
			atom!("figure") => Self::Figure(<T![Ident]>::build(p, c)),
			atom!("footer") => Self::Footer(<T![Ident]>::build(p, c)),
			atom!("form") => Self::Form(<T![Ident]>::build(p, c)),
			atom!("h1") => Self::H1(<T![Ident]>::build(p, c)),
			atom!("h2") => Self::H2(<T![Ident]>::build(p, c)),
			atom!("h3") => Self::H3(<T![Ident]>::build(p, c)),
			atom!("h4") => Self::H4(<T![Ident]>::build(p, c)),
			atom!("h5") => Self::H5(<T![Ident]>::build(p, c)),
			atom!("h6") => Self::H6(<T![Ident]>::build(p, c)),
			atom!("head") => Self::Head(<T![Ident]>::build(p, c)),
			atom!("header") => Self::Header(<T![Ident]>::build(p, c)),
			atom!("hgroup") => Self::Hgroup(<T![Ident]>::build(p, c)),
			atom!("hr") => Self::Hr(<T![Ident]>::build(p, c)),
			atom!("html") => Self::Html(<T![Ident]>::build(p, c)),
			atom!("i") => Self::I(<T![Ident]>::build(p, c)),
			atom!("iframe") => Self::Iframe(<T![Ident]>::build(p, c)),
			atom!("img") => Self::Img(<T![Ident]>::build(p, c)),
			atom!("input") => Self::Input(<T![Ident]>::build(p, c)),
			atom!("ins") => Self::Ins(<T![Ident]>::build(p, c)),
			atom!("kbd") => Self::Kbd(<T![Ident]>::build(p, c)),
			atom!("label") => Self::Label(<T![Ident]>::build(p, c)),
			atom!("legend") => Self::Legend(<T![Ident]>::build(p, c)),
			atom!("li") => Self::Li(<T![Ident]>::build(p, c)),
			atom!("link") => Self::Link(<T![Ident]>::build(p, c)),
			atom!("main") => Self::Main(<T![Ident]>::build(p, c)),
			atom!("map") => Self::Map(<T![Ident]>::build(p, c)),
			atom!("mark") => Self::Mark(<T![Ident]>::build(p, c)),
			atom!("menu") => Self::Menu(<T![Ident]>::build(p, c)),
			atom!("meta") => Self::Meta(<T![Ident]>::build(p, c)),
			atom!("meter") => Self::Meter(<T![Ident]>::build(p, c)),
			atom!("nav") => Self::Nav(<T![Ident]>::build(p, c)),
			atom!("noscript") => Self::Noscript(<T![Ident]>::build(p, c)),
			atom!("object") => Self::Object(<T![Ident]>::build(p, c)),
			atom!("ol") => Self::Ol(<T![Ident]>::build(p, c)),
			atom!("optgroup") => Self::Optgroup(<T![Ident]>::build(p, c)),
			atom!("option") => Self::Option(<T![Ident]>::build(p, c)),
			atom!("output") => Self::Output(<T![Ident]>::build(p, c)),
			atom!("p") => Self::P(<T![Ident]>::build(p, c)),
			atom!("picture") => Self::Picture(<T![Ident]>::build(p, c)),
			atom!("pre") => Self::Pre(<T![Ident]>::build(p, c)),
			atom!("progress") => Self::Progress(<T![Ident]>::build(p, c)),
			atom!("q") => Self::Q(<T![Ident]>::build(p, c)),
			atom!("rp") => Self::Rp(<T![Ident]>::build(p, c)),
			atom!("rt") => Self::Rt(<T![Ident]>::build(p, c)),
			atom!("ruby") => Self::Ruby(<T![Ident]>::build(p, c)),
			atom!("s") => Self::S(<T![Ident]>::build(p, c)),
			atom!("samp") => Self::Samp(<T![Ident]>::build(p, c)),
			atom!("script") => Self::Script(<T![Ident]>::build(p, c)),
			atom!("search") => Self::Search(<T![Ident]>::build(p, c)),
			atom!("section") => Self::Section(<T![Ident]>::build(p, c)),
			atom!("select") => Self::Select(<T![Ident]>::build(p, c)),
			atom!("slot") => Self::Slot(<T![Ident]>::build(p, c)),
			atom!("small") => Self::Small(<T![Ident]>::build(p, c)),
			atom!("source") => Self::Source(<T![Ident]>::build(p, c)),
			atom!("span") => Self::Span(<T![Ident]>::build(p, c)),
			atom!("strong") => Self::Strong(<T![Ident]>::build(p, c)),
			atom!("style") => Self::Style(<T![Ident]>::build(p, c)),
			atom!("sub") => Self::Sub(<T![Ident]>::build(p, c)),
			atom!("summary") => Self::Summary(<T![Ident]>::build(p, c)),
			atom!("sup") => Self::Sup(<T![Ident]>::build(p, c)),
			atom!("table") => Self::Table(<T![Ident]>::build(p, c)),
			atom!("tbody") => Self::Tbody(<T![Ident]>::build(p, c)),
			atom!("td") => Self::Td(<T![Ident]>::build(p, c)),
			atom!("template") => Self::Template(<T![Ident]>::build(p, c)),
			atom!("textarea") => Self::Textarea(<T![Ident]>::build(p, c)),
			atom!("tfoot") => Self::Tfoot(<T![Ident]>::build(p, c)),
			atom!("th") => Self::Th(<T![Ident]>::build(p, c)),
			atom!("thead") => Self::Thead(<T![Ident]>::build(p, c)),
			atom!("time") => Self::Time(<T![Ident]>::build(p, c)),
			atom!("title") => Self::Title(<T![Ident]>::build(p, c)),
			atom!("tr") => Self::Tr(<T![Ident]>::build(p, c)),
			atom!("track") => Self::Track(<T![Ident]>::build(p, c)),
			atom!("u") => Self::U(<T![Ident]>::build(p, c)),
			atom!("ul") => Self::Ul(<T![Ident]>::build(p, c)),
			atom!("var") => Self::Var(<T![Ident]>::build(p, c)),
			atom!("video") => Self::Video(<T![Ident]>::build(p, c)),
			atom!("wbr") => Self::Wbr(<T![Ident]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<HtmlTag> for Cursor {
	fn from(value: HtmlTag) -> Self {
		match value {
			HtmlTag::A(c) => c.into(),
			HtmlTag::Abbr(c) => c.into(),
			HtmlTag::Address(c) => c.into(),
			HtmlTag::Area(c) => c.into(),
			HtmlTag::Article(c) => c.into(),
			HtmlTag::Aside(c) => c.into(),
			HtmlTag::Audio(c) => c.into(),
			HtmlTag::B(c) => c.into(),
			HtmlTag::Base(c) => c.into(),
			HtmlTag::Bdi(c) => c.into(),
			HtmlTag::Bdo(c) => c.into(),
			HtmlTag::Big(c) => c.into(),
			HtmlTag::Blockquote(c) => c.into(),
			HtmlTag::Body(c) => c.into(),
			HtmlTag::Br(c) => c.into(),
			HtmlTag::Button(c) => c.into(),
			HtmlTag::Canvas(c) => c.into(),
			HtmlTag::Caption(c) => c.into(),
			HtmlTag::Center(c) => c.into(),
			HtmlTag::Cite(c) => c.into(),
			HtmlTag::Code(c) => c.into(),
			HtmlTag::Col(c) => c.into(),
			HtmlTag::Colgroup(c) => c.into(),
			HtmlTag::Data(c) => c.into(),
			HtmlTag::Datalist(c) => c.into(),
			HtmlTag::Dd(c) => c.into(),
			HtmlTag::Del(c) => c.into(),
			HtmlTag::Details(c) => c.into(),
			HtmlTag::Dfn(c) => c.into(),
			HtmlTag::Dialog(c) => c.into(),
			HtmlTag::Dir(c) => c.into(),
			HtmlTag::Div(c) => c.into(),
			HtmlTag::Dl(c) => c.into(),
			HtmlTag::Dt(c) => c.into(),
			HtmlTag::Em(c) => c.into(),
			HtmlTag::Embed(c) => c.into(),
			HtmlTag::Fencedframe(c) => c.into(),
			HtmlTag::Fieldset(c) => c.into(),
			HtmlTag::Figcaption(c) => c.into(),
			HtmlTag::Figure(c) => c.into(),
			HtmlTag::Font(c) => c.into(),
			HtmlTag::Footer(c) => c.into(),
			HtmlTag::Form(c) => c.into(),
			HtmlTag::Frame(c) => c.into(),
			HtmlTag::Frameset(c) => c.into(),
			HtmlTag::H1(c) => c.into(),
			HtmlTag::H2(c) => c.into(),
			HtmlTag::H3(c) => c.into(),
			HtmlTag::H4(c) => c.into(),
			HtmlTag::H5(c) => c.into(),
			HtmlTag::H6(c) => c.into(),
			HtmlTag::Head(c) => c.into(),
			HtmlTag::Header(c) => c.into(),
			HtmlTag::Hgroup(c) => c.into(),
			HtmlTag::Hr(c) => c.into(),
			HtmlTag::Html(c) => c.into(),
			HtmlTag::I(c) => c.into(),
			HtmlTag::Iframe(c) => c.into(),
			HtmlTag::Img(c) => c.into(),
			HtmlTag::Input(c) => c.into(),
			HtmlTag::Ins(c) => c.into(),
			HtmlTag::Kbd(c) => c.into(),
			HtmlTag::Label(c) => c.into(),
			HtmlTag::Legend(c) => c.into(),
			HtmlTag::Li(c) => c.into(),
			HtmlTag::Link(c) => c.into(),
			HtmlTag::Main(c) => c.into(),
			HtmlTag::Map(c) => c.into(),
			HtmlTag::Mark(c) => c.into(),
			HtmlTag::Marquee(c) => c.into(),
			HtmlTag::Menu(c) => c.into(),
			HtmlTag::Menuitem(c) => c.into(),
			HtmlTag::Meta(c) => c.into(),
			HtmlTag::Meter(c) => c.into(),
			HtmlTag::Nav(c) => c.into(),
			HtmlTag::Nobr(c) => c.into(),
			HtmlTag::Noembed(c) => c.into(),
			HtmlTag::Noframes(c) => c.into(),
			HtmlTag::Noscript(c) => c.into(),
			HtmlTag::Object(c) => c.into(),
			HtmlTag::Ol(c) => c.into(),
			HtmlTag::Optgroup(c) => c.into(),
			HtmlTag::Option(c) => c.into(),
			HtmlTag::Output(c) => c.into(),
			HtmlTag::P(c) => c.into(),
			HtmlTag::Param(c) => c.into(),
			HtmlTag::Picture(c) => c.into(),
			HtmlTag::Plaintext(c) => c.into(),
			HtmlTag::Portal(c) => c.into(),
			HtmlTag::Pre(c) => c.into(),
			HtmlTag::Progress(c) => c.into(),
			HtmlTag::Q(c) => c.into(),
			HtmlTag::Rb(c) => c.into(),
			HtmlTag::Rp(c) => c.into(),
			HtmlTag::Rt(c) => c.into(),
			HtmlTag::Rtc(c) => c.into(),
			HtmlTag::Ruby(c) => c.into(),
			HtmlTag::S(c) => c.into(),
			HtmlTag::Samp(c) => c.into(),
			HtmlTag::Script(c) => c.into(),
			HtmlTag::Search(c) => c.into(),
			HtmlTag::Section(c) => c.into(),
			HtmlTag::Select(c) => c.into(),
			HtmlTag::Slot(c) => c.into(),
			HtmlTag::Small(c) => c.into(),
			HtmlTag::Source(c) => c.into(),
			HtmlTag::Span(c) => c.into(),
			HtmlTag::Strike(c) => c.into(),
			HtmlTag::Strong(c) => c.into(),
			HtmlTag::Style(c) => c.into(),
			HtmlTag::Sub(c) => c.into(),
			HtmlTag::Summary(c) => c.into(),
			HtmlTag::Sup(c) => c.into(),
			HtmlTag::Table(c) => c.into(),
			HtmlTag::Tbody(c) => c.into(),
			HtmlTag::Td(c) => c.into(),
			HtmlTag::Template(c) => c.into(),
			HtmlTag::Textarea(c) => c.into(),
			HtmlTag::Tfoot(c) => c.into(),
			HtmlTag::Th(c) => c.into(),
			HtmlTag::Thead(c) => c.into(),
			HtmlTag::Time(c) => c.into(),
			HtmlTag::Title(c) => c.into(),
			HtmlTag::Tr(c) => c.into(),
			HtmlTag::Track(c) => c.into(),
			HtmlTag::Tt(c) => c.into(),
			HtmlTag::U(c) => c.into(),
			HtmlTag::Ul(c) => c.into(),
			HtmlTag::Var(c) => c.into(),
			HtmlTag::Video(c) => c.into(),
			HtmlTag::Wbr(c) => c.into(),
			HtmlTag::Xmp(c) => c.into(),
		}
	}
}

// https://html.spec.whatwg.org/multipage/obsolete.html#non-conforming-features
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum HtmlNonConformingTag {
	Acronym(T![Ident]),
	Applet(T![Ident]),
	Basefont(T![Ident]),
	Bgsound(T![Ident]),
	Big(T![Ident]),
	Blink(T![Ident]),
	Center(T![Ident]),
	Dir(T![Ident]),
	Font(T![Ident]),
	Frame(T![Ident]),
	Frameset(T![Ident]),
	Isindex(T![Ident]),
	Keygen(T![Ident]),
	Listing(T![Ident]),
	Marquee(T![Ident]),
	Menuitem(T![Ident]),
	Multicol(T![Ident]),
	Nextid(T![Ident]),
	Nobr(T![Ident]),
	Noembed(T![Ident]),
	Noframes(T![Ident]),
	Param(T![Ident]),
	Plaintext(T![Ident]),
	Rb(T![Ident]),
	Rtc(T![Ident]),
	Spacer(T![Ident]),
	Strike(T![Ident]),
	Tt(T![Ident]),
	Xmp(T![Ident]),
}

impl<'a> Is<'a> for HtmlNonConformingTag {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		matches!(
			p.parse_atom_lower(c),
			atom!("acronym")
				| atom!("big")
				| atom!("dir")
				| atom!("fencedframe")
				| atom!("font")
				| atom!("frame")
				| atom!("frameset")
				| atom!("marquee")
				| atom!("menuitem")
				| atom!("nobr")
				| atom!("noembed")
				| atom!("noframes")
				| atom!("param")
				| atom!("plaintext")
				| atom!("portal")
				| atom!("rb")
				| atom!("rtc")
				| atom!("strike")
				| atom!("tt")
				| atom!("xmp")
		)
	}
}

impl<'a> Build<'a> for HtmlNonConformingTag {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("acronym") => Self::Acronym(<T![Ident]>::build(p, c)),
			atom!("big") => Self::Big(<T![Ident]>::build(p, c)),
			atom!("dir") => Self::Dir(<T![Ident]>::build(p, c)),
			atom!("font") => Self::Font(<T![Ident]>::build(p, c)),
			atom!("frame") => Self::Frame(<T![Ident]>::build(p, c)),
			atom!("frameset") => Self::Frameset(<T![Ident]>::build(p, c)),
			atom!("marquee") => Self::Marquee(<T![Ident]>::build(p, c)),
			atom!("menuitem") => Self::Menuitem(<T![Ident]>::build(p, c)),
			atom!("nobr") => Self::Nobr(<T![Ident]>::build(p, c)),
			atom!("noembed") => Self::Noembed(<T![Ident]>::build(p, c)),
			atom!("noframes") => Self::Noframes(<T![Ident]>::build(p, c)),
			atom!("param") => Self::Param(<T![Ident]>::build(p, c)),
			atom!("plaintext") => Self::Plaintext(<T![Ident]>::build(p, c)),
			atom!("rb") => Self::Rb(<T![Ident]>::build(p, c)),
			atom!("rtc") => Self::Rtc(<T![Ident]>::build(p, c)),
			atom!("strike") => Self::Strike(<T![Ident]>::build(p, c)),
			atom!("tt") => Self::Tt(<T![Ident]>::build(p, c)),
			atom!("xmp") => Self::Xmp(<T![Ident]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<HtmlNonConformingTag> for Cursor {
	fn from(value: HtmlNonConformingTag) -> Self {
		match value {
			HtmlNonConformingTag::Acronym(c) => c.into(),
			HtmlNonConformingTag::Applet(c) => c.into(),
			HtmlNonConformingTag::Basefont(c) => c.into(),
			HtmlNonConformingTag::Bgsound(c) => c.into(),
			HtmlNonConformingTag::Big(c) => c.into(),
			HtmlNonConformingTag::Blink(c) => c.into(),
			HtmlNonConformingTag::Center(c) => c.into(),
			HtmlNonConformingTag::Dir(c) => c.into(),
			HtmlNonConformingTag::Font(c) => c.into(),
			HtmlNonConformingTag::Frame(c) => c.into(),
			HtmlNonConformingTag::Frameset(c) => c.into(),
			HtmlNonConformingTag::Isindex(c) => c.into(),
			HtmlNonConformingTag::Keygen(c) => c.into(),
			HtmlNonConformingTag::Listing(c) => c.into(),
			HtmlNonConformingTag::Marquee(c) => c.into(),
			HtmlNonConformingTag::Menuitem(c) => c.into(),
			HtmlNonConformingTag::Multicol(c) => c.into(),
			HtmlNonConformingTag::Nextid(c) => c.into(),
			HtmlNonConformingTag::Nobr(c) => c.into(),
			HtmlNonConformingTag::Noembed(c) => c.into(),
			HtmlNonConformingTag::Noframes(c) => c.into(),
			HtmlNonConformingTag::Param(c) => c.into(),
			HtmlNonConformingTag::Plaintext(c) => c.into(),
			HtmlNonConformingTag::Rb(c) => c.into(),
			HtmlNonConformingTag::Rtc(c) => c.into(),
			HtmlNonConformingTag::Spacer(c) => c.into(),
			HtmlNonConformingTag::Strike(c) => c.into(),
			HtmlNonConformingTag::Tt(c) => c.into(),
			HtmlNonConformingTag::Xmp(c) => c.into(),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum HtmlNonStandardTag {
	// https://wicg.github.io/fenced-frame/#the-fencedframe-element
	Fencedframe(T![Ident]),
	// https://wicg.github.io/portals/#the-portal-element
	Portal(T![Ident]),
	// https://wicg.github.io/PEPC/permission-element.html#the-permission-element
	Permission(T![Ident]),
	// https://open-ui.org/components/customizableselect/
	Selectedcontent(T![Ident]),
}

impl<'a> Is<'a> for HtmlNonStandardTag {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		matches!(
			p.parse_atom_lower(c),
			atom!("fencedframe") | atom!("portal") | atom!("permission") | atom!("selectedcontent")
		)
	}
}

impl<'a> Build<'a> for HtmlNonStandardTag {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("fencedframe") => Self::Fencedframe(<T![Ident]>::build(p, c)),
			atom!("portal") => Self::Portal(<T![Ident]>::build(p, c)),
			atom!("permission") => Self::Permission(<T![Ident]>::build(p, c)),
			atom!("selectedcontent") => Self::Selectedcontent(<T![Ident]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<HtmlNonStandardTag> for Cursor {
	fn from(value: HtmlNonStandardTag) -> Self {
		match value {
			HtmlNonStandardTag::Fencedframe(c) => c.into(),
			HtmlNonStandardTag::Portal(c) => c.into(),
			HtmlNonStandardTag::Permission(c) => c.into(),
			HtmlNonStandardTag::Selectedcontent(c) => c.into(),
		}
	}
}

// https://svgwg.org/svg2-draft/eltindex.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SvgTag {
	A(T![Ident]),
	Animate(T![Ident]),
	Animatemotion(T![Ident]),
	Animatetransform(T![Ident]),
	Circle(T![Ident]),
	Clippath(T![Ident]),
	Defs(T![Ident]),
	Desc(T![Ident]),
	Discard(T![Ident]),
	Ellipse(T![Ident]),
	Feblend(T![Ident]),
	Fecolormatrix(T![Ident]),
	Fecomponenttransfer(T![Ident]),
	Fecomposite(T![Ident]),
	Feconvolvematrix(T![Ident]),
	Fediffuselighting(T![Ident]),
	Fedisplacementmap(T![Ident]),
	Fedistantlight(T![Ident]),
	Fedropshadow(T![Ident]),
	Feflood(T![Ident]),
	Fefunca(T![Ident]),
	Fefuncb(T![Ident]),
	Fefuncg(T![Ident]),
	Fefuncr(T![Ident]),
	Fegaussianblur(T![Ident]),
	Feimage(T![Ident]),
	Femerge(T![Ident]),
	Femergenode(T![Ident]),
	Femorphology(T![Ident]),
	Feoffset(T![Ident]),
	Fepointlight(T![Ident]),
	Fespecularlighting(T![Ident]),
	Fespotlight(T![Ident]),
	Fetile(T![Ident]),
	Feturbulence(T![Ident]),
	Filter(T![Ident]),
	Foreignobject(T![Ident]),
	G(T![Ident]),
	Image(T![Ident]),
	Line(T![Ident]),
	Lineargradient(T![Ident]),
	Marker(T![Ident]),
	Mask(T![Ident]),
	Metadata(T![Ident]),
	Mpath(T![Ident]),
	Path(T![Ident]),
	Pattern(T![Ident]),
	Polygon(T![Ident]),
	Polyline(T![Ident]),
	Radialgradient(T![Ident]),
	Rect(T![Ident]),
	Script(T![Ident]),
	Set(T![Ident]),
	Stop(T![Ident]),
	Style(T![Ident]),
	Svg(T![Ident]),
	Switch(T![Ident]),
	Symbol(T![Ident]),
	Text(T![Ident]),
	Textpath(T![Ident]),
	Title(T![Ident]),
	Tspan(T![Ident]),
	Use(T![Ident]),
	View(T![Ident]),
}

impl<'a> Is<'a> for SvgTag {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		matches!(
			p.parse_atom_lower(c),
			atom!("a")
				| atom!("animate")
				| atom!("animatemotion")
				| atom!("animatetransform")
				| atom!("circle")
				| atom!("clippath")
				| atom!("defs")
				| atom!("desc")
				| atom!("discard")
				| atom!("ellipse")
				| atom!("feblend")
				| atom!("fecolormatrix")
				| atom!("fecomponenttransfer")
				| atom!("fecomposite")
				| atom!("feconvolvematrix")
				| atom!("fediffuselighting")
				| atom!("fedisplacementmap")
				| atom!("fedistantlight")
				| atom!("fedropshadow")
				| atom!("feflood")
				| atom!("fefunca")
				| atom!("fefuncb")
				| atom!("fefuncg")
				| atom!("fefuncr")
				| atom!("fegaussianblur")
				| atom!("feimage")
				| atom!("femerge")
				| atom!("femergenode")
				| atom!("femorphology")
				| atom!("feoffset")
				| atom!("fepointlight")
				| atom!("fespecularlighting")
				| atom!("fespotlight")
				| atom!("fetile")
				| atom!("feturbulence")
				| atom!("filter")
				| atom!("foreignobject")
				| atom!("g") | atom!("image")
				| atom!("line")
				| atom!("lineargradient")
				| atom!("marker")
				| atom!("mask")
				| atom!("metadata")
				| atom!("mpath")
				| atom!("path")
				| atom!("pattern")
				| atom!("polygon")
				| atom!("polyline")
				| atom!("radialgradient")
				| atom!("rect")
				| atom!("script")
				| atom!("set")
				| atom!("stop")
				| atom!("style")
				| atom!("svg")
				| atom!("switch")
				| atom!("symbol")
				| atom!("text")
				| atom!("textpath")
				| atom!("title")
				| atom!("tspan")
				| atom!("use")
				| atom!("view")
		)
	}
}

impl<'a> Build<'a> for SvgTag {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("a") => Self::A(<T![Ident]>::build(p, c)),
			atom!("animate") => Self::Animate(<T![Ident]>::build(p, c)),
			atom!("animatemotion") => Self::Animatemotion(<T![Ident]>::build(p, c)),
			atom!("animatetransform") => Self::Animatetransform(<T![Ident]>::build(p, c)),
			atom!("circle") => Self::Circle(<T![Ident]>::build(p, c)),
			atom!("clippath") => Self::Clippath(<T![Ident]>::build(p, c)),
			atom!("defs") => Self::Defs(<T![Ident]>::build(p, c)),
			atom!("desc") => Self::Desc(<T![Ident]>::build(p, c)),
			atom!("discard") => Self::Discard(<T![Ident]>::build(p, c)),
			atom!("ellipse") => Self::Ellipse(<T![Ident]>::build(p, c)),
			atom!("feblend") => Self::Feblend(<T![Ident]>::build(p, c)),
			atom!("fecolormatrix") => Self::Fecolormatrix(<T![Ident]>::build(p, c)),
			atom!("fecomponenttransfer") => Self::Fecomponenttransfer(<T![Ident]>::build(p, c)),
			atom!("fecomposite") => Self::Fecomposite(<T![Ident]>::build(p, c)),
			atom!("feconvolvematrix") => Self::Feconvolvematrix(<T![Ident]>::build(p, c)),
			atom!("fediffuselighting") => Self::Fediffuselighting(<T![Ident]>::build(p, c)),
			atom!("fedisplacementmap") => Self::Fedisplacementmap(<T![Ident]>::build(p, c)),
			atom!("fedistantlight") => Self::Fedistantlight(<T![Ident]>::build(p, c)),
			atom!("fedropshadow") => Self::Fedropshadow(<T![Ident]>::build(p, c)),
			atom!("feflood") => Self::Feflood(<T![Ident]>::build(p, c)),
			atom!("fefunca") => Self::Fefunca(<T![Ident]>::build(p, c)),
			atom!("fefuncb") => Self::Fefuncb(<T![Ident]>::build(p, c)),
			atom!("fefuncg") => Self::Fefuncg(<T![Ident]>::build(p, c)),
			atom!("fefuncr") => Self::Fefuncr(<T![Ident]>::build(p, c)),
			atom!("fegaussianblur") => Self::Fegaussianblur(<T![Ident]>::build(p, c)),
			atom!("feimage") => Self::Feimage(<T![Ident]>::build(p, c)),
			atom!("femerge") => Self::Femerge(<T![Ident]>::build(p, c)),
			atom!("femergenode") => Self::Femergenode(<T![Ident]>::build(p, c)),
			atom!("femorphology") => Self::Femorphology(<T![Ident]>::build(p, c)),
			atom!("feoffset") => Self::Feoffset(<T![Ident]>::build(p, c)),
			atom!("fepointlight") => Self::Fepointlight(<T![Ident]>::build(p, c)),
			atom!("fespecularlighting") => Self::Fespecularlighting(<T![Ident]>::build(p, c)),
			atom!("fespotlight") => Self::Fespotlight(<T![Ident]>::build(p, c)),
			atom!("fetile") => Self::Fetile(<T![Ident]>::build(p, c)),
			atom!("feturbulence") => Self::Feturbulence(<T![Ident]>::build(p, c)),
			atom!("filter") => Self::Filter(<T![Ident]>::build(p, c)),
			atom!("foreignobject") => Self::Foreignobject(<T![Ident]>::build(p, c)),
			atom!("g") => Self::G(<T![Ident]>::build(p, c)),
			atom!("image") => Self::Image(<T![Ident]>::build(p, c)),
			atom!("line") => Self::Line(<T![Ident]>::build(p, c)),
			atom!("lineargradient") => Self::Lineargradient(<T![Ident]>::build(p, c)),
			atom!("marker") => Self::A(<T![Ident]>::build(p, c)),
			atom!("mask") => Self::A(<T![Ident]>::build(p, c)),
			atom!("metadata") => Self::A(<T![Ident]>::build(p, c)),
			atom!("mpath") => Self::A(<T![Ident]>::build(p, c)),
			atom!("path") => Self::A(<T![Ident]>::build(p, c)),
			atom!("pattern") => Self::A(<T![Ident]>::build(p, c)),
			atom!("polygon") => Self::A(<T![Ident]>::build(p, c)),
			atom!("polyline") => Self::A(<T![Ident]>::build(p, c)),
			atom!("radialgradient") => Self::A(<T![Ident]>::build(p, c)),
			atom!("rect") => Self::A(<T![Ident]>::build(p, c)),
			atom!("script") => Self::A(<T![Ident]>::build(p, c)),
			atom!("set") => Self::A(<T![Ident]>::build(p, c)),
			atom!("stop") => Self::A(<T![Ident]>::build(p, c)),
			atom!("style") => Self::A(<T![Ident]>::build(p, c)),
			atom!("svg") => Self::A(<T![Ident]>::build(p, c)),
			atom!("switch") => Self::A(<T![Ident]>::build(p, c)),
			atom!("symbol") => Self::A(<T![Ident]>::build(p, c)),
			atom!("text") => Self::A(<T![Ident]>::build(p, c)),
			atom!("textpath") => Self::A(<T![Ident]>::build(p, c)),
			atom!("title") => Self::A(<T![Ident]>::build(p, c)),
			atom!("tspan") => Self::A(<T![Ident]>::build(p, c)),
			atom!("use") => Self::A(<T![Ident]>::build(p, c)),
			atom!("view") => Self::A(<T![Ident]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<SvgTag> for Cursor {
	fn from(value: SvgTag) -> Self {
		match value {
			SvgTag::A(c) => c.into(),
			SvgTag::Animate(c) => c.into(),
			SvgTag::Animatemotion(c) => c.into(),
			SvgTag::Animatetransform(c) => c.into(),
			SvgTag::Circle(c) => c.into(),
			SvgTag::Clippath(c) => c.into(),
			SvgTag::Defs(c) => c.into(),
			SvgTag::Desc(c) => c.into(),
			SvgTag::Discard(c) => c.into(),
			SvgTag::Ellipse(c) => c.into(),
			SvgTag::Feblend(c) => c.into(),
			SvgTag::Fecolormatrix(c) => c.into(),
			SvgTag::Fecomponenttransfer(c) => c.into(),
			SvgTag::Fecomposite(c) => c.into(),
			SvgTag::Feconvolvematrix(c) => c.into(),
			SvgTag::Fediffuselighting(c) => c.into(),
			SvgTag::Fedisplacementmap(c) => c.into(),
			SvgTag::Fedistantlight(c) => c.into(),
			SvgTag::Fedropshadow(c) => c.into(),
			SvgTag::Feflood(c) => c.into(),
			SvgTag::Fefunca(c) => c.into(),
			SvgTag::Fefuncb(c) => c.into(),
			SvgTag::Fefuncg(c) => c.into(),
			SvgTag::Fefuncr(c) => c.into(),
			SvgTag::Fegaussianblur(c) => c.into(),
			SvgTag::Feimage(c) => c.into(),
			SvgTag::Femerge(c) => c.into(),
			SvgTag::Femergenode(c) => c.into(),
			SvgTag::Femorphology(c) => c.into(),
			SvgTag::Feoffset(c) => c.into(),
			SvgTag::Fepointlight(c) => c.into(),
			SvgTag::Fespecularlighting(c) => c.into(),
			SvgTag::Fespotlight(c) => c.into(),
			SvgTag::Fetile(c) => c.into(),
			SvgTag::Feturbulence(c) => c.into(),
			SvgTag::Filter(c) => c.into(),
			SvgTag::Foreignobject(c) => c.into(),
			SvgTag::G(c) => c.into(),
			SvgTag::Image(c) => c.into(),
			SvgTag::Line(c) => c.into(),
			SvgTag::Lineargradient(c) => c.into(),
			SvgTag::Marker(c) => c.into(),
			SvgTag::Mask(c) => c.into(),
			SvgTag::Metadata(c) => c.into(),
			SvgTag::Mpath(c) => c.into(),
			SvgTag::Path(c) => c.into(),
			SvgTag::Pattern(c) => c.into(),
			SvgTag::Polygon(c) => c.into(),
			SvgTag::Polyline(c) => c.into(),
			SvgTag::Radialgradient(c) => c.into(),
			SvgTag::Rect(c) => c.into(),
			SvgTag::Script(c) => c.into(),
			SvgTag::Set(c) => c.into(),
			SvgTag::Stop(c) => c.into(),
			SvgTag::Style(c) => c.into(),
			SvgTag::Svg(c) => c.into(),
			SvgTag::Switch(c) => c.into(),
			SvgTag::Symbol(c) => c.into(),
			SvgTag::Text(c) => c.into(),
			SvgTag::Textpath(c) => c.into(),
			SvgTag::Title(c) => c.into(),
			SvgTag::Tspan(c) => c.into(),
			SvgTag::Use(c) => c.into(),
			SvgTag::View(c) => c.into(),
		}
	}
}

// https://w3c.github.io/mathml/#mmlindex_elements
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum MathmlTag {
	Abs(T![Ident]),
	And(T![Ident]),
	Annotation(T![Ident]),
	AnnotationXml(T![Ident]),
	Apply(T![Ident]),
	Approx(T![Ident]),
	Arg(T![Ident]),
	Bind(T![Ident]),
	Bvar(T![Ident]),
	Card(T![Ident]),
	Cartesianproduct(T![Ident]),
	Cbytes(T![Ident]),
	Ceiling(T![Ident]),
	Cerror(T![Ident]),
	Ci(T![Ident]),
	Cn(T![Ident]),
	Codomain(T![Ident]),
	Compose(T![Ident]),
	Condition(T![Ident]),
	Conjugate(T![Ident]),
	Cs(T![Ident]),
	Csymbol(T![Ident]),
	Curl(T![Ident]),
	Declare(T![Ident]),
	Degree(T![Ident]),
	Determinant(T![Ident]),
	Diff(T![Ident]),
	Divergence(T![Ident]),
	Divide(T![Ident]),
	Domain(T![Ident]),
	Domainofapplication(T![Ident]),
	Emptyset(T![Ident]),
	Eq(T![Ident]),
	Equivalent(T![Ident]),
	Exists(T![Ident]),
	Exp(T![Ident]),
	Factorial(T![Ident]),
	Factorof(T![Ident]),
	Floor(T![Ident]),
	Fn(T![Ident]),
	Forall(T![Ident]),
	Gcd(T![Ident]),
	Geq(T![Ident]),
	Grad(T![Ident]),
	Gt(T![Ident]),
	Ident(T![Ident]),
	Image(T![Ident]),
	Imaginary(T![Ident]),
	Img(T![Ident]),
	Implies(T![Ident]),
	In(T![Ident]),
	Int(T![Ident]),
	Intersect(T![Ident]),
	Interval(T![Ident]),
	Inverse(T![Ident]),
	Lambda(T![Ident]),
	Laplacian(T![Ident]),
	Lcm(T![Ident]),
	Leq(T![Ident]),
	Limit(T![Ident]),
	List(T![Ident]),
	Ln(T![Ident]),
	Log(T![Ident]),
	Logbase(T![Ident]),
	Lowlimit(T![Ident]),
	Lt(T![Ident]),
	Maction(T![Ident]),
	Maligngroup(T![Ident]),
	Malignmark(T![Ident]),
	Math(T![Ident]),
	Matrix(T![Ident]),
	Matrixrow(T![Ident]),
	Max(T![Ident]),
	Mean(T![Ident]),
	Median(T![Ident]),
	Menclose(T![Ident]),
	Merror(T![Ident]),
	Mfenced(T![Ident]),
	Mfrac(T![Ident]),
	Mfraction(T![Ident]),
	Mglyph(T![Ident]),
	Mi(T![Ident]),
	Min(T![Ident]),
	Minus(T![Ident]),
	Mlabeledtr(T![Ident]),
	Mlongdiv(T![Ident]),
	Mmultiscripts(T![Ident]),
	Mn(T![Ident]),
	Mo(T![Ident]),
	Mode(T![Ident]),
	Moment(T![Ident]),
	Momentabout(T![Ident]),
	Mover(T![Ident]),
	Mpadded(T![Ident]),
	Mphantom(T![Ident]),
	Mprescripts(T![Ident]),
	Mroot(T![Ident]),
	Mrow(T![Ident]),
	Ms(T![Ident]),
	Mscarries(T![Ident]),
	Mscarry(T![Ident]),
	Msgroup(T![Ident]),
	Msline(T![Ident]),
	Mspace(T![Ident]),
	Msqrt(T![Ident]),
	Msrow(T![Ident]),
	Mstack(T![Ident]),
	Mstyle(T![Ident]),
	Msub(T![Ident]),
	Msubsup(T![Ident]),
	Msup(T![Ident]),
	Mtable(T![Ident]),
	Mtd(T![Ident]),
	Mtext(T![Ident]),
	Mtr(T![Ident]),
	Munder(T![Ident]),
	Munderover(T![Ident]),
	Neq(T![Ident]),
	None(T![Ident]),
	Not(T![Ident]),
	Notin(T![Ident]),
	Notprsubset(T![Ident]),
	Notsubset(T![Ident]),
	Or(T![Ident]),
	Otherwise(T![Ident]),
	Outerproduct(T![Ident]),
	Partialdiff(T![Ident]),
	Piece(T![Ident]),
	Piecewise(T![Ident]),
	Plus(T![Ident]),
	Power(T![Ident]),
	Product(T![Ident]),
	Prsubset(T![Ident]),
	Quotient(T![Ident]),
	Real(T![Ident]),
	Reln(T![Ident]),
	Rem(T![Ident]),
	Root(T![Ident]),
	Scalarproduct(T![Ident]),
	Sdev(T![Ident]),
	Selector(T![Ident]),
	Semantics(T![Ident]),
	Sep(T![Ident]),
	Set(T![Ident]),
	Setdiff(T![Ident]),
	Share(T![Ident]),
	Sin(T![Ident]),
	Subset(T![Ident]),
	Sum(T![Ident]),
	Tendsto(T![Ident]),
	Times(T![Ident]),
	Transpose(T![Ident]),
	Union(T![Ident]),
	Uplimit(T![Ident]),
	Variance(T![Ident]),
	Vector(T![Ident]),
	Vectorproduct(T![Ident]),
	Xo(T![Ident]),
}

impl<'a> Is<'a> for MathmlTag {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		matches!(
			p.parse_atom_lower(c),
			atom!("abs")
				| atom!("and")
				| atom!("annotation")
				| atom!("annotationxml")
				| atom!("apply")
				| atom!("approx")
				| atom!("arg")
				| atom!("bind")
				| atom!("bvar")
				| atom!("card")
				| atom!("cartesianproduct")
				| atom!("cbytes")
				| atom!("ceiling")
				| atom!("cerror")
				| atom!("ci")
				| atom!("cn")
				| atom!("codomain")
				| atom!("compose")
				| atom!("condition")
				| atom!("conjugate")
				| atom!("cs")
				| atom!("csymbol")
				| atom!("curl")
				| atom!("declare")
				| atom!("degree")
				| atom!("determinant")
				| atom!("diff")
				| atom!("divergence")
				| atom!("divide")
				| atom!("domain")
				| atom!("domainofapplication")
				| atom!("emptyset")
				| atom!("eq")
				| atom!("equivalent")
				| atom!("exists")
				| atom!("exp")
				| atom!("factorial")
				| atom!("factorof")
				| atom!("floor")
				| atom!("fn")
				| atom!("forall")
				| atom!("gcd")
				| atom!("geq")
				| atom!("grad")
				| atom!("gt")
				| atom!("ident")
				| atom!("image")
				| atom!("imaginary")
				| atom!("img")
				| atom!("implies")
				| atom!("in")
				| atom!("int")
				| atom!("intersect")
				| atom!("interval")
				| atom!("inverse")
				| atom!("lambda")
				| atom!("laplacian")
				| atom!("lcm")
				| atom!("leq")
				| atom!("limit")
				| atom!("list")
				| atom!("ln")
				| atom!("log")
				| atom!("logbase")
				| atom!("lowlimit")
				| atom!("lt")
				| atom!("maction")
				| atom!("maligngroup")
				| atom!("malignmark")
				| atom!("math")
				| atom!("matrix")
				| atom!("matrixrow")
				| atom!("max")
				| atom!("mean")
				| atom!("median")
				| atom!("menclose")
				| atom!("merror")
				| atom!("mfenced")
				| atom!("mfrac")
				| atom!("mfraction")
				| atom!("mglyph")
				| atom!("mi")
				| atom!("min")
				| atom!("minus")
				| atom!("mlabeledtr")
				| atom!("mlongdiv")
				| atom!("mmultiscripts")
				| atom!("mn")
				| atom!("mo")
				| atom!("mode")
				| atom!("moment")
				| atom!("momentabout")
				| atom!("mover")
				| atom!("mpadded")
				| atom!("mphantom")
				| atom!("mprescripts")
				| atom!("mroot")
				| atom!("mrow")
				| atom!("ms")
				| atom!("mscarries")
				| atom!("mscarry")
				| atom!("msgroup")
				| atom!("msline")
				| atom!("mspace")
				| atom!("msqrt")
				| atom!("msrow")
				| atom!("mstack")
				| atom!("mstyle")
				| atom!("msub")
				| atom!("msubsup")
				| atom!("msup")
				| atom!("mtable")
				| atom!("mtd")
				| atom!("mtext")
				| atom!("mtr")
				| atom!("munder")
				| atom!("munderover")
				| atom!("neq")
				| atom!("none")
				| atom!("not")
				| atom!("notin")
				| atom!("notprsubset")
				| atom!("notsubset")
				| atom!("or")
				| atom!("otherwise")
				| atom!("outerproduct")
				| atom!("partialdiff")
				| atom!("piece")
				| atom!("piecewise")
				| atom!("plus")
				| atom!("power")
				| atom!("product")
				| atom!("prsubset")
				| atom!("quotient")
				| atom!("real")
				| atom!("reln")
				| atom!("rem")
				| atom!("root")
				| atom!("scalarproduct")
				| atom!("sdev")
				| atom!("selector")
				| atom!("semantics")
				| atom!("sep")
				| atom!("set")
				| atom!("setdiff")
				| atom!("share")
				| atom!("sin")
				| atom!("subset")
				| atom!("sum")
				| atom!("tendsto")
				| atom!("times")
				| atom!("transpose")
				| atom!("union")
				| atom!("uplimit")
				| atom!("variance")
				| atom!("vector")
				| atom!("vectorproduct")
				| atom!("xo")
		)
	}
}

impl<'a> Build<'a> for MathmlTag {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("abs") => Self::Abs(<T![Ident]>::build(p, c)),
			atom!("and") => Self::And(<T![Ident]>::build(p, c)),
			atom!("annotation") => Self::Annotation(<T![Ident]>::build(p, c)),
			atom!("annotationxml") => Self::AnnotationXml(<T![Ident]>::build(p, c)),
			atom!("apply") => Self::Apply(<T![Ident]>::build(p, c)),
			atom!("approx") => Self::Approx(<T![Ident]>::build(p, c)),
			atom!("arg") => Self::Arg(<T![Ident]>::build(p, c)),
			atom!("bind") => Self::Bind(<T![Ident]>::build(p, c)),
			atom!("bvar") => Self::Bvar(<T![Ident]>::build(p, c)),
			atom!("card") => Self::Card(<T![Ident]>::build(p, c)),
			atom!("cartesianproduct") => Self::Cartesianproduct(<T![Ident]>::build(p, c)),
			atom!("cbytes") => Self::Cbytes(<T![Ident]>::build(p, c)),
			atom!("ceiling") => Self::Ceiling(<T![Ident]>::build(p, c)),
			atom!("cerror") => Self::Cerror(<T![Ident]>::build(p, c)),
			atom!("ci") => Self::Ci(<T![Ident]>::build(p, c)),
			atom!("cn") => Self::Cn(<T![Ident]>::build(p, c)),
			atom!("codomain") => Self::Codomain(<T![Ident]>::build(p, c)),
			atom!("compose") => Self::Compose(<T![Ident]>::build(p, c)),
			atom!("condition") => Self::Condition(<T![Ident]>::build(p, c)),
			atom!("conjugate") => Self::Conjugate(<T![Ident]>::build(p, c)),
			atom!("cs") => Self::Cs(<T![Ident]>::build(p, c)),
			atom!("csymbol") => Self::Csymbol(<T![Ident]>::build(p, c)),
			atom!("curl") => Self::Curl(<T![Ident]>::build(p, c)),
			atom!("declare") => Self::Declare(<T![Ident]>::build(p, c)),
			atom!("degree") => Self::Degree(<T![Ident]>::build(p, c)),
			atom!("determinant") => Self::Determinant(<T![Ident]>::build(p, c)),
			atom!("diff") => Self::Diff(<T![Ident]>::build(p, c)),
			atom!("divergence") => Self::Divergence(<T![Ident]>::build(p, c)),
			atom!("divide") => Self::Divide(<T![Ident]>::build(p, c)),
			atom!("domain") => Self::Domain(<T![Ident]>::build(p, c)),
			atom!("domainofapplication") => Self::Domainofapplication(<T![Ident]>::build(p, c)),
			atom!("emptyset") => Self::Emptyset(<T![Ident]>::build(p, c)),
			atom!("eq") => Self::Eq(<T![Ident]>::build(p, c)),
			atom!("equivalent") => Self::Equivalent(<T![Ident]>::build(p, c)),
			atom!("exists") => Self::Exists(<T![Ident]>::build(p, c)),
			atom!("exp") => Self::Exp(<T![Ident]>::build(p, c)),
			atom!("factorial") => Self::Factorial(<T![Ident]>::build(p, c)),
			atom!("factorof") => Self::Factorof(<T![Ident]>::build(p, c)),
			atom!("floor") => Self::Floor(<T![Ident]>::build(p, c)),
			atom!("fn") => Self::Fn(<T![Ident]>::build(p, c)),
			atom!("forall") => Self::Forall(<T![Ident]>::build(p, c)),
			atom!("gcd") => Self::Gcd(<T![Ident]>::build(p, c)),
			atom!("geq") => Self::Geq(<T![Ident]>::build(p, c)),
			atom!("grad") => Self::Grad(<T![Ident]>::build(p, c)),
			atom!("gt") => Self::Gt(<T![Ident]>::build(p, c)),
			atom!("ident") => Self::Ident(<T![Ident]>::build(p, c)),
			atom!("image") => Self::Image(<T![Ident]>::build(p, c)),
			atom!("imaginary") => Self::Imaginary(<T![Ident]>::build(p, c)),
			atom!("img") => Self::Img(<T![Ident]>::build(p, c)),
			atom!("implies") => Self::Implies(<T![Ident]>::build(p, c)),
			atom!("in") => Self::In(<T![Ident]>::build(p, c)),
			atom!("int") => Self::Int(<T![Ident]>::build(p, c)),
			atom!("intersect") => Self::Intersect(<T![Ident]>::build(p, c)),
			atom!("interval") => Self::Interval(<T![Ident]>::build(p, c)),
			atom!("inverse") => Self::Inverse(<T![Ident]>::build(p, c)),
			atom!("lambda") => Self::Lambda(<T![Ident]>::build(p, c)),
			atom!("laplacian") => Self::Laplacian(<T![Ident]>::build(p, c)),
			atom!("lcm") => Self::Lcm(<T![Ident]>::build(p, c)),
			atom!("leq") => Self::Leq(<T![Ident]>::build(p, c)),
			atom!("limit") => Self::Limit(<T![Ident]>::build(p, c)),
			atom!("list") => Self::List(<T![Ident]>::build(p, c)),
			atom!("ln") => Self::Ln(<T![Ident]>::build(p, c)),
			atom!("log") => Self::Log(<T![Ident]>::build(p, c)),
			atom!("logbase") => Self::Logbase(<T![Ident]>::build(p, c)),
			atom!("lowlimit") => Self::Lowlimit(<T![Ident]>::build(p, c)),
			atom!("lt") => Self::Lt(<T![Ident]>::build(p, c)),
			atom!("maction") => Self::Maction(<T![Ident]>::build(p, c)),
			atom!("maligngroup") => Self::Maligngroup(<T![Ident]>::build(p, c)),
			atom!("malignmark") => Self::Malignmark(<T![Ident]>::build(p, c)),
			atom!("math") => Self::Math(<T![Ident]>::build(p, c)),
			atom!("matrix") => Self::Matrix(<T![Ident]>::build(p, c)),
			atom!("matrixrow") => Self::Matrixrow(<T![Ident]>::build(p, c)),
			atom!("max") => Self::Max(<T![Ident]>::build(p, c)),
			atom!("mean") => Self::Mean(<T![Ident]>::build(p, c)),
			atom!("median") => Self::Median(<T![Ident]>::build(p, c)),
			atom!("menclose") => Self::Menclose(<T![Ident]>::build(p, c)),
			atom!("merror") => Self::Merror(<T![Ident]>::build(p, c)),
			atom!("mfenced") => Self::Mfenced(<T![Ident]>::build(p, c)),
			atom!("mfrac") => Self::Mfrac(<T![Ident]>::build(p, c)),
			atom!("mfraction") => Self::Mfraction(<T![Ident]>::build(p, c)),
			atom!("mglyph") => Self::Mglyph(<T![Ident]>::build(p, c)),
			atom!("mi") => Self::Mi(<T![Ident]>::build(p, c)),
			atom!("min") => Self::Min(<T![Ident]>::build(p, c)),
			atom!("minus") => Self::Minus(<T![Ident]>::build(p, c)),
			atom!("mlabeledtr") => Self::Mlabeledtr(<T![Ident]>::build(p, c)),
			atom!("mlongdiv") => Self::Mlongdiv(<T![Ident]>::build(p, c)),
			atom!("mmultiscripts") => Self::Mmultiscripts(<T![Ident]>::build(p, c)),
			atom!("mn") => Self::Mn(<T![Ident]>::build(p, c)),
			atom!("mo") => Self::Mo(<T![Ident]>::build(p, c)),
			atom!("mode") => Self::Mode(<T![Ident]>::build(p, c)),
			atom!("moment") => Self::Moment(<T![Ident]>::build(p, c)),
			atom!("momentabout") => Self::Momentabout(<T![Ident]>::build(p, c)),
			atom!("mover") => Self::Mover(<T![Ident]>::build(p, c)),
			atom!("mpadded") => Self::Mpadded(<T![Ident]>::build(p, c)),
			atom!("mphantom") => Self::Mphantom(<T![Ident]>::build(p, c)),
			atom!("mprescripts") => Self::Mprescripts(<T![Ident]>::build(p, c)),
			atom!("mroot") => Self::Mroot(<T![Ident]>::build(p, c)),
			atom!("mrow") => Self::Mrow(<T![Ident]>::build(p, c)),
			atom!("ms") => Self::Ms(<T![Ident]>::build(p, c)),
			atom!("mscarries") => Self::Mscarries(<T![Ident]>::build(p, c)),
			atom!("mscarry") => Self::Mscarry(<T![Ident]>::build(p, c)),
			atom!("msgroup") => Self::Msgroup(<T![Ident]>::build(p, c)),
			atom!("msline") => Self::Msline(<T![Ident]>::build(p, c)),
			atom!("mspace") => Self::Mspace(<T![Ident]>::build(p, c)),
			atom!("msqrt") => Self::Msqrt(<T![Ident]>::build(p, c)),
			atom!("msrow") => Self::Msrow(<T![Ident]>::build(p, c)),
			atom!("mstack") => Self::Mstack(<T![Ident]>::build(p, c)),
			atom!("mstyle") => Self::Mstyle(<T![Ident]>::build(p, c)),
			atom!("msub") => Self::Msub(<T![Ident]>::build(p, c)),
			atom!("msubsup") => Self::Msubsup(<T![Ident]>::build(p, c)),
			atom!("msup") => Self::Msup(<T![Ident]>::build(p, c)),
			atom!("mtable") => Self::Mtable(<T![Ident]>::build(p, c)),
			atom!("mtd") => Self::Mtd(<T![Ident]>::build(p, c)),
			atom!("mtext") => Self::Mtext(<T![Ident]>::build(p, c)),
			atom!("mtr") => Self::Mtr(<T![Ident]>::build(p, c)),
			atom!("munder") => Self::Munder(<T![Ident]>::build(p, c)),
			atom!("munderover") => Self::Munderover(<T![Ident]>::build(p, c)),
			atom!("neq") => Self::Neq(<T![Ident]>::build(p, c)),
			atom!("none") => Self::None(<T![Ident]>::build(p, c)),
			atom!("not") => Self::Not(<T![Ident]>::build(p, c)),
			atom!("notin") => Self::Notin(<T![Ident]>::build(p, c)),
			atom!("notprsubset") => Self::Notprsubset(<T![Ident]>::build(p, c)),
			atom!("notsubset") => Self::Notsubset(<T![Ident]>::build(p, c)),
			atom!("or") => Self::Or(<T![Ident]>::build(p, c)),
			atom!("otherwise") => Self::Otherwise(<T![Ident]>::build(p, c)),
			atom!("outerproduct") => Self::Outerproduct(<T![Ident]>::build(p, c)),
			atom!("partialdiff") => Self::Partialdiff(<T![Ident]>::build(p, c)),
			atom!("piece") => Self::Piece(<T![Ident]>::build(p, c)),
			atom!("piecewise") => Self::Piecewise(<T![Ident]>::build(p, c)),
			atom!("plus") => Self::Plus(<T![Ident]>::build(p, c)),
			atom!("power") => Self::Power(<T![Ident]>::build(p, c)),
			atom!("product") => Self::Product(<T![Ident]>::build(p, c)),
			atom!("prsubset") => Self::Prsubset(<T![Ident]>::build(p, c)),
			atom!("quotient") => Self::Quotient(<T![Ident]>::build(p, c)),
			atom!("real") => Self::Real(<T![Ident]>::build(p, c)),
			atom!("reln") => Self::Reln(<T![Ident]>::build(p, c)),
			atom!("rem") => Self::Rem(<T![Ident]>::build(p, c)),
			atom!("root") => Self::Root(<T![Ident]>::build(p, c)),
			atom!("scalarproduct") => Self::Scalarproduct(<T![Ident]>::build(p, c)),
			atom!("sdev") => Self::Sdev(<T![Ident]>::build(p, c)),
			atom!("selector") => Self::Selector(<T![Ident]>::build(p, c)),
			atom!("semantics") => Self::Semantics(<T![Ident]>::build(p, c)),
			atom!("sep") => Self::Sep(<T![Ident]>::build(p, c)),
			atom!("set") => Self::Set(<T![Ident]>::build(p, c)),
			atom!("setdiff") => Self::Setdiff(<T![Ident]>::build(p, c)),
			atom!("share") => Self::Share(<T![Ident]>::build(p, c)),
			atom!("sin") => Self::Sin(<T![Ident]>::build(p, c)),
			atom!("subset") => Self::Subset(<T![Ident]>::build(p, c)),
			atom!("sum") => Self::Sum(<T![Ident]>::build(p, c)),
			atom!("tendsto") => Self::Tendsto(<T![Ident]>::build(p, c)),
			atom!("times") => Self::Times(<T![Ident]>::build(p, c)),
			atom!("transpose") => Self::Transpose(<T![Ident]>::build(p, c)),
			atom!("union") => Self::Union(<T![Ident]>::build(p, c)),
			atom!("uplimit") => Self::Uplimit(<T![Ident]>::build(p, c)),
			atom!("variance") => Self::Variance(<T![Ident]>::build(p, c)),
			atom!("vector") => Self::Vector(<T![Ident]>::build(p, c)),
			atom!("vectorproduct") => Self::Vectorproduct(<T![Ident]>::build(p, c)),
			atom!("xo") => Self::Xo(<T![Ident]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<MathmlTag> for Cursor {
	fn from(value: MathmlTag) -> Self {
		match value {
			MathmlTag::Abs(c) => c.into(),
			MathmlTag::And(c) => c.into(),
			MathmlTag::Annotation(c) => c.into(),
			MathmlTag::AnnotationXml(c) => c.into(),
			MathmlTag::Apply(c) => c.into(),
			MathmlTag::Approx(c) => c.into(),
			MathmlTag::Arg(c) => c.into(),
			MathmlTag::Bind(c) => c.into(),
			MathmlTag::Bvar(c) => c.into(),
			MathmlTag::Card(c) => c.into(),
			MathmlTag::Cartesianproduct(c) => c.into(),
			MathmlTag::Cbytes(c) => c.into(),
			MathmlTag::Ceiling(c) => c.into(),
			MathmlTag::Cerror(c) => c.into(),
			MathmlTag::Ci(c) => c.into(),
			MathmlTag::Cn(c) => c.into(),
			MathmlTag::Codomain(c) => c.into(),
			MathmlTag::Compose(c) => c.into(),
			MathmlTag::Condition(c) => c.into(),
			MathmlTag::Conjugate(c) => c.into(),
			MathmlTag::Cs(c) => c.into(),
			MathmlTag::Csymbol(c) => c.into(),
			MathmlTag::Curl(c) => c.into(),
			MathmlTag::Declare(c) => c.into(),
			MathmlTag::Degree(c) => c.into(),
			MathmlTag::Determinant(c) => c.into(),
			MathmlTag::Diff(c) => c.into(),
			MathmlTag::Divergence(c) => c.into(),
			MathmlTag::Divide(c) => c.into(),
			MathmlTag::Domain(c) => c.into(),
			MathmlTag::Domainofapplication(c) => c.into(),
			MathmlTag::Emptyset(c) => c.into(),
			MathmlTag::Eq(c) => c.into(),
			MathmlTag::Equivalent(c) => c.into(),
			MathmlTag::Exists(c) => c.into(),
			MathmlTag::Exp(c) => c.into(),
			MathmlTag::Factorial(c) => c.into(),
			MathmlTag::Factorof(c) => c.into(),
			MathmlTag::Floor(c) => c.into(),
			MathmlTag::Fn(c) => c.into(),
			MathmlTag::Forall(c) => c.into(),
			MathmlTag::Gcd(c) => c.into(),
			MathmlTag::Geq(c) => c.into(),
			MathmlTag::Grad(c) => c.into(),
			MathmlTag::Gt(c) => c.into(),
			MathmlTag::Ident(c) => c.into(),
			MathmlTag::Image(c) => c.into(),
			MathmlTag::Imaginary(c) => c.into(),
			MathmlTag::Img(c) => c.into(),
			MathmlTag::Implies(c) => c.into(),
			MathmlTag::In(c) => c.into(),
			MathmlTag::Int(c) => c.into(),
			MathmlTag::Intersect(c) => c.into(),
			MathmlTag::Interval(c) => c.into(),
			MathmlTag::Inverse(c) => c.into(),
			MathmlTag::Lambda(c) => c.into(),
			MathmlTag::Laplacian(c) => c.into(),
			MathmlTag::Lcm(c) => c.into(),
			MathmlTag::Leq(c) => c.into(),
			MathmlTag::Limit(c) => c.into(),
			MathmlTag::List(c) => c.into(),
			MathmlTag::Ln(c) => c.into(),
			MathmlTag::Log(c) => c.into(),
			MathmlTag::Logbase(c) => c.into(),
			MathmlTag::Lowlimit(c) => c.into(),
			MathmlTag::Lt(c) => c.into(),
			MathmlTag::Maction(c) => c.into(),
			MathmlTag::Maligngroup(c) => c.into(),
			MathmlTag::Malignmark(c) => c.into(),
			MathmlTag::Math(c) => c.into(),
			MathmlTag::Matrix(c) => c.into(),
			MathmlTag::Matrixrow(c) => c.into(),
			MathmlTag::Max(c) => c.into(),
			MathmlTag::Mean(c) => c.into(),
			MathmlTag::Median(c) => c.into(),
			MathmlTag::Menclose(c) => c.into(),
			MathmlTag::Merror(c) => c.into(),
			MathmlTag::Mfenced(c) => c.into(),
			MathmlTag::Mfrac(c) => c.into(),
			MathmlTag::Mfraction(c) => c.into(),
			MathmlTag::Mglyph(c) => c.into(),
			MathmlTag::Mi(c) => c.into(),
			MathmlTag::Min(c) => c.into(),
			MathmlTag::Minus(c) => c.into(),
			MathmlTag::Mlabeledtr(c) => c.into(),
			MathmlTag::Mlongdiv(c) => c.into(),
			MathmlTag::Mmultiscripts(c) => c.into(),
			MathmlTag::Mn(c) => c.into(),
			MathmlTag::Mo(c) => c.into(),
			MathmlTag::Mode(c) => c.into(),
			MathmlTag::Moment(c) => c.into(),
			MathmlTag::Momentabout(c) => c.into(),
			MathmlTag::Mover(c) => c.into(),
			MathmlTag::Mpadded(c) => c.into(),
			MathmlTag::Mphantom(c) => c.into(),
			MathmlTag::Mprescripts(c) => c.into(),
			MathmlTag::Mroot(c) => c.into(),
			MathmlTag::Mrow(c) => c.into(),
			MathmlTag::Ms(c) => c.into(),
			MathmlTag::Mscarries(c) => c.into(),
			MathmlTag::Mscarry(c) => c.into(),
			MathmlTag::Msgroup(c) => c.into(),
			MathmlTag::Msline(c) => c.into(),
			MathmlTag::Mspace(c) => c.into(),
			MathmlTag::Msqrt(c) => c.into(),
			MathmlTag::Msrow(c) => c.into(),
			MathmlTag::Mstack(c) => c.into(),
			MathmlTag::Mstyle(c) => c.into(),
			MathmlTag::Msub(c) => c.into(),
			MathmlTag::Msubsup(c) => c.into(),
			MathmlTag::Msup(c) => c.into(),
			MathmlTag::Mtable(c) => c.into(),
			MathmlTag::Mtd(c) => c.into(),
			MathmlTag::Mtext(c) => c.into(),
			MathmlTag::Mtr(c) => c.into(),
			MathmlTag::Munder(c) => c.into(),
			MathmlTag::Munderover(c) => c.into(),
			MathmlTag::Neq(c) => c.into(),
			MathmlTag::None(c) => c.into(),
			MathmlTag::Not(c) => c.into(),
			MathmlTag::Notin(c) => c.into(),
			MathmlTag::Notprsubset(c) => c.into(),
			MathmlTag::Notsubset(c) => c.into(),
			MathmlTag::Or(c) => c.into(),
			MathmlTag::Otherwise(c) => c.into(),
			MathmlTag::Outerproduct(c) => c.into(),
			MathmlTag::Partialdiff(c) => c.into(),
			MathmlTag::Piece(c) => c.into(),
			MathmlTag::Piecewise(c) => c.into(),
			MathmlTag::Plus(c) => c.into(),
			MathmlTag::Power(c) => c.into(),
			MathmlTag::Product(c) => c.into(),
			MathmlTag::Prsubset(c) => c.into(),
			MathmlTag::Quotient(c) => c.into(),
			MathmlTag::Real(c) => c.into(),
			MathmlTag::Reln(c) => c.into(),
			MathmlTag::Rem(c) => c.into(),
			MathmlTag::Root(c) => c.into(),
			MathmlTag::Scalarproduct(c) => c.into(),
			MathmlTag::Sdev(c) => c.into(),
			MathmlTag::Selector(c) => c.into(),
			MathmlTag::Semantics(c) => c.into(),
			MathmlTag::Sep(c) => c.into(),
			MathmlTag::Set(c) => c.into(),
			MathmlTag::Setdiff(c) => c.into(),
			MathmlTag::Share(c) => c.into(),
			MathmlTag::Sin(c) => c.into(),
			MathmlTag::Subset(c) => c.into(),
			MathmlTag::Sum(c) => c.into(),
			MathmlTag::Tendsto(c) => c.into(),
			MathmlTag::Times(c) => c.into(),
			MathmlTag::Transpose(c) => c.into(),
			MathmlTag::Union(c) => c.into(),
			MathmlTag::Uplimit(c) => c.into(),
			MathmlTag::Variance(c) => c.into(),
			MathmlTag::Vector(c) => c.into(),
			MathmlTag::Vectorproduct(c) => c.into(),
			MathmlTag::Xo(c) => c.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Tag, 20);
		assert_size!(HtmlTag, 16);
		assert_size!(SvgTag, 16);
		assert_size!(MathmlTag, 16);
		assert_size!(CustomElementTag, 12);
		assert_size!(HtmlNonConformingTag, 16);
		assert_size!(HtmlNonStandardTag, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Tag, "div");
	}
}
