use hdx_atom::{atom, Atom, Atomizable};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "lowercase"))]
pub enum DimensionUnit {
	#[default]
	Unknown = 0,
	Cap,
	Ch,
	Cm,
	Cqb,
	Cqh,
	Cqi,
	Cqmax,
	Cqmin,
	Cqw,
	Deg,
	Dpcm,
	Dpi,
	Dppx,
	Dvb,
	Dvh,
	Dvi,
	Dvmax,
	Dvmin,
	Dvw,
	Em,
	Ex,
	Fr,
	Grad,
	Hz,
	Ic,
	In,
	Khz,
	Lh,
	Lvb,
	Lvh,
	Lvi,
	Lvmax,
	Lvmin,
	Lvw,
	Mm,
	Ms,
	Pc,
	#[cfg_attr(feature = "serde", serde(rename = "%"))]
	Percent,
	Pt,
	Px,
	Q,
	Rad,
	Rcap,
	Rch,
	Rem,
	Rex,
	Ric,
	Rlh,
	S,
	Svb,
	Svh,
	Svi,
	Svmax,
	Svmin,
	Svw,
	Turn,
	Vb,
	Vh,
	Vi,
	Vmax,
	Vmin,
	Vw,
	X,
}

impl DimensionUnit {
	pub fn is_empty(&self) -> bool {
		self == &Self::Unknown
	}

	pub fn len(&self) -> u32 {
		match self {
			Self::Unknown => 0,
			Self::Percent | Self::Q | Self::S | Self::X => 1,
			Self::Ch
			| Self::Cm
			| Self::Em
			| Self::Ex
			| Self::Fr
			| Self::Hz
			| Self::Ic
			| Self::In
			| Self::Lh
			| Self::Mm
			| Self::Ms
			| Self::Pc
			| Self::Pt
			| Self::Px
			| Self::Vb
			| Self::Vh
			| Self::Vi
			| Self::Vw => 2,
			Self::Cap
			| Self::Cqb
			| Self::Cqh
			| Self::Cqi
			| Self::Cqw
			| Self::Deg
			| Self::Dpi
			| Self::Dvb
			| Self::Dvi
			| Self::Dvh
			| Self::Dvw
			| Self::Khz
			| Self::Lvb
			| Self::Lvi
			| Self::Lvh
			| Self::Lvw
			| Self::Rad
			| Self::Rch
			| Self::Rem
			| Self::Rex
			| Self::Ric
			| Self::Rlh
			| Self::Svb
			| Self::Svi
			| Self::Svh
			| Self::Svw => 3,
			Self::Dpcm | Self::Dppx | Self::Grad | Self::Rcap | Self::Turn | Self::Vmax | Self::Vmin => 4,
			Self::Dvmax
			| Self::Dvmin
			| Self::Lvmax
			| Self::Lvmin
			| Self::Svmax
			| Self::Svmin
			| Self::Cqmax
			| Self::Cqmin => 5,
		}
	}
}

impl From<u8> for DimensionUnit {
	fn from(value: u8) -> Self {
		let unit = match value {
			1 => Self::Cap,
			2 => Self::Ch,
			3 => Self::Cm,
			4 => Self::Cqb,
			5 => Self::Cqh,
			6 => Self::Cqi,
			7 => Self::Cqmax,
			8 => Self::Cqmin,
			9 => Self::Cqw,
			10 => Self::Deg,
			11 => Self::Dpcm,
			12 => Self::Dpi,
			13 => Self::Dppx,
			14 => Self::Dvb,
			15 => Self::Dvh,
			16 => Self::Dvi,
			17 => Self::Dvmax,
			18 => Self::Dvmin,
			19 => Self::Dvw,
			20 => Self::Em,
			21 => Self::Ex,
			22 => Self::Fr,
			23 => Self::Grad,
			24 => Self::Hz,
			25 => Self::Ic,
			26 => Self::In,
			27 => Self::Khz,
			28 => Self::Lh,
			29 => Self::Lvb,
			30 => Self::Lvh,
			31 => Self::Lvi,
			32 => Self::Lvmax,
			33 => Self::Lvmin,
			34 => Self::Lvw,
			35 => Self::Mm,
			36 => Self::Ms,
			37 => Self::Pc,
			38 => Self::Percent,
			39 => Self::Pt,
			40 => Self::Px,
			41 => Self::Q,
			42 => Self::Rad,
			43 => Self::Rcap,
			44 => Self::Rch,
			45 => Self::Rem,
			46 => Self::Rex,
			47 => Self::Ric,
			48 => Self::Rlh,
			49 => Self::S,
			50 => Self::Svb,
			51 => Self::Svh,
			52 => Self::Svi,
			53 => Self::Svmax,
			54 => Self::Svmin,
			55 => Self::Svw,
			56 => Self::Turn,
			57 => Self::Vb,
			58 => Self::Vh,
			59 => Self::Vi,
			60 => Self::Vmax,
			61 => Self::Vmin,
			62 => Self::Vw,
			63 => Self::X,
			_ => Self::Unknown,
		};
		debug_assert!(unit as u8 == value, "{:#010b} != {:#010b} ({:?})", unit as u8, value, unit);
		unit
	}
}

