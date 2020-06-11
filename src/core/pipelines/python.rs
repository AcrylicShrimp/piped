use super::super::attribute::Attribute;
use super::super::pipeline::Pipeline;
use super::super::value::{Value, ValueType};
use pyo3::{
	prelude::*,
	types::{PyDict, PyModule, PyString},
};
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};

pub fn load_all() -> HashMap<String, Pipeline> {
	let mut pipelines = HashMap::new();

	for entry in read_dir("pipelines").unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();

		if path.is_dir() {
			continue;
		}

		let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
		let pipeline = load_file(&read_to_string(path).unwrap(), &filename);

		pipelines.insert(pipeline.name().clone(), pipeline);
	}

	pipelines
}

pub fn load_file(code: &str, filename: &str) -> Pipeline {
	let gil = Python::acquire_gil();
	let python = gil.python();
	let functions = PyModule::from_code(python, code, filename, "pipelines").unwrap();

	let raw_name = functions.call("name", (), None);
	let name;

	match raw_name {
		Ok(raw_name) => match raw_name.downcast::<PyString>() {
			Ok(string) => name = string.to_string_lossy().as_ref().to_owned(),
			Err(..) => panic!("the name function must return a string"),
		},
		Err(err) => {
			err.print_and_set_sys_last_vars(python);
			panic!();
		}
	}

	let raw_desc = functions.call("desc", (), None);
	let mut desc = None;

	match raw_desc {
		Ok(raw_desc) => match raw_desc.downcast::<PyString>() {
			Ok(string) => desc = Some(string.to_string_lossy().as_ref().to_owned()),
			Err(..) => panic!("the desc function must return a string"),
		},
		Err(..) => {}
	}

	let raw_attributes = functions.call("attributes", (), None);
	let mut attributes = HashMap::new();

	match raw_attributes {
		Ok(raw_attributes) => match raw_attributes.downcast::<PyDict>() {
			Ok(dict) => {
				for (name, attribute) in dict.iter() {
					let dict = match attribute.downcast::<PyDict>() {
						Ok(dict) => dict,
						Err(..) => panic!("a item of a attribute must be a dict"),
					};
					let name = match name.downcast::<PyString>() {
						Ok(string) => string.to_string_lossy().as_ref().to_owned(),
						Err(..) => panic!("the name of a attribute must be a string"),
					};
					let desc = match dict.get_item("desc") {
						Some(desc) => match desc.downcast::<PyString>() {
							Ok(string) => Some(string.to_string_lossy().as_ref().to_owned()),
							Err(..) => panic!("the desc of a attribute must be a string"),
						},
						None => None,
					};
					let value_type =
						match dict.get_item("value_type").unwrap().downcast::<PyString>() {
							Ok(string) => match string.to_string_lossy().as_ref() {
								"Str" => ValueType::Str,
								_ => panic!("malformed attribute value type"),
							},
							Err(..) => panic!("the name of a attribute must be a string"),
						};
					let default_value = match dict.get_item("default_type") {
						Some(default_value) => match value_type {
							ValueType::Str => match default_value.downcast::<PyString>() {
								Ok(string) => {
									Some(Value::Str(string.to_string_lossy().as_ref().to_owned()))
								}
								Err(..) => panic!(
									"the default value of the attribute {:?} must be a string",
									name
								),
							},
						},
						None => None,
					};
					let attribute = Attribute::new(name.clone(), desc, value_type, default_value);
					attributes.insert(name, attribute);
				}
			}
			Err(..) => panic!("the attributes function must return a dict"),
		},
		Err(err) => {
			err.print_and_set_sys_last_vars(python);
			panic!();
		}
	}

	let code = code.to_owned();
	let filename = filename.to_owned();

	let mut pipeline = Pipeline::new(
		name,
		desc,
		Box::new(move |attributes, values| {
			let gil = Python::acquire_gil();
			let python = gil.python();
			let functions = PyModule::from_code(python, &code, &filename, "pipelines").unwrap();

			let py_values = PyDict::new(python);

			for (name, value) in values.iter() {
				py_values
					.set_item(
						name,
						match value {
							Value::Str(string) => string,
						},
					)
					.unwrap();
			}

			functions.call("execute", (py_values,), None).unwrap();
		}),
	);

	for (_name, attribute) in attributes.into_iter() {
		pipeline.add_attribute(attribute);
	}

	pipeline
}
