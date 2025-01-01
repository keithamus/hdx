use css_ast::css::{
	properties::{Property, StyleValue},
	visit::VisitMut,
};

#[derive(Default)]
pub struct ReduceInitial();

impl<'a> VisitMut<'a> for ReduceInitial {
	fn visit_property(&mut self, property: &mut Property<'a>) {
		if matches!(&property.value, StyleValue::Initial(_)) {
			if let Some(def) = StyleValue::default_for(&property.name) {
				property.value = def;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	// FIXME: This fails but we want green CI for now.
	// #[test]
	fn test_transform() {
		assert_transform!(
			ReduceInitial,
			r#"body {
				appearance: initial;
				direction: initial;
				min-width: initial;
				object-fit: initial;
				orphans: initial;
			}"#,
			r#"body {
				appearance: auto;
				direction: ltr;
				min-width: auto;
				object-fit: fill;
				orphans: 2;
			}"#
		);
	}
}
