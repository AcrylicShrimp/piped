use super::super::super::function::Function;
use super::super::super::sub_execution::SubExecution;
use super::super::super::value::Value;
use std::path::PathBuf;

define_function!(JoinPath(_execution, argument_vec) => {
	let mut path = PathBuf::new();

	for argument in argument_vec.into_iter() {
		match argument.to_strict::<String>() {
			Some(string_value) => {
				path.push(string_value);
			}
			None => panic!("string is required")
		}
	}

	Value::String(path.to_str().unwrap().to_owned())
});
