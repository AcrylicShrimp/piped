use super::super::compiler::lexer::TokenType;
use super::super::compiler::parser::{
	AwaitAST, ExpressionAST, IfAST, LiteralAST, NonBlockAST, PipelineAST, PrintAST, PrintErrAST,
	SetAST, AST,
};
use super::value::Value;
use std::collections::HashMap;

pub struct Context {
	variable_map: HashMap<String, Value>,
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

		Context { variable_map }
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
				AST::Await(await_ast) => {}
				AST::AwaitAll => {}
				AST::NonBlock(non_block_ast) => {}
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
				AST::Pipeline(pipeline_ast) => {}
			}
		}
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
