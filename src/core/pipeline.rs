use super::attribute::Attribute;
use super::value::Value;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Pipeline {
	name: String,
	desc: Option<String>,
	attributes: HashMap<String, Attribute>,

	#[serde(skip)]
	#[serde(default = "default_executor")]
	executor: Box<dyn Fn(&HashMap<String, Attribute>, &HashMap<String, Value>)>,
}

fn default_executor() -> Box<dyn Fn(&HashMap<String, Attribute>, &HashMap<String, Value>)> {
	Box::new(move |_attributes, _values| {})
}

impl Pipeline {
	pub fn new(
		name: String,
		desc: Option<String>,
		executor: Box<dyn Fn(&HashMap<String, Attribute>, &HashMap<String, Value>)>,
	) -> Pipeline {
		if !Regex::new("^[a-z][a-z\\d-]*[a-z\\d]?$")
			.unwrap()
			.is_match(&name)
		{
			panic!("malformed pipeline name");
		}

		Pipeline {
			name,
			desc,
			attributes: HashMap::new(),
			executor,
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

	pub fn add_attribute(&mut self, attribute: Attribute) {
		let name = attribute.name().clone();
		self.attributes.insert(name, attribute);
	}

	pub fn execute(&self, values: &HashMap<String, Value>) {
		let mut resolved_values = values.clone();

		for (name, attribute) in self.attributes.iter() {
			if !resolved_values.contains_key(name) {
				match attribute.default_value() {
					Some(default_value) => {
						resolved_values.insert(name.clone(), default_value.clone());
					}
					None => panic!(
						"failed to supply a value to the attribute {:?} of the pipeline {:?}",
						name, self.name,
					),
				}
			}
		}

		(*self.executor)(&self.attributes, &resolved_values);
	}
}
