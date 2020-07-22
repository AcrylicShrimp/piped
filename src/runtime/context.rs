use super::super::compiler::parser::{ExpressionAST, LiteralAST, AST};
use super::builtins::functions::{Get, JoinPath};
use super::builtins::pipelines::Exec;
use super::function::Function;
use super::pipeline::{PipelineExecutionResult, PipelineFactory};
use super::value::Value;
use std::collections::HashMap;
use std::thread::{spawn, JoinHandle};

pub struct Context {
	variable_map: HashMap<String, Value>,
	function_map: HashMap<String, Box<dyn Function>>,
	pipeline_factory_map: HashMap<String, Box<PipelineFactory>>,
}

impl Context {
	pub fn new() -> Context {
		let mut variable_map = HashMap::new();

		#[cfg(target_arch = "x86")]
		variable_map.insert("hostArch".to_owned(), Value::String("x86".to_owned()));
		#[cfg(target_arch = "x86_64")]
		variable_map.insert("hostArch".to_owned(), Value::String("x86_64".to_owned()));
		#[cfg(target_arch = "arm")]
		variable_map.insert("hostArch".to_owned(), Value::String("arm".to_owned()));

		#[cfg(target_os = "linux")]
		variable_map.insert("hostOS".to_owned(), Value::String("linux".to_owned()));
		#[cfg(target_os = "macos")]
		variable_map.insert("hostOS".to_owned(), Value::String("macos".to_owned()));
		#[cfg(target_os = "windows")]
		variable_map.insert("hostOS".to_owned(), Value::String("windows".to_owned()));

		variable_map.insert("lastExecExitCode".to_owned(), Value::Integer(0));
		variable_map.insert("lastExecStdOut".to_owned(), Value::String("".to_owned()));
		variable_map.insert("lastExecStdErr".to_owned(), Value::String("".to_owned()));

		let mut function_map: HashMap<_, Box<dyn Function>> = HashMap::new();

		function_map.insert("get".to_owned(), Box::new(Get::new()));
		function_map.insert("join_path".to_owned(), Box::new(JoinPath::new()));

		let mut pipeline_factory_map: HashMap<_, Box<PipelineFactory>> = HashMap::new();

		pipeline_factory_map.insert("exec".to_owned(), Box::new(Exec::new));

		Context {
			variable_map,
			function_map,
			pipeline_factory_map,
		}
	}

	pub fn execute(&mut self, ast_vec: &Vec<AST>) {
		let (named_pipelines, unnamed_pipelines) = self.__execute(ast_vec);

		for (_, pipeline) in named_pipelines.into_iter() {
			for pipeline in pipeline {
				pipeline.join().unwrap();
			}
		}

		for pipeline in unnamed_pipelines {
			pipeline.join().unwrap();
		}
	}

