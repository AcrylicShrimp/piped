use super::super::super::value::ValueType;

define_function!(Typeof(_execution, argument_vec) => {
	if argument_vec.len() != 1 {
		panic!("1 argument required, got {}.", argument_vec.len())
	}

	Value::String {
		0: match argument_vec[0].value_type() {
			ValueType::Array => "array",
			ValueType::Dictionary => "dictionary",
			ValueType::Bool => "bool",
			ValueType::Integer => "integer",
			ValueType::String => "string",
		}.to_owned()
	}
});
