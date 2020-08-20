use super::super::compiler::parser::{ExpressionAST, LiteralAST, AST};
use super::builtins::pipelines::pipeline::build_pipeline_map;
use super::builtins::variables::variable::build_variable_map;
use super::execution::Execution;
use super::function::Function;
use super::imported_pipeline::ImportedPipeline;
use super::pipeline::{PipelineExecutionResult, PipelineFactory};
use super::value::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub struct SubExecution {
	execution: Arc<Execution>,
	variable_map: HashMap<String, Value>,
	pipeline_factory_map: HashMap<String, Box<PipelineFactory>>,
}

impl SubExecution {
	pub fn new(execution: Arc<Execution>) -> SubExecution {
		SubExecution {
			execution,
			variable_map: build_variable_map(),
			pipeline_factory_map: build_pipeline_map(),
		}
	}

	pub fn get_variable(&self, name: &str) -> Option<&Value> {
		self.variable_map.get(name)
	}

	pub fn set_variable(&mut self, name: String, value: Value) {
		self.variable_map.insert(name, value);
	}

	pub fn execute(
		&mut self,
		function_map: &Arc<HashMap<String, Box<dyn Function + Sync + Send>>>,
		pipeline: &ImportedPipeline,
	) -> Option<Value> {
		let (return_value, named_pipelines, unnamed_pipelines) =
			self.__execute(&function_map, pipeline, pipeline.ast_vec());

		for (_, pipeline) in named_pipelines.into_iter() {
			for pipeline in pipeline {
				pipeline.1.join().unwrap();
			}
		}

		for pipeline in unnamed_pipelines {
			pipeline.1.join().unwrap();
		}

		match return_value {
			Some(value) => {
				if value.is_some() {
					value
				} else {
					None
				}
			}
			None => None,
		}
	}

