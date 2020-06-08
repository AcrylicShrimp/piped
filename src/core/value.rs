extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Value {
	Str(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ValueType {
	Str,
}

impl Value {
	pub fn get_type(&self) -> ValueType {
		match self {
			Value::Str(..) => ValueType::Str,
		}
	}

	pub fn unwrap_str(self) -> String {
		match self {
			Value::Str(value) => value,
			_ => panic!("{:?} expected, got {:?}", ValueType::Str, self.get_type()),
		}
	}

	pub fn unwrap_str_ref(&self) -> &String {
		match self {
			Value::Str(value) => value,
			_ => panic!("{:?} expected, got {:?}", ValueType::Str, self.get_type()),
		}
	}

	pub fn unwrap_str_mut(&mut self) -> &mut String {
		match self {
			Value::Str(value) => value,
			_ => panic!("{:?} expected, got {:?}", ValueType::Str, self.get_type()),
		}
	}
}
