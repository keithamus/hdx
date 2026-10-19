use super::{GridTemplateColumnsStyleValue, GridTemplateRowsStyleValue};
use crate::{AutoTrackList, CssAtomSet, KeywordValue, LineNameList, TrackList, Value};
use css_parse::{Cursor, Parse, Parser, Result as ParserResult, T};

// `grid-template-columns`/`grid-template-rows` share `none | <track-list> | <auto-track-list> |
// subgrid <line-name-list>?`. The generated dispatch picks between `<track-list>` and
// `<auto-track-list>` using a single-token peek with no backtracking, but the two grammars share
// a valid prefix (both can start with a `<fixed-size>`-shaped track) and only diverge on whether
// a `repeat(auto-fill|auto-fit, ...)` shows up later - something a single-token peek can't see.
// Trying `<auto-track-list>` first via a checkpointing `try_parse` is safe (it can only succeed
// when a literal auto-repeat is present, which `<track-list>` can never contain) and falls back
// to `<track-list>` otherwise.
macro_rules! impl_grid_template_axis_parse {
	($ty:ident) => {
		impl<'a> Parse<'a> for $ty<'a> {
			fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
			where
				I: Iterator<Item = Cursor> + Clone,
			{
				if p.peek::<T![Ident]>() {
					let c = p.peek_n(1);
					if p.equals_atom(c.into(), &CssAtomSet::None) {
						return Ok(Self::None(p.parse::<KeywordValue<'a, T![Ident]>>()?));
					}
					if p.equals_atom(c.into(), &CssAtomSet::Subgrid) {
						let keyword = p.parse::<KeywordValue<'a, T![Ident]>>()?;
						let names = p.parse_if_peek::<Value<'a, LineNameList<'a>>>()?;
						return Ok(Self::Subgrid(keyword, names));
					}
					// Not `none`/`subgrid` - could still be a track keyword like `auto` or
					// `min-content`, so fall through to the track-list attempts below.
				}
				if let Ok(auto_track_list) = p.try_parse::<Value<'a, AutoTrackList<'a>>>() {
					return Ok(Self::AutoTrackList(auto_track_list));
				}
				Ok(Self::TrackList(p.parse::<Value<'a, TrackList<'a>>>()?))
			}
		}
	};
}

impl_grid_template_axis_parse!(GridTemplateColumnsStyleValue);
impl_grid_template_axis_parse!(GridTemplateRowsStyleValue);

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GridTemplateAreasStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateAreasStyleValue, r#""foo""bar""#);

		assert_parse!(CssAtomSet::ATOMS, FitToleranceStyleValue, "infinite");
		assert_parse!(CssAtomSet::ATOMS, FitToleranceStyleValue, "30px");
	}

	#[test]
	fn test_grid_template_columns_and_rows() {
		assert_parse!(CssAtomSet::ATOMS, GridTemplateColumnsStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateColumnsStyleValue, "100px 1fr");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateColumnsStyleValue, "[a] 100px [b] repeat(2,1fr) [c]");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateColumnsStyleValue, "repeat(auto-fill,minmax(100px,1fr))");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateColumnsStyleValue, "subgrid");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateColumnsStyleValue, "subgrid [a] [b c] repeat(2,[d])");

		assert_parse!(CssAtomSet::ATOMS, GridTemplateRowsStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateRowsStyleValue, "minmax(min-content,1fr) fit-content(20%)");

		assert_peek_false!(CssAtomSet::ATOMS, GridTemplateColumnsStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, GridTemplateColumnsStyleValue, "1fr repeat(auto-fill,1fr)");
	}

	#[test]
	fn test_grid_auto_columns_and_rows() {
		assert_parse!(CssAtomSet::ATOMS, GridAutoColumnsStyleValue, "min-content");
		assert_parse!(CssAtomSet::ATOMS, GridAutoColumnsStyleValue, "minmax(100px,1fr)");
		assert_parse!(CssAtomSet::ATOMS, GridAutoColumnsStyleValue, "fit-content(20%)");
		assert_parse!(CssAtomSet::ATOMS, GridAutoRowsStyleValue, "auto 1fr");
	}

	#[test]
	fn test_grid_auto_flow() {
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "row");
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "column");
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "dense");
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "row dense");
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "column dense");
		assert_peek_false!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "row column");
	}
}