	fn __execute(
		&mut self,
		ast_vec: &Vec<AST>,
	) -> (
		HashMap<String, Vec<JoinHandle<PipelineExecutionResult>>>,
		Vec<JoinHandle<PipelineExecutionResult>>,
	) {
		let mut named_pipeline_map: HashMap<String, Vec<JoinHandle<PipelineExecutionResult>>> =
			HashMap::new();
		let mut unnamed_pipeline_vec: Vec<JoinHandle<PipelineExecutionResult>> = Vec::new();

		for ast in ast_vec.iter() {
			match ast {
				AST::Set(set_ast) => {
					let value = self.expression_to_value(&set_ast.value);
					self.variable_map
						.insert(set_ast.name.token_content.clone(), value);
				}
				AST::Print(print_ast) => {
					for expression_ast in print_ast.expression_vec.iter() {
						print!("{}", self.expression_to_value(expression_ast));
					}
					println!("");
				}
				AST::PrintErr(print_err_ast) => {
					for expression_ast in print_err_ast.expression_vec.iter() {
						eprint!("{}", self.expression_to_value(expression_ast));
					}
					eprintln!("");
				}
				AST::Await(await_ast) => match &await_ast.name {
					Some(name) => match named_pipeline_map.remove(&name.token_content) {
						Some(named_pipeline_vec) => {
							for named_pipeline in named_pipeline_vec {
								named_pipeline.join().unwrap();
							}
						}
						None => {}
					},
					None => {
						for unnamed_pipeline in unnamed_pipeline_vec {
							unnamed_pipeline.join().unwrap();
						}
						unnamed_pipeline_vec = Vec::new();
					}
				},
				AST::AwaitAll => {
					for named_pipeline_vec in named_pipeline_map.into_iter() {
						for named_pipeline in named_pipeline_vec.1 {
							named_pipeline.join().unwrap();
						}
					}
					named_pipeline_map = HashMap::new();

					for unnamed_pipeline in unnamed_pipeline_vec {
						unnamed_pipeline.join().unwrap();
					}
					unnamed_pipeline_vec = Vec::new();
				}
				AST::NonBlock(non_block_ast) => {
					let argument_map = non_block_ast
						.pipeline
						.argument_vec
						.iter()
						.map(|(key, value)| {
							(key.token_content.clone(), self.expression_to_value(value))
						})
						.collect();

					let mut pipeline = match self
						.pipeline_factory_map
						.get(&non_block_ast.pipeline.name.token_content)
					{
						Some(pipeline) => pipeline(&argument_map),
						None => panic!(
							"undefined pipeline '{}' used",
							&non_block_ast.pipeline.name.token_content
						),
					};

					let pipeline_join_handle = spawn(move || pipeline());

					match &non_block_ast.name {
						Some(name) => match named_pipeline_map.get_mut(&name.token_content) {
							Some(named_pipeline_vec) => {
								named_pipeline_vec.push(pipeline_join_handle);
							}
							None => {
								named_pipeline_map
									.insert(name.token_content.clone(), vec![pipeline_join_handle]);
							}
						},
						None => {
							unnamed_pipeline_vec.push(pipeline_join_handle);
						}
					}
				}
				AST::If(if_ast) => {
					if compare_value(
						&self.expression_to_value(&if_ast.criteria_left),
						&self.expression_to_value(&if_ast.criteria_right),
					) {
						let (named_pipelines, unnamed_pipelines) =
							self.__execute(&if_ast.if_ast_vec);

						for (pipeline_name, pipeline) in named_pipelines.into_iter() {
							match named_pipeline_map.get_mut(&pipeline_name) {
								Some(named_pipeline_vec) => {
									named_pipeline_vec.extend(pipeline);
								}
								None => {
									named_pipeline_map.insert(pipeline_name, pipeline);
								}
							}
						}

						unnamed_pipeline_vec.extend(unnamed_pipelines);
					} else if let Some(else_ast) = &if_ast.else_ast_vec {
						let (named_pipelines, unnamed_pipelines) = self.__execute(else_ast);

						for (pipeline_name, pipeline) in named_pipelines.into_iter() {
							match named_pipeline_map.get_mut(&pipeline_name) {
								Some(named_pipeline_vec) => {
									named_pipeline_vec.extend(pipeline);
								}
								None => {
									named_pipeline_map.insert(pipeline_name, pipeline);
								}
							}
						}

						unnamed_pipeline_vec.extend(unnamed_pipelines);
					}
				}
				AST::Pipeline(pipeline_ast) => {
					let argument_map = pipeline_ast
						.argument_vec
						.iter()
						.map(|(key, value)| {
							(key.token_content.clone(), self.expression_to_value(value))
						})
						.collect();

					match self
						.pipeline_factory_map
						.get(&pipeline_ast.name.token_content)
					{
						Some(pipeline) => {
							pipeline(&argument_map)();
						}
						None => panic!(
							"undefined pipeline '{}' used",
							&pipeline_ast.name.token_content
						),
					}
				}
				AST::Call(call_ast) => {
					let argument_expression_vec = call_ast
						.argument_vec
						.iter()
						.map(|expression| self.expression_to_value(expression))
						.collect();

					match self.function_map.get_mut(&call_ast.name.token_content) {
						Some(function) => {
							function.call(argument_expression_vec);
						}
						None => {
							panic!("undefined function '{}' used", &call_ast.name.token_content)
						}
					}
				}
			}
		}

		(named_pipeline_map, unnamed_pipeline_vec)
	}

	fn expression_to_value(&mut self, expression_ast: &ExpressionAST) -> Value {
		match expression_ast {
			ExpressionAST::Array(array) => Value::Array(
				array
					.iter()
					.map(|element| self.expression_to_value(element))
					.collect(),
			),
			ExpressionAST::Dictionary(dictionary) => Value::Dictionary(
				dictionary
					.iter()
					.map(|(key, value)| (key.clone(), self.expression_to_value(&value.1)))
					.collect(),
			),
			ExpressionAST::Literal(literal_ast) => literal_to_value(literal_ast),
			ExpressionAST::Variable(token) => match self.variable_map.get(&token.token_content) {
				Some(value) => value.clone(),
				None => panic!("undefined variable '{}' used", &token.token_content),
			},
			ExpressionAST::Call(call_ast) => {
				let argument_expression_vec = call_ast
					.argument_vec
					.iter()
					.map(|expression| self.expression_to_value(expression))
					.collect();

				match self.function_map.get_mut(&call_ast.name.token_content) {
					Some(function) => function.call(argument_expression_vec),
					None => panic!("undefined function '{}' used", &call_ast.name.token_content),
				}
			}
		}
	}
}

fn literal_to_value(literal_ast: &LiteralAST) -> Value {
	match literal_ast {
		LiteralAST::Bool(token) => Value::Bool(token.token_content == "true"),
		LiteralAST::Integer(token) => Value::Integer(token.token_content.parse::<i64>().unwrap()),
		LiteralAST::String(token) => Value::String(token.token_content.clone()),
	}
}

fn compare_value(left: &Value, right: &Value) -> bool {
	if left.value_type() != right.value_type() {
		return false;
	}

	match left {
		Value::Array(left_array) => {
			if let Value::Array(right_array) = right {
				if left_array.len() != right_array.len() {
					return false;
				}

				for index in 0..left_array.len() {
					if !compare_value(&left_array[index], &right_array[index]) {
						return true;
					}
				}

				true
			} else {
				unreachable!()
			}
		}
		Value::Dictionary(left_dictionary) => false,
		Value::Bool(left_bool) => {
			if let Value::Bool(right_bool) = right {
				left_bool == right_bool
			} else {
				unreachable!()
			}
		}
		Value::Integer(left_integer) => {
			if let Value::Integer(right_integer) = right {
				left_integer == right_integer
			} else {
				unreachable!()
			}
		}
		Value::String(left_string) => {
			if let Value::String(right_string) = right {
				left_string == right_string
			} else {
				unreachable!()
			}
		}
	}
}
