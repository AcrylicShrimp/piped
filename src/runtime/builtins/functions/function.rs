use super::super::super::function::Function;
use super::{
    function_contains, function_equals, function_get, function_is_exists, function_join_path,
    function_len, function_typeof,
};
use std::collections::HashMap;

macro_rules! define_function {
    ($name:ident ($execution:ident, $arg:ident) => $body:block) => {
        use super::super::super::function::Function;
        use super::super::super::sub_execution::SubExecution;
        use super::super::super::value::Value;

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

macro_rules! insert_function {
    ($function:ty, $function_name:literal >>> $function_map:ident) => {
        $function_map.insert($function_name.to_owned(), Box::new(<$function>::new()))
    };
}

pub fn build_function_map() -> HashMap<String, Box<dyn Function + Send + Sync>> {
    let mut function_map: HashMap<_, Box<dyn Function + Send + Sync>> = HashMap::new();

    insert_function!(function_contains::Contains, "contains" >>> function_map);
    insert_function!(function_equals::Equals, "equals" >>> function_map);
    insert_function!(function_get::Get, "get" >>> function_map);
    insert_function!(function_is_exists::IsExists, "is_exists" >>> function_map);
    insert_function!(function_join_path::JoinPath, "join_path" >>> function_map);
    insert_function!(function_len::Len, "len" >>> function_map);
    insert_function!(function_typeof::Typeof, "typeof" >>> function_map);

    function_map
}
