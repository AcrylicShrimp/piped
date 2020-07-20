use super::super::pipeline::Pipeline;
use super::super::value::{Value, ValueType};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::process::{Child, Command};

pub struct Exec {
    command: Command,
    child: Option<Child>,
}

impl Exec {
    pub fn new(argument_map: &HashMap<String, Value>) -> Box<dyn Pipeline> {
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

        Box::new(Exec {
            command,
            child: None,
        })
    }
}

impl Pipeline for Exec {
    fn execute(&mut self) {
        match self.command.spawn().unwrap().wait_with_output() {
            Ok(output) => {
                stdout().write_all(&output.stdout).unwrap();
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    fn execute_background(&mut self) {
        if self.child.is_none() {
            self.child = Some(self.command.spawn().unwrap());
        }
    }

    fn wait(&mut self) {
        match self.child.take() {
            Some(child) => match child.wait_with_output() {
                Ok(output) => {
                    stdout().write_all(&output.stdout).unwrap();
                }
                Err(err) => {
                    println!("{}", err);
                }
            },
            None => {}
        }
    }
}
