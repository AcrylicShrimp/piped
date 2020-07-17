use super::super::compiler::lexer::TokenType;
use super::super::compiler::parser::{
	AwaitAST, ExpressionAST, IfAST, LiteralAST, NonBlockAST, PipelineAST, PrintAST, PrintErrAST,
	SetAST, AST,
};
use super::builtins::exec_pipeline::{new as new_exec_pipeline, ExecPipeline};
use super::pipeline::{Pipeline, PipelineFactory};
use super::value::Value;
use std::collections::HashMap;

pub struct Context {
	variable_map: HashMap<String, Value>,
	pipeline_map: HashMap<String, Box<PipelineFactory>>,
	named_pipeline_map: HashMap<String, Vec<Box<dyn Pipeline>>>,
	unnamed_pipeline_vec: Vec<Box<dyn Pipeline>>,
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

		let mut pipeline_map: HashMap<_, Box<PipelineFactory>> = HashMap::new();

		pipeline_map.insert("exec".to_owned(), Box::new(new_exec_pipeline));

		Context {
			variable_map,
			pipeline_map,
			named_pipeline_map: HashMap::new(),
			unnamed_pipeline_vec: Vec::new(),
		}
	}

	pub fn execute(&mut self, ast_vec: &Vec<AST>) {
		for ast in ast_vec.iter() {
			match ast {
				AST::Set(set_ast) => {
					self.variable_map.insert(
						set_ast.name.token_content.clone(),
						self.expression_to_value(&set_ast.value),
					);
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
					Some(name) => match self.named_pipeline_map.get_mut(&name.token_content) {
						Some(named_pipeline_vec) => {
							for named_pipeline in named_pipeline_vec.iter_mut() {
								named_pipeline.wait();
							}
							named_pipeline_vec.clear();
						}
						None => {}
					},
					None => {
						for unnamed_pipeline in self.unnamed_pipeline_vec.iter_mut() {
							unnamed_pipeline.wait();
						}
						self.unnamed_pipeline_vec.clear();
					}
				},
				AST::AwaitAll => {
					for named_pipeline_vec in self.named_pipeline_map.values_mut() {
						for named_pipeline in named_pipeline_vec.iter_mut() {
							named_pipeline.wait();
						}
					}
					self.named_pipeline_map.clear();

					for unnamed_pipeline in self.unnamed_pipeline_vec.iter_mut() {
						unnamed_pipeline.wait();
					}
					self.unnamed_pipeline_vec.clear();
				}
				AST::NonBlock(non_block_ast) => {
					let mut pipeline = match self
						.pipeline_map
						.get(&non_block_ast.pipeline.name.token_content)
					{
						Some(pipeline) => pipeline(
							&non_block_ast
								.pipeline
								.argument_vec
								.iter()
								.map(|(key, value)| {
									(key.token_content.clone(), self.expression_to_value(value))
								})
								.collect(),
						),
						None => panic!(
							"undefined pipeline '{}' used",
							&non_block_ast.pipeline.name.token_content
						),
					};

					pipeline.execute_background();

					match &non_block_ast.name {
						Some(name) => match self.named_pipeline_map.get_mut(&name.token_content) {
							Some(named_pipeline_vec) => {
								named_pipeline_vec.push(pipeline);
							}
							None => {
								self.named_pipeline_map
									.insert(name.token_content.clone(), vec![pipeline]);
							}
						},
						None => {
							self.unnamed_pipeline_vec.push(pipeline);
						}
					}
				}
				AST::If(if_ast) => {
					if compare_value(
						&self.expression_to_value(&if_ast.criteria_left),
						&self.expression_to_value(&if_ast.criteria_right),
					) {
						self.execute(&if_ast.if_ast_vec);
					} else if let Some(else_ast) = &if_ast.else_ast_vec {
						self.execute(else_ast);
					}
				}
				AST::Pipeline(pipeline_ast) => {
					match self.pipeline_map.get(&pipeline_ast.name.token_content) {
						Some(pipeline) => {
							pipeline(
								&pipeline_ast
									.argument_vec
									.iter()
									.map(|(key, value)| {
										(key.token_content.clone(), self.expression_to_value(value))
									})
									.collect(),
							)
							.execute();
						}
						None => panic!(
							"undefined pipeline '{}' used",
							&pipeline_ast.name.token_content
						),
					}
				}
			}
		}

		// TODO: Merge all execuing pipelines to the parent ast execution to improve its performance.
		for named_pipeline_vec in self.named_pipeline_map.values_mut() {
			for named_pipeline in named_pipeline_vec.iter_mut() {
				named_pipeline.wait();
			}
		}
		self.named_pipeline_map.clear();

		for unnamed_pipeline in self.unnamed_pipeline_vec.iter_mut() {
			unnamed_pipeline.wait();
		}
		self.unnamed_pipeline_vec.clear();
	}

	fn expression_to_value(&self, expression_ast: &ExpressionAST) -> Value {
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
