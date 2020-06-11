use super::pipeline::Pipeline;
use super::value::Value;
use ron::de::from_str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Task<'pipeline> {
	name: String,
	desc: Option<String>,
	pipeline: &'pipeline Pipeline,
	values: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "Task")]
pub struct TaskDefinition {
	name: String,
	desc: Option<String>,
	pipeline: String,
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

	pub fn load(content: &str, pipelines: &'pipeline HashMap<String, Pipeline>) -> Task<'pipeline> {
		let task_definition: TaskDefinition = from_str(content).unwrap();
		let mut task = Task::new(
			task_definition.name.clone(),
			task_definition.desc,
			pipelines.get(&task_definition.pipeline).unwrap(),
		);

		for (name, value) in task_definition.values.into_iter() {
			task.add_value(name, value);
		}

		task
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
