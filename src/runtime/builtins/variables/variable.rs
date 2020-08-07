use super::super::super::value::Value;
use super::{variable_host_arch, variable_host_os};
use std::collections::HashMap;

macro_rules! define_variable {
	($name:literal, $body:expr) => {
		use super::super::super::value::Value;

		pub fn eval_variable() -> (&'static str, Value) {
			($name, $body)
		}
	};
}

macro_rules! insert_variable {
	($variable:ident >>> $variable_map:ident) => {
		let varible = $variable::eval_variable();
		$variable_map.insert(varible.0.to_owned(), varible.1);
	};
}

pub fn build_variable_map() -> HashMap<String, Value> {
	let mut variable_map: HashMap<_, Value> = HashMap::new();

	insert_variable!(variable_host_arch >>> variable_map);
	insert_variable!(variable_host_os >>> variable_map);

	variable_map
}
