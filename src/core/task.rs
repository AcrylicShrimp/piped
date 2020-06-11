use super::pipeline::Pipeline;
use super::value::Value;
use std::collections::HashMap;

pub struct Task<'pipeline> {
	name: String,
	desc: Option<String>,
	pipeline: &'pipeline Pipeline,
	values: HashMap<String, Value>,
}

impl<'pipeline> Task<'pipeline> {
	pub fn new(name: String, desc: Option<String>, pipeline: &'pipeline Pipeline) -> Task {
		Task {
			name,
			desc,
			pipeline,
			values: HashMap::new(),
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn desc(&self) -> Option<&String> {
		self.desc.as_ref()
	}

	pub fn pipeline(&self) -> &Pipeline {
		&self.pipeline
	}

	pub fn value(&self, name: &str) -> Option<&Value> {
		self.values.get(name)
	}

	pub fn add_value(&mut self, name: String, value: Value) {
		match self.pipeline.attribute(&name) {
			Some(attribute) => {
				if value.get_type() != attribute.value_type() {
					panic!("the given value {:?} is not compatible with the attribute {:?} which is a {:?} type", value, name, attribute.value_type());
				}

				self.values.insert(name, value);
			}
			None => panic!(
				"the pipeline {:?} used by the task {:?} has no attribute {:?}",
				self.pipeline.name(),
				self.name,
				name
			),
		}
	}

	pub fn execute(&self) {
		self.pipeline.execute(&self.values);
	}
}
