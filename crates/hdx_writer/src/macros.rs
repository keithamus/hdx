#[macro_export]
macro_rules! write_css {
    ($sink: ident, $($a: expr),+) => {
		{
		$($a.write_css($sink)?;)*
		}
    };
}

#[macro_export]
macro_rules! write_list {
	($sink: ident, $list: expr,) => {
		let mut iter = $list.iter().peekable();
		while let Some(item) = iter.next() {
			item.write_css($sink)?;
			if iter.peek().is_some() {
				$sink.write_char(',')?;
				$sink.write_whitespace()?;
			}
		}
	};
}
