define_function!(IsExists(execution, argument_vec) => {
	if argument_vec.len() != 1 {
		panic!("1 argument required, got {}.", argument_vec.len())
	}

	Value::Bool {
		0: match argument_vec[0].to_strict::<String>() {
			Some(string_value) => {
				execution.get_variable(&string_value).is_some()
			}
			None => panic!("string is required")
		}
	}
});
