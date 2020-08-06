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
