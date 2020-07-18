use super::value::Value;

pub trait Function {
    fn call(&mut self, argument_vec: Vec<Value>) -> Value;
}
