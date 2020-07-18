use super::super::function::Function;
use super::super::value::Value;
use std::path::PathBuf;

macro_rules! define_function {
    ($name:ident ($arg:ident) => $body:block) => {
        pub struct $name {}

        impl $name {
            pub fn new() -> $name {
                $name {}
            }
        }

        impl Function for $name {
            fn call(&mut self, $arg: Vec<Value>) -> Value $body
        }
    };
}

define_function!(JoinPath(argument_vec) => {
    let mut path = PathBuf::new();

    for argument in argument_vec.into_iter() {
        if let Value::String(string_argument) = argument {
            path.push(&string_argument);

        } else {panic!("string is required")}
    }

    Value::String(path.to_str().unwrap().to_owned())
});
