use super::value::{Value, ValueType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Attribute {
	name: String,
	desc: Option<String>,
	value_type: ValueType,
	default_value: Option<Value>,
}

impl Attribute {
	pub fn new(
		name: String,
		desc: Option<String>,
		value_type: ValueType,
		default_value: Option<Value>,
	) -> Attribute {
		match &default_value {
			Some(default_value) => {
				if default_value.get_type() != value_type {
					panic!(
						"the given default value {:?} is not compatible with the attribute {:?} which is a {:?} type",
						default_value, name, value_type
					);
				}
			}
			None => {}
		}

		Attribute {
			name,
			desc,
			value_type,
			default_value,
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn desc(&self) -> Option<&String> {
		self.desc.as_ref()
	}

	pub fn value_type(&self) -> ValueType {
		self.value_type
	}

	pub fn default_value(&self) -> Option<&Value> {
		self.default_value.as_ref()
	}
}
