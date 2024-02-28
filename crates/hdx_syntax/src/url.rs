pub fn is_non_printable(c: char) -> bool {
	matches!(c, '\x00'..='\x08' | '\x0B' | '\x0E'..='\x1F' | '\x7F')
}

pub fn is_url_ident(ident: &str) -> bool {
	ident.to_ascii_lowercase() == "url"
}
