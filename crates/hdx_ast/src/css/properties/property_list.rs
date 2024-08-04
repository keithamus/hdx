macro_rules! apply_properties {
	($macro: ident) => {
		$crate::css::values::align::properties! {$macro}
		// $crate::css::values::anchor_position::properties! {$macro}
		// $crate::css::values::animations::properties! {$macro}
		// $crate::css::values::backgrounds::properties! {$macro}
		// $crate::css::values::borders::properties! {$macro}
		// $crate::css::values::r#box::properties! {$macro}
		// $crate::css::values::r#break::properties! {$macro}
		// $crate::css::values::cascade::properties! {$macro}
		// $crate::css::values::color::properties! {$macro}
		// $crate::css::values::color_adjust::properties! {$macro}
		// $crate::css::values::color_hdr::properties! {$macro}
		// $crate::css::values::conditional::properties! {$macro}
		// $crate::css::values::contain::properties! {$macro}
		// $crate::css::values::content::properties! {$macro}
		// $crate::css::values::display::properties! {$macro}
		// $crate::css::values::exclusions::properties! {$macro}
		// $crate::css::values::flexbox::properties! {$macro}
		// $crate::css::values::fonts::properties! {$macro}
		// $crate::css::values::gcpm::properties! {$macro}
		// $crate::css::values::grid::properties! {$macro}
		// $crate::css::values::images::properties! {$macro}
		// $crate::css::values::inline::properties! {$macro}
		// $crate::css::values::line_grid::properties! {$macro}
		// $crate::css::values::link_params::properties! {$macro}
		// $crate::css::values::lists::properties! {$macro}
		// $crate::css::values::logical::properties! {$macro}
		// $crate::css::values::multicol::properties! {$macro}
		// $crate::css::values::nav::properties! {$macro}
		// $crate::css::values::overflow::properties! {$macro}
		// $crate::css::values::overscroll::properties! {$macro}
		// $crate::css::values::page::properties! {$macro}
		// $crate::css::values::page_floats::properties! {$macro}
		// $crate::css::values::position::properties! {$macro}
		// $crate::css::values::regions::properties! {$macro}
		// $crate::css::values::rhythm::properties! {$macro}
		// $crate::css::values::round_display::properties! {$macro}
		// $crate::css::values::ruby::properties! {$macro}
		// $crate::css::values::scroll_anchoring::properties! {$macro}
		// $crate::css::values::scroll_animations::properties! {$macro}
		// $crate::css::values::scroll_snap::properties! {$macro}
		// $crate::css::values::scrollbars::properties! {$macro}
		// $crate::css::values::shapes::properties! {$macro}
		// $crate::css::values::size_adjust::properties! {$macro}
		// $crate::css::values::sizing::properties! {$macro}
		// $crate::css::values::speech::properties! {$macro}
		// $crate::css::values::tables::properties! {$macro}
		// $crate::css::values::text::properties! {$macro}
		// $crate::css::values::text_decor::properties! {$macro}
		// $crate::css::values::transforms::properties! {$macro}
		// $crate::css::values::transitions::properties! {$macro}
		$crate::css::values::ui::properties! {$macro}
		// $crate::css::values::values::properties! {$macro}
		// $crate::css::values::variables::properties! {$macro}
		// $crate::css::values::view_transitions::properties! {$macro}
		// $crate::css::values::viewport::properties! {$macro}
		// $crate::css::values::will_change::properties! {$macro}
		// $crate::css::values::writing_modes::properties! {$macro}
	};
}

pub(crate) use apply_properties;
