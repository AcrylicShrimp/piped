use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Value {
	Str(String),
	StrList(Vec<String>),
	PathList(Vec<PathBuf>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum ValueType {
	Str,
	StrList,
	PathList,
}

impl Value {
	pub fn get_type(&self) -> ValueType {
		match self {
			Value::Str(..) => ValueType::Str,
			Value::StrList(..) => ValueType::StrList,
			Value::PathList(..) => ValueType::PathList,
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

	pub fn unwrap_pathlist(self) -> Vec<PathBuf> {
		match self {
			Value::PathList(value) => value,
			_ => panic!(
				"{:?} expected, got {:?}",
				ValueType::PathList,
				self.get_type()
			),
		}
	}

	pub fn unwrap_pathlist_ref(&self) -> &Vec<PathBuf> {
		match self {
			Value::PathList(value) => value,
			_ => panic!(
				"{:?} expected, got {:?}",
				ValueType::PathList,
				self.get_type()
			),
		}
	}

	pub fn unwrap_pathlist_mut(&mut self) -> &mut Vec<PathBuf> {
		match self {
			Value::PathList(value) => value,
			_ => panic!(
				"{:?} expected, got {:?}",
				ValueType::PathList,
				self.get_type()
			),
		}
	}
}
