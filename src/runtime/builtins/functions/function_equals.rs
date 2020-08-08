use super::super::super::value::compare_value;

define_function!(Equals(_execution, argument_vec) => {
	if argument_vec.len() != 2 {
		panic!("2 argument required, got {}.", argument_vec.len())
	}

	Value::Bool {
		0: compare_value(&argument_vec[0], &argument_vec[1])
	}
});
