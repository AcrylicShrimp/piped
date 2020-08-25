use glob::{glob, GlobError};
use std::fs::canonicalize;
use std::path::PathBuf;

define_function!(Glob(_execution, argument_vec) => {
	if argument_vec.len() != 1 {
		panic!("1 argument required, got {}.", argument_vec.len())
	}

	Value::Array(
		match argument_vec[0].to_strict::<String>() {
			Some(string_value) => {
				match glob(&string_value).map(|paths| paths.collect::<Vec<Result<PathBuf, GlobError>>>().into_iter().flatten().collect::<Vec<PathBuf>>()) {
					Ok(path_vec) => path_vec.into_iter().map(|path| canonicalize(path)).flatten().map(|path| path.into_os_string().into_string()).flatten().map(|path| Value::String(path)).collect::<Vec<Value>>(),
					Err(err) => panic!("wrong glob pattern: {}", err)
				}
			}
			None => panic!("string is required")
		}
	)
});
