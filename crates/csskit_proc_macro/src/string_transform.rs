pub fn kebab(str: String) -> String {
	let mut kebab = String::new();
	for (i, ch) in str.char_indices() {
		if i > 0 && ch.is_uppercase() {
			kebab.push('-');
		}
		kebab.push(ch.to_ascii_lowercase());
	}
	kebab
}

pub fn pascal(str: String) -> String {
	let mut pascal = String::new();
	let mut make_upper = true;
	for ch in str.chars() {
		if ch == '_' || ch == '-' {
			make_upper = true;
			continue;
		}
		if make_upper {
			pascal.push(ch.to_ascii_uppercase());
			make_upper = false;
		} else {
			pascal.push(ch);
		}
	}
	pascal
}
