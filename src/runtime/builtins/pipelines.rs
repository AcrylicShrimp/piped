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
            Some(cmd) => {
                if let Value::String(cmd) = cmd {
                    cmd
                } else {
                    panic!("'{}' must be a '{:#?}' type", "cmd", ValueType::String)
                }
            }
            None => panic!("'{}' is requied", "cmd"),
        };
        let params = match argument_map.get("params") {
            Some(params) => {
                if let Value::Array(array) = params {
                    array.iter().map(|element| format!("{}", element)).collect()
                } else {
                    panic!("'{}' must be a '{:#?}' type", "params", ValueType::Array)
                }
            }
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
