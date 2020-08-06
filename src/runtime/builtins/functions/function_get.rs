use super::super::super::function::Function;
use super::super::super::sub_execution::SubExecution;
use super::super::super::value::{Value, ValueType};

define_function!(Get(_execution, argument_vec) => {
	if argument_vec.len() != 2 {
		panic!("2 arguments required, got {}.", argument_vec.len())
	}

	if let Value::Array(array) = &argument_vec[0] {
		let mut index = match argument_vec[1].to_strict::<i64>() {
			Some(index) => index,
			None => panic!("Type mismatch; only {:#?} can be used here.", ValueType::Integer)
		};

		if index < 0 {
			index += array.len() as i64
		}

		if index < 0 || array.len() as i64 <= index {
			panic!("Out of index.")
		}

		array[index as usize].clone()
	} else if let Value::Dictionary(dict) = &argument_vec[0] {
		let key = match argument_vec[1].to_strict::<String>() {
			Some(key) => key,
			None => panic!("Type mismatch; only {:#?} can be used here.", ValueType::String)
		};

		match dict.get(&key) {
			Some(value) => value.clone(),
			None => panic!("Unable to find key \"{}\".", key)
		}
	} else {
		 panic!("Type mismatch; only {:#?} or {:#?} can be used here.", ValueType::Array, ValueType::Dictionary)
	}
});
