use regex::Regex;

define_function!(ReReplace(_execution, argument_vec) => {
	if argument_vec.len() != 3 {
		panic!("2 argument required, got {}.", argument_vec.len())
	}

	let pattern = match argument_vec[0].to_strict::<String>() {
		Some(string_value) => string_value,
		None => panic!("string is required")
	};
	let source = match argument_vec[1].to_strict::<String>() {
		Some(string_value) => string_value,
		None => panic!("string is required")
	};
	let replacement = match argument_vec[2].to_strict::<String>() {
		Some(string_value) => string_value,
		None => panic!("string is required")
	};

	let re = match Regex::new(&pattern) {
		Ok(re) => re,
		Err(err) => panic!("wrong regex pattern: {}", err)
	};

	Value::String((&*re.replace_all(&source, &*replacement)).to_owned())
});
