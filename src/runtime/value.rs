use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub enum Value {
	Array(Vec<Value>),
	Dictionary(HashMap<String, Value>),
	Bool(bool),
	Integer(i64),
	String(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueType {
	Array,
	Dictionary,
	Bool,
	Integer,
	String,
}

impl Value {
	pub fn value_type(&self) -> ValueType {
		match self {
			Value::Array(..) => ValueType::Array,
			Value::Dictionary(..) => ValueType::Dictionary,
			Value::Bool(..) => ValueType::Bool,
			Value::Integer(..) => ValueType::Integer,
			Value::String(..) => ValueType::String,
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Value::Array(array) => {
				write!(f, "[")?;
				if !array.is_empty() {
					write!(f, "{}", array.first().unwrap())?;
				}
				for index in 1..array.len() {
					write!(f, ", {}", array[index])?;
				}
				write!(f, "]")
			}
			Value::Dictionary(dictionary) => {
				write!(f, "{{")?;
				let mut count = 0;
				for (key, value) in dictionary.iter() {
					if count != 0 {
						write!(f, ", ")?;
					}
					write!(f, "\"{}\": {}", key, value)?;
					count += 1;
				}
				write!(f, "}}")
			}
			Value::Bool(bool_value) => write!(f, "{}", bool_value),
			Value::Integer(integer_value) => write!(f, "{}", integer_value),
			Value::String(string_value) => write!(f, "{}", string_value),
		}
	}
}
