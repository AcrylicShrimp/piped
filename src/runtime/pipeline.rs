use super::value::Value;
use std::collections::HashMap;

pub type PipelineFactory = dyn Fn(&HashMap<String, Value>) -> Box<PipelineExecution>;
pub type PipelineExecution = dyn FnMut() -> PipelineExecutionResult + Send;

pub struct PipelineExecutionResult {
	pub success: bool,
}
