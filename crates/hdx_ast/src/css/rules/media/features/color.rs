use crate::css::units::CSSInt;
use hdx_parser::ranged_feature;

ranged_feature!(ColorMediaFeature[atom!("color")], CSSInt);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ColorMediaFeature, 68);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorMediaFeature, "color:2");
		assert_parse!(ColorMediaFeature, "color:8");
		assert_parse!(ColorMediaFeature, "min-color:2");
		assert_parse!(ColorMediaFeature, "max-color:2");
		assert_parse!(ColorMediaFeature, "color<=3");
		assert_parse!(ColorMediaFeature, "color>=5");
		assert_parse!(ColorMediaFeature, "color>=8");
		assert_parse!(ColorMediaFeature, "color=16");
		assert_parse!(ColorMediaFeature, "6=color");
		assert_parse!(ColorMediaFeature, "2<=color");
		assert_parse!(ColorMediaFeature, "2<color<4");
		assert_parse!(ColorMediaFeature, "4>color<8");
		assert_parse!(ColorMediaFeature, "4>=color<=8");
		assert_parse!(ColorMediaFeature, "4<=color>8");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ColorMediaFeature, "color:");
		assert_parse_error!(ColorMediaFeature, "color: > 10px");
		assert_parse_error!(ColorMediaFeature, "max-color > 10px");
		assert_parse_error!(ColorMediaFeature, "min-color > 10px");
		assert_parse_error!(ColorMediaFeature, "color: 1px");
		assert_parse_error!(ColorMediaFeature, "color: red");
		assert_parse_error!(ColorMediaFeature, "pointer: 1");
	}
}
