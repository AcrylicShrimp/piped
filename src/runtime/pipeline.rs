use super::value::Value;
use std::collections::HashMap;

pub type PipelineFactory = dyn Fn(&HashMap<String, Value>) -> Box<dyn Pipeline>;

pub trait Pipeline {
	fn execute(&mut self);
	fn execute_background(&mut self);
	fn wait(&mut self);
}
