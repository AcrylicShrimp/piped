extern crate serde;

use super::attribute::Attribute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
	name: String,
	desc: Option<String>,
	attributes: HashMap<String, Attribute>,
}

impl Pipeline {
	pub fn new(name: String, desc: Option<String>) -> Pipeline {
		Pipeline {
			name,
			desc,
			attributes: HashMap::new(),
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn desc(&self) -> Option<&String> {
		self.desc.as_ref()
	}

	pub fn attribute(&self, name: &str) -> Option<&Attribute> {
		self.attributes.get(name)
	}

	pub fn add_attributes(&mut self, attribute: Attribute) {
		let name = attribute.name().clone();
		self.attributes.insert(name, attribute);
	}
}
