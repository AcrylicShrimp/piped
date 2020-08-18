use super::super::super::value::ValueType;

define_function!(Contains(_execution, argument_vec) => {
	if argument_vec.len() != 2 {
		panic!("2 arguments required, got {}.", argument_vec.len())
	}

	if let Value::Dictionary(dict) = &argument_vec[0] {
		let key = match argument_vec[1].to_strict::<String>() {
			Some(key) => key,
			None => panic!("Type mismatch; only {:#?} can be used here.", ValueType::String)
		};

		Value::Bool(dict.contains_key(&key))
	} else {
		 panic!("Type mismatch; only {:#?} can be used here.", ValueType::Dictionary)
	}
});