impl Atomizable for DimensionUnit {
	fn from_atom(atom: &Atom) -> Option<Self> {
		match atom.to_ascii_lowercase() {
			atom!("cap") => Some(Self::Cap),
			atom!("ch") => Some(Self::Ch),
			atom!("cm") => Some(Self::Cm),
			atom!("cqb") => Some(Self::Cqb),
			atom!("cqh") => Some(Self::Cqh),
			atom!("cqi") => Some(Self::Cqi),
			atom!("cqmax") => Some(Self::Cqmax),
			atom!("cqmin") => Some(Self::Cqmin),
			atom!("cqw") => Some(Self::Cqw),
			atom!("deg") => Some(Self::Deg),
			atom!("dpcm") => Some(Self::Dpcm),
			atom!("dpi") => Some(Self::Dpi),
			atom!("dppx") => Some(Self::Dppx),
			atom!("dvh") => Some(Self::Dvh),
			atom!("dvw") => Some(Self::Dvw),
			atom!("em") => Some(Self::Em),
			atom!("ex") => Some(Self::Ex),
			atom!("fr") => Some(Self::Fr),
			atom!("grad") => Some(Self::Grad),
			atom!("hz") => Some(Self::Hz),
			atom!("ic") => Some(Self::Ic),
			atom!("in") => Some(Self::In),
			atom!("khz") => Some(Self::Khz),
			atom!("lh") => Some(Self::Lh),
			atom!("lvh") => Some(Self::Lvh),
			atom!("lvw") => Some(Self::Lvw),
			atom!("mm") => Some(Self::Mm),
			atom!("ms") => Some(Self::Ms),
			atom!("pc") => Some(Self::Pc),
			atom!("%") => Some(Self::Percent),
			atom!("pt") => Some(Self::Pt),
			atom!("px") => Some(Self::Px),
			atom!("q") => Some(Self::Q),
			atom!("rad") => Some(Self::Rad),
			atom!("rcap") => Some(Self::Rcap),
			atom!("rch") => Some(Self::Rch),
			atom!("rem") => Some(Self::Rem),
			atom!("rex") => Some(Self::Rex),
			atom!("ric") => Some(Self::Ric),
			atom!("rlh") => Some(Self::Rlh),
			atom!("s") => Some(Self::S),
			atom!("svh") => Some(Self::Svh),
			atom!("svw") => Some(Self::Svw),
			atom!("turn") => Some(Self::Turn),
			atom!("vb") => Some(Self::Vb),
			atom!("vh") => Some(Self::Vh),
			atom!("vi") => Some(Self::Vi),
			atom!("vmax") => Some(Self::Vmax),
			atom!("vmin") => Some(Self::Vmin),
			atom!("vw") => Some(Self::Vw),
			atom!("x") => Some(Self::X),
			_ => None,
		}
	}

	fn to_atom(&self) -> Atom {
		match self {
			Self::Unknown => atom!(""),
			Self::Cap => atom!("cap"),
			Self::Ch => atom!("ch"),
			Self::Cm => atom!("cm"),
			Self::Cqb => atom!("cqb"),
			Self::Cqh => atom!("cqh"),
			Self::Cqi => atom!("cqi"),
			Self::Cqmax => atom!("cqmax"),
			Self::Cqmin => atom!("cqmin"),
			Self::Cqw => atom!("cqw"),
			Self::Deg => atom!("deg"),
			Self::Dpcm => atom!("dpcm"),
			Self::Dpi => atom!("dpi"),
			Self::Dppx => atom!("dppx"),
			Self::Dvb => atom!("dvb"),
			Self::Dvh => atom!("dvh"),
			Self::Dvi => atom!("dvi"),
			Self::Dvmax => atom!("dvmax"),
			Self::Dvmin => atom!("dvmin"),
			Self::Dvw => atom!("dvw"),
			Self::Em => atom!("em"),
			Self::Ex => atom!("ex"),
			Self::Fr => atom!("fr"),
			Self::Grad => atom!("grad"),
			Self::Hz => atom!("hz"),
			Self::Ic => atom!("ic"),
			Self::In => atom!("in"),
			Self::Khz => atom!("khz"),
			Self::Lh => atom!("lh"),
			Self::Lvb => atom!("lvb"),
			Self::Lvh => atom!("lvh"),
			Self::Lvi => atom!("lvi"),
			Self::Lvmax => atom!("lvmax"),
			Self::Lvmin => atom!("lvmin"),
			Self::Lvw => atom!("lvw"),
			Self::Mm => atom!("mm"),
			Self::Ms => atom!("ms"),
			Self::Pc => atom!("pc"),
			Self::Percent => atom!("%"),
			Self::Pt => atom!("pt"),
			Self::Px => atom!("px"),
			Self::Q => atom!("q"),
			Self::Rad => atom!("rad"),
			Self::Rcap => atom!("rcap"),
			Self::Rch => atom!("rch"),
			Self::Rem => atom!("rem"),
			Self::Rex => atom!("rex"),
			Self::Ric => atom!("ric"),
			Self::Rlh => atom!("rlh"),
			Self::S => atom!("s"),
			Self::Svb => atom!("svb"),
			Self::Svh => atom!("svh"),
			Self::Svi => atom!("svi"),
			Self::Svmax => atom!("svmin"),
			Self::Svmin => atom!("svmax"),
			Self::Svw => atom!("svw"),
			Self::Turn => atom!("turn"),
			Self::Vb => atom!("vb"),
			Self::Vh => atom!("vh"),
			Self::Vi => atom!("vi"),
			Self::Vmax => atom!("vmax"),
			Self::Vmin => atom!("vmin"),
			Self::Vw => atom!("vw"),
			Self::X => atom!("x"),
		}
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<DimensionUnit>(), 1);
}
