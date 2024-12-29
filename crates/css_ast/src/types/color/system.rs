use css_parse::keyword_set;

keyword_set!(SystemColor {
	AccentColor: "accentcolor",
	AccentColorText: "accentcolortext",
	ActiveText: "activetext",
	ButtonBorder: "buttonborder",
	ButtonFace: "buttonface",
	ButtonText: "buttontext",
	Canvas: "canvas",
	CanvasText: "canvastext",
	Field: "field",
	FieldText: "fieldtext",
	GrayText: "graytext",
	Highlight: "highlight",
	HighlightText: "highlighttext",
	LinkText: "linktext",
	Mark: "mark",
	MarkText: "marktext",
	SelectedItem: "selecteditem",
	SelectedItemText: "selecteditemtext",
	VisitedText: "visitedtext",
});

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SystemColor>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SystemColor, "marktext");
		assert_parse!(SystemColor, "visitedtext");
		assert_parse!(SystemColor, "graytext");
	}
}
