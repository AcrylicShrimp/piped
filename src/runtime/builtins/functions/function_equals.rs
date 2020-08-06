use super::super::super::function::Function;
use super::super::super::sub_execution::SubExecution;
use super::super::super::value::{compare_value, Value};

define_function!(Equals(_execution, argument_vec) => {
	if argument_vec.len() != 2 {
		panic!("2 argument required, got {}.", argument_vec.len())
	}

	Value::Bool {
		0: compare_value(&argument_vec[0], &argument_vec[1])
	}
});
