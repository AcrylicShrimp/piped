use super::sub_execution::SubExecution;
use super::value::Value;

pub trait Function {
    fn call(&self, sub_execution: &mut SubExecution, argument_vec: Vec<Value>) -> Value;
}
