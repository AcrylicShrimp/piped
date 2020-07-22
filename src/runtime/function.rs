use super::value::Value;

pub trait Function {
    fn call(&self, argument_vec: Vec<Value>) -> Value;
}
