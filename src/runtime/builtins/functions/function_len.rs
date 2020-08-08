use super::super::super::value::ValueType;

define_function!(Len(_execution, argument_vec) => {
	if argument_vec.len() != 1 {
		panic!("1 argument required, got {}.", argument_vec.len())
	}

	if let Value::Array(array) = &argument_vec[0] {
		Value::Integer{0: array.len() as i64}
	} else if let Value::Dictionary(dict) = &argument_vec[0] {
		Value::Integer{0: dict.len() as i64}
	} else {
		 panic!("Type mismatch; only {:#?} or {:#?} can be used here.", ValueType::Array, ValueType::Dictionary)
	}
});
