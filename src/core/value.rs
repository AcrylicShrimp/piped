use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Value {
	Str(String),
	StrList(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum ValueType {
	Str,
	StrList,
}

impl Value {
	pub fn get_type(&self) -> ValueType {
		match self {
			Value::Str(..) => ValueType::Str,
			Value::StrList(..) => ValueType::StrList,
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

	pub fn unwrap_strlist(self) -> Vec<String> {
		match self {
			Value::StrList(value) => value,
			_ => panic!(
				"{:?} expected, got {:?}",
				ValueType::StrList,
				self.get_type()
			),
		}
	}

	pub fn unwrap_strlist_ref(&self) -> &Vec<String> {
		match self {
			Value::StrList(value) => value,
			_ => panic!(
				"{:?} expected, got {:?}",
				ValueType::StrList,
				self.get_type()
			),
		}
	}

	pub fn unwrap_strlist_mut(&mut self) -> &mut Vec<String> {
		match self {
			Value::StrList(value) => value,
			_ => panic!(
				"{:?} expected, got {:?}",
				ValueType::StrList,
				self.get_type()
			),
		}
	}
}
