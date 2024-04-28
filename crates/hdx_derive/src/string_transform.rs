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

pub fn snake(str: String) -> String {
	let mut snake = String::new();
	for (i, ch) in str.char_indices() {
		if i > 0 && ch.is_uppercase() {
			snake.push('_');
		}
		snake.push(ch.to_ascii_lowercase());
	}
	snake
}
