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

pub trait FromValue: Sized {
	fn from_value(value: &Value) -> Option<Self>;
}

impl<T: FromValue> FromValue for Vec<T> {
	fn from_value(value: &Value) -> Option<Self> {
		match value {
			Value::Array(value_vec) => Some(
				value_vec
					.iter()
					.map(|element| element.to_strict::<T>().unwrap())
					.collect(),
			),
			_ => None,
		}
	}
}

impl<T: FromValue> FromValue for HashMap<String, T> {
	fn from_value(value: &Value) -> Option<Self> {
		match value {
			Value::Dictionary(value_map) => Some(
				value_map
					.iter()
					.map(|pair| (pair.0.clone(), pair.1.to_strict::<T>().unwrap()))
					.collect(),
			),
			_ => None,
		}
	}
}

impl FromValue for i64 {
	fn from_value(value: &Value) -> Option<Self> {
		match value {
			Value::Integer(integer_value) => Some(*integer_value),
			_ => None,
		}
	}
}

impl FromValue for String {
	fn from_value(value: &Value) -> Option<Self> {
		match value {
			Value::String(string_value) => Some(string_value.clone()),
			_ => None,
		}
	}
}

impl FromValue for bool {
	fn from_value(value: &Value) -> Option<Self> {
		match value {
			Value::Bool(bool_value) => Some(*bool_value),
			_ => None,
		}
	}
}

impl Value {
	pub fn to_strict<T: FromValue>(&self) -> Option<T> {
		T::from_value(self)
	}

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
