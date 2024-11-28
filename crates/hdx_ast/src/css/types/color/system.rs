use hdx_parser::keyword_typedef;

keyword_typedef!(SystemColor {
	AccentColor: atom!("accentcolor"),
	AccentColorText: atom!("accentcolortext"),
	ActiveText: atom!("activetext"),
	ButtonBorder: atom!("buttonborder"),
	ButtonFace: atom!("buttonface"),
	ButtonText: atom!("buttontext"),
	Canvas: atom!("canvas"),
	CanvasText: atom!("canvastext"),
	Field: atom!("field"),
	FieldText: atom!("fieldtext"),
	GrayText: atom!("graytext"),
	Highlight: atom!("highlight"),
	HighlightText: atom!("highlighttext"),
	LinkText: atom!("linktext"),
	Mark: atom!("mark"),
	MarkText: atom!("marktext"),
	SelectedItem: atom!("selecteditem"),
	SelectedItemText: atom!("selecteditemtext"),
	VisitedText: atom!("visitedtext"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(SystemColor, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SystemColor, "marktext");
		assert_parse!(SystemColor, "visitedtext");
		assert_parse!(SystemColor, "graytext");
	}
}
