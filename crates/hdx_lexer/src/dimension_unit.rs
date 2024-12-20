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

impl From<DimensionUnit> for &'static str {
	fn from(value: DimensionUnit) -> &'static str {
		match value {
			DimensionUnit::Unknown => "",
			DimensionUnit::Cap => "cap",
			DimensionUnit::Ch => "ch",
			DimensionUnit::Cm => "cm",
			DimensionUnit::Cqb => "cqb",
			DimensionUnit::Cqh => "cqh",
			DimensionUnit::Cqi => "cqi",
			DimensionUnit::Cqmax => "cqmax",
			DimensionUnit::Cqmin => "cqmin",
			DimensionUnit::Cqw => "cqw",
			DimensionUnit::Deg => "deg",
			DimensionUnit::Dpcm => "dpcm",
			DimensionUnit::Dpi => "dpi",
			DimensionUnit::Dppx => "dppx",
			DimensionUnit::Dvb => "dvb",
			DimensionUnit::Dvh => "dvh",
			DimensionUnit::Dvi => "dvi",
			DimensionUnit::Dvmax => "dvmax",
			DimensionUnit::Dvmin => "dvmin",
			DimensionUnit::Dvw => "dvw",
			DimensionUnit::Em => "em",
			DimensionUnit::Ex => "ex",
			DimensionUnit::Fr => "fr",
			DimensionUnit::Grad => "grad",
			DimensionUnit::Hz => "hz",
			DimensionUnit::Ic => "ic",
			DimensionUnit::In => "in",
			DimensionUnit::Khz => "khz",
			DimensionUnit::Lh => "lh",
			DimensionUnit::Lvb => "lvb",
			DimensionUnit::Lvh => "lvh",
			DimensionUnit::Lvi => "lvi",
			DimensionUnit::Lvmax => "lvmax",
			DimensionUnit::Lvmin => "lvmin",
			DimensionUnit::Lvw => "lvw",
			DimensionUnit::Mm => "mm",
			DimensionUnit::Ms => "ms",
			DimensionUnit::Pc => "pc",
			DimensionUnit::Percent => "%",
			DimensionUnit::Pt => "pt",
			DimensionUnit::Px => "px",
			DimensionUnit::Q => "q",
			DimensionUnit::Rad => "rad",
			DimensionUnit::Rcap => "rcap",
			DimensionUnit::Rch => "rch",
			DimensionUnit::Rem => "rem",
			DimensionUnit::Rex => "rex",
			DimensionUnit::Ric => "ric",
			DimensionUnit::Rlh => "rlh",
			DimensionUnit::S => "s",
			DimensionUnit::Svb => "svb",
			DimensionUnit::Svh => "svh",
			DimensionUnit::Svi => "svi",
			DimensionUnit::Svmax => "svmin",
			DimensionUnit::Svmin => "svmax",
			DimensionUnit::Svw => "svw",
			DimensionUnit::Turn => "turn",
			DimensionUnit::Vb => "vb",
			DimensionUnit::Vh => "vh",
			DimensionUnit::Vi => "vi",
			DimensionUnit::Vmax => "vmax",
			DimensionUnit::Vmin => "vmin",
			DimensionUnit::Vw => "vw",
			DimensionUnit::X => "x",
		}
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<DimensionUnit>(), 1);
}
