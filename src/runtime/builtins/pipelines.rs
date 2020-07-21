use super::super::pipeline::{PipelineExecution, PipelineExecutionResult};
use super::super::value::{Value, ValueType};
use std::collections::HashMap;
use std::process::Command;

pub struct Exec {}

impl Exec {
    pub fn new(argument_map: &HashMap<String, Value>) -> Box<PipelineExecution> {
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

        let mut command = Command::new(cmd);

        if !params.is_empty() {
            command.args(params);
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
            }
        })
    }
}