	fn __execute(
		&mut self,
		function_map: &Arc<HashMap<String, Box<dyn Function + Sync + Send>>>,
		pipeline: &ImportedPipeline,
		ast_vec: &Vec<AST>,
	) -> (
		Option<Option<Value>>,
		HashMap<String, Vec<(Option<String>, JoinHandle<PipelineExecutionResult>)>>,
		Vec<(Option<String>, JoinHandle<PipelineExecutionResult>)>,
	) {
		let mut named_pipeline_map: HashMap<
			String,
			Vec<(Option<String>, JoinHandle<PipelineExecutionResult>)>,
		> = HashMap::new();
		let mut unnamed_pipeline_vec: Vec<(Option<String>, JoinHandle<PipelineExecutionResult>)> =
			Vec::new();

		for ast in ast_vec.iter() {
			match ast {
				AST::Import(import_ast) => {
					if let Value::String(path) =
						self.expression_to_value(function_map, &import_ast.path)
					{
						let mut base_path = pipeline.path().clone();
						base_path.pop();

						match self.execution.import(base_path.join(Path::new(&path))) {
							Ok(imported_pipeline) => {
								let function_map = function_map.clone();
								let execution = self.execution.clone();

								self.pipeline_factory_map.insert(
									import_ast.name.token_content.clone(),
									Box::new(move |argument_map| {
										let function_map = function_map.clone();
										let variable_map = argument_map.clone();
										let imported_pipeline = imported_pipeline.clone();
										let execution = execution.clone();

										Box::new(move || {
											let mut sub_execution =
												SubExecution::new(execution.clone());

											for (name, value) in variable_map.iter() {
												sub_execution
													.set_variable(name.clone(), value.clone());
											}

											PipelineExecutionResult {
												success: true,
												result: sub_execution
													.execute(&function_map, &imported_pipeline),
											}
										})
									}),
								);
							}
							Err(err) => {
								panic!("unable to import pipeline: {}", err);
							}
						}
					} else {
						panic!("path must be a string type",)
					}
				}
				AST::Set(set_ast) => {
					let value = self.expression_to_value(function_map, &set_ast.value);
					self.variable_map
						.insert(set_ast.name.token_content.clone(), value);
				}
				AST::Print(print_ast) => {
					for expression_ast in print_ast.expression_vec.iter() {
						print!("{}", self.expression_to_value(function_map, expression_ast));
					}
					println!("");
				}
				AST::PrintErr(print_err_ast) => {
					for expression_ast in print_err_ast.expression_vec.iter() {
						eprint!("{}", self.expression_to_value(function_map, expression_ast));
					}
					eprintln!("");
				}
				AST::Return(return_ast) => {
					return (
						Some(
							return_ast
								.value
								.as_ref()
								.map(|value| self.expression_to_value(function_map, value)),
						),
						named_pipeline_map,
						unnamed_pipeline_vec,
					);
				}
				AST::Await(await_ast) => match &await_ast.name {
					Some(name) => match named_pipeline_map.remove(&name.token_content) {
						Some(named_pipeline_vec) => {
							for named_pipeline in named_pipeline_vec {
								let result = named_pipeline.1.join().unwrap();

								match named_pipeline.0 {
									Some(result_as) => match result.result {
										Some(result) => {
											self.variable_map.insert(result_as, result);
										}
										None => (),
									},
									None => (),
								}
							}
						}
						None => {}
					},
					None => {
						for unnamed_pipeline in unnamed_pipeline_vec {
							let result = unnamed_pipeline.1.join().unwrap();

							match unnamed_pipeline.0 {
								Some(result_as) => match result.result {
									Some(result) => {
										self.variable_map.insert(result_as, result);
									}
									None => (),
								},
								None => (),
							}
						}
						unnamed_pipeline_vec = Vec::new();
					}
				},
				AST::AwaitAll => {
					for named_pipeline_vec in named_pipeline_map.into_iter() {
						for named_pipeline in named_pipeline_vec.1 {
							let result = named_pipeline.1.join().unwrap();

							match named_pipeline.0 {
								Some(result_as) => match result.result {
									Some(result) => {
										self.variable_map.insert(result_as, result);
									}
									None => (),
								},
								None => (),
							}
						}
					}
					named_pipeline_map = HashMap::new();

					for unnamed_pipeline in unnamed_pipeline_vec {
						let result = unnamed_pipeline.1.join().unwrap();

						match unnamed_pipeline.0 {
							Some(result_as) => match result.result {
								Some(result) => {
									self.variable_map.insert(result_as, result);
								}
								None => (),
							},
							None => (),
						}
					}
					unnamed_pipeline_vec = Vec::new();
				}
				AST::NonBlock(non_block_ast) => {
					let argument_map = non_block_ast
						.pipeline
						.argument_vec
						.iter()
						.map(|(key, value)| {
							(
								key.token_content.clone(),
								self.expression_to_value(function_map, value),
							)
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
								named_pipeline_vec.push((
									non_block_ast
										.pipeline
										.result_as
										.as_ref()
										.map(|result_as| result_as.token_content.clone()),
									pipeline_join_handle,
								));
							}
							None => {
								named_pipeline_map.insert(
									name.token_content.clone(),
									vec![(
										non_block_ast
											.pipeline
											.result_as
											.as_ref()
											.map(|result_as| result_as.token_content.clone()),
										pipeline_join_handle,
									)],
								);
							}
						},
						None => {
							unnamed_pipeline_vec.push((
								non_block_ast
									.pipeline
									.result_as
									.as_ref()
									.map(|result_as| result_as.token_content.clone()),
								pipeline_join_handle,
							));
						}
					}
				}
				AST::If(if_ast) => {
					if match self.expression_to_value(function_map, &if_ast.criteria) {
						Value::Array(array_value) => !array_value.is_empty(),
						Value::Dictionary(dictionary_value) => !dictionary_value.is_empty(),
						Value::Bool(bool_value) => bool_value,
						Value::Integer(integer_value) => integer_value != 0,
						Value::String(string_value) => !string_value.is_empty(),
					} {
						let (return_value, named_pipelines, unnamed_pipelines) =
							self.__execute(function_map, pipeline, &if_ast.if_ast_vec);

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

						if return_value.is_some() {
							return (return_value, named_pipeline_map, unnamed_pipeline_vec);
						}
					} else if let Some(else_ast) = &if_ast.else_ast_vec {
						let (return_value, named_pipelines, unnamed_pipelines) =
							self.__execute(function_map, pipeline, else_ast);

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

						if return_value.is_some() {
							return (return_value, named_pipeline_map, unnamed_pipeline_vec);
						}
					}
				}
				AST::Pipeline(pipeline_ast) => {
					let argument_map = pipeline_ast
						.argument_vec
						.iter()
						.map(|(key, value)| {
							(
								key.token_content.clone(),
								self.expression_to_value(function_map, value),
							)
						})
						.collect();

					match self
						.pipeline_factory_map
						.get(&pipeline_ast.name.token_content)
					{
						Some(pipeline) => {
							let result = pipeline(&argument_map)();

							match result.result {
								Some(result) => match &pipeline_ast.result_as {
									Some(result_as) => {
										self.variable_map
											.insert(result_as.token_content.clone(), result);
									}
									None => (),
								},
								None => (),
							}
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
						.map(|expression| self.expression_to_value(function_map, expression))
						.collect();

					match function_map.get(&call_ast.name.token_content) {
						Some(function) => {
							function.call(self, argument_expression_vec);
						}
						None => {
							panic!("undefined function '{}' used", &call_ast.name.token_content)
						}
					}
				}
			}
		}

		(None, named_pipeline_map, unnamed_pipeline_vec)
	}

	fn expression_to_value(
		&mut self,
		function_map: &Arc<HashMap<String, Box<dyn Function + Sync + Send>>>,
		expression_ast: &ExpressionAST,
	) -> Value {
		match expression_ast {
			ExpressionAST::Array(array) => Value::Array(
				array
					.iter()
					.map(|element| self.expression_to_value(function_map, element))
					.collect(),
			),
			ExpressionAST::Dictionary(dictionary) => Value::Dictionary(
				dictionary
					.iter()
					.map(|(key, value)| {
						(
							key.clone(),
							self.expression_to_value(function_map, &value.1),
						)
					})
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
					.map(|expression| self.expression_to_value(function_map, expression))
					.collect();

				match function_map.get(&call_ast.name.token_content) {
					Some(function) => function.call(self, argument_expression_vec),
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
