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
			Value::Array(value_vec) => value_vec
				.iter()
				.map(|element| element.to_strict::<T>())
				.collect::<Option<Vec<_>>>(),
			_ => None,
		}
	}
}

impl<T: FromValue> FromValue for HashMap<String, T> {
	fn from_value(value: &Value) -> Option<Self> {
		match value {
			Value::Dictionary(value_map) => value_map
				.iter()
				.map(|(key, value)| match value.to_strict::<T>() {
					Some(value) => Some((key.clone(), value)),
					None => None,
				})
				.collect::<Option<HashMap<_, _>>>(),
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

pub fn compare_value(left: &Value, right: &Value) -> bool {
	if left.value_type() != right.value_type() {
		return false;
	}

	match left {
		Value::Array(left_array) => {
			if let Value::Array(right_array) = right {
				if left_array.len() != right_array.len() {
					return false;
				}

				for index in 0..left_array.len() {
					if !compare_value(&left_array[index], &right_array[index]) {
						return true;
					}
				}

				true
			} else {
				unreachable!()
			}
		}
		Value::Dictionary(left_dictionary) => {
			if let Value::Dictionary(right_dictionary) = right {
				if left_dictionary.len() != right_dictionary.len() {
					return false;
				}

				for (key, left_value) in left_dictionary.iter() {
					match right_dictionary.get(key) {
						Some(right_value) => {
							if !compare_value(left_value, right_value) {
								return false;
							}
						}
						None => {
							return false;
						}
					}
				}

				true
			} else {
				unreachable!()
			}
		}
		Value::Bool(left_bool) => {
			if let Value::Bool(right_bool) = right {
				left_bool == right_bool
			} else {
				unreachable!()
			}
		}
		Value::Integer(left_integer) => {
			if let Value::Integer(right_integer) = right {
				left_integer == right_integer
			} else {
				unreachable!()
			}
		}
		Value::String(left_string) => {
			if let Value::String(right_string) = right {
				left_string == right_string
			} else {
				unreachable!()
			}
		}
	}
}
