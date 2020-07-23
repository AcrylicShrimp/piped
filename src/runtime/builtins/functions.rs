use super::super::function::Function;
use super::super::sub_execution::SubExecution;
use super::super::value::{Value, ValueType};
use std::path::PathBuf;

macro_rules! define_function {
    ($name:ident ($execution:ident, $arg:ident) => $body:block) => {
        pub struct $name {}

        impl $name {
            pub fn new() -> $name {
                $name {}
            }
        }

        impl Function for $name {
            fn call(&self, $execution: &mut SubExecution, $arg: Vec<Value>) -> Value $body
        }
    };
}

define_function!(Get(_execution, argument_vec) => {
    if argument_vec.len() != 2 {
        panic!("2 arguments required, got {}.", argument_vec.len())
    }

    if let Value::Array(array) = &argument_vec[0] {
        let mut index = match argument_vec[1].to_strict::<i64>() {
            Some(index) => index,
            None => panic!("Type mismatch; only {:#?} can be used here.", ValueType::Integer)
        };

        if index < 0 {
            index += array.len() as i64
        }

        if index < 0 || array.len() as i64 <= index {
            panic!("Out of index.")
        }

        array[index as usize].clone()
    } else if let Value::Dictionary(dict) = &argument_vec[0] {
        let key = match argument_vec[1].to_strict::<String>() {
            Some(key) => key,
            None => panic!("Type mismatch; only {:#?} can be used here.", ValueType::String)
        };

        match dict.get(&key) {
            Some(value) => value.clone(),
            None => panic!("Unable to find key \"{}\".", key)
        }
    } else {
         panic!("Type mismatch; only {:#?} or {:#?} can be used here.", ValueType::Array, ValueType::Dictionary)
    }
});
define_function!(Typeof(_execution, argument_vec) => {
    if argument_vec.len() != 1 {
        panic!("1 argument required, got {}.", argument_vec.len())
    }

    Value::String {
        0: match argument_vec[0].value_type() {
            ValueType::Array => "array",
            ValueType::Dictionary => "dictionary",
            ValueType::Bool => "bool",
            ValueType::Integer => "integer",
            ValueType::String => "string",
        }.to_owned()
    }
});
define_function!(IsExists(execution, argument_vec) => {
    if argument_vec.len() != 1 {
        panic!("1 argument required, got {}.", argument_vec.len())
    }

    Value::Bool {
        0: match argument_vec[0].to_strict::<String>() {
            Some(string_value) => {
                execution.get_variable(&string_value).is_some()
            }
            None => panic!("string is required")
        }
    }
});
define_function!(JoinPath(_execution, argument_vec) => {
    let mut path = PathBuf::new();

    for argument in argument_vec.into_iter() {
        match argument.to_strict::<String>() {
            Some(string_value) => {
                path.push(string_value);
            }
            None => panic!("string is required")
        }
    }

    Value::String(path.to_str().unwrap().to_owned())
});
