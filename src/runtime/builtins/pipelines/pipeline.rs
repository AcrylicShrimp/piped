use super::super::super::pipeline::PipelineFactory;
use super::pipeline_exec;
use std::collections::HashMap;

macro_rules! define_pipeline {
	($name:ident, $argument_map:ident => $body:block) => {
		use super::super::super::pipeline::{PipelineExecution, PipelineExecutionResult};
		use super::super::super::value::Value;
		use std::collections::HashMap;

		pub struct $name {}

		impl $name {
			pub fn new($argument_map: &HashMap<String, Value>) -> Box<PipelineExecution> $body
		}
	};
}

macro_rules! insert_pipeline {
	($pipeline:ty, $pipeline_name:literal >>> $pipeline_map:ident) => {
		$pipeline_map.insert($pipeline_name.to_owned(), Box::new(<$pipeline>::new))
	};
}

pub fn build_pipeline_map() -> HashMap<String, Box<PipelineFactory>> {
	let mut pipeline_map: HashMap<_, Box<PipelineFactory>> = HashMap::new();

	insert_pipeline!(pipeline_exec::Exec, "exec" >>> pipeline_map);

	pipeline_map
}
