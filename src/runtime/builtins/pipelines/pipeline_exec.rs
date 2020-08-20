use super::super::super::value::ValueType;
use std::process::Command;

define_pipeline!(Exec, argument_map => {
	let cmd = match argument_map.get("cmd") {
		Some(cmd) => match cmd.to_strict::<String>() {
			Some(cmd) => cmd,
			None => panic!("'{}' must be a '{:#?}' type", "cmd", ValueType::String),
		},
		None => panic!("'{}' is requied", "cmd"),
	};
	let params = match argument_map.get("params") {
		Some(params) => match params.to_strict::<Vec<String>>() {
			Some(params) => params,
			None => panic!(
				"'{}' must be a '{:#?}' of '{:#?}' type",
				"params",
				ValueType::Array,
				ValueType::String
			),
		},
		None => vec![],
	};
	let envs = match argument_map.get("envs") {
		Some(envs) => match envs.to_strict::<HashMap<String, String>>() {
			Some(envs) => envs,
			None => panic!(
				"'{}' must be a '{:#?}' of '{:#?}' type",
				"env",
				ValueType::Dictionary,
				ValueType::String
			),
		},
		None => HashMap::new(),
	};

	let mut command = Command::new(cmd);

	if !params.is_empty() {
		command.args(params);
	}

	if !envs.is_empty() {
		command.envs(envs);
	}

	Box::new(move || -> PipelineExecutionResult {
		PipelineExecutionResult {
			success: match command.spawn() {
				Ok(child) => match child.wait_with_output() {
					Ok(..) => true,
					Err(..) => false,
				},
				Err(..) => false,
			},
			result: None,
		}
	})
});
