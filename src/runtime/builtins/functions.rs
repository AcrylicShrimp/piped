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
        match argument.to_strict::<String>() {
            Some(string_value) =>{ path.push(string_value);}
            None => panic!("string is required")
        }
    }

    Value::String(path.to_str().unwrap().to_owned())
});
