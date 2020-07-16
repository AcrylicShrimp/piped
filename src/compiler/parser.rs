use super::lexer::{Token, TokenType};
use super::lookahead_lexer::LookaheadLexer as Lexer;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AST {
    Set(SetAST),
    Print(PrintAST),
    PrintErr(PrintErrAST),
    Await(AwaitAST),
    AwaitAll,
    NonBlock(NonBlockAST),
    If(IfAST),
    Pipeline(PipelineAST),
}

#[derive(Debug)]
pub struct SetAST {
    pub name: Token,
    pub value: ExpressionAST,
}

#[derive(Debug)]
pub struct PrintAST {
    pub expression_vec: Vec<ExpressionAST>,
}

#[derive(Debug)]
pub struct PrintErrAST {
    pub expression_vec: Vec<ExpressionAST>,
}

#[derive(Debug)]
pub struct AwaitAST {
    pub name: Option<Token>,
}

#[derive(Debug)]
pub struct NonBlockAST {
    pub name: Option<Token>,
    pub pipeline: PipelineAST,
}

#[derive(Debug)]
pub struct IfAST {
    pub criteria_left: ExpressionAST,
    pub criteria_right: ExpressionAST,
    pub if_ast_vec: Vec<AST>,
    pub else_ast_vec: Option<Vec<AST>>,
}

#[derive(Debug)]
pub struct PipelineAST {
    pub name: Token,
    pub argument_vec: Vec<(Token, ExpressionAST)>,
}

#[derive(Debug)]
pub enum ExpressionAST {
    Array(Vec<ExpressionAST>),
    Dictionary(HashMap<String, (Token, ExpressionAST)>),
    Literal(LiteralAST),
    Variable(Token),
}

#[derive(Debug)]
pub enum LiteralAST {
    Bool(Token),
    Integer(Token),
    String(Token),
}

enum ParserStatus {
    TopLevel,
    Statement,
    StatementSet,
    StatementPrint,
    StatementPrintErr,
    StatementAwait,
    StatementNonBlock,
    StatementIf,
    StatementIfNext(IfAST),
    StatementIfNextStatement(IfAST),
    StatementIfNextElse(IfAST),
}

pub fn parse(lexer: &mut Lexer) -> Vec<AST> {
    let ast_vec = parse_statement_vec(lexer);
    next_token(lexer, TokenType::Eof);
    ast_vec
}

fn parse_statement_vec(lexer: &mut Lexer) -> Vec<AST> {
    let mut ast_vec = Vec::new();

    loop {
        let statement_vec = parse_statement(lexer, ParserStatus::TopLevel);

        if statement_vec.is_empty() {
            break;
        }

        ast_vec.extend(statement_vec);
    }

    ast_vec
}

fn parse_statement(lexer: &mut Lexer, status: ParserStatus) -> Vec<AST> {
    let mut ast_vec = Vec::new();
    let mut status = status;

    'parse: loop {
        if let ParserStatus::TopLevel = status {
            let token = next_lookahead(lexer);

            if token.token_type == TokenType::Eof {
                return ast_vec;
            }

            if token.token_type == TokenType::At {
                next(lexer);
                status = ParserStatus::Statement;
                continue 'parse;
            }

            if token.token_type == TokenType::Id {
                ast_vec.push(parse_pipeline(lexer));
                return ast_vec;
            }

            return ast_vec;
        } else if let ParserStatus::Statement = status {
            let statement_token = next(lexer);

            match statement_token.token_type {
                TokenType::KeywordSet => {
                    status = ParserStatus::StatementSet;
                    continue 'parse;
                }
                TokenType::KeywordPrint => {
                    status = ParserStatus::StatementPrint;
                    continue 'parse;
                }
                TokenType::KeywordPrintErr => {
                    status = ParserStatus::StatementPrintErr;
                    continue 'parse;
                }
                TokenType::KeywordAwait => {
                    status = ParserStatus::StatementAwait;
                    continue 'parse;
                }
                TokenType::KeywordNonBlock => {
                    status = ParserStatus::StatementNonBlock;
                    continue 'parse;
                }
                TokenType::KeywordIf => {
                    status = ParserStatus::StatementIf;
                    continue 'parse;
                }
                _ => panic!("unexpected token \"{:#?}\" found", statement_token),
            }
        } else if let ParserStatus::StatementSet = status {
            let name_token = next_token(lexer, TokenType::Id);

            next_token(lexer, TokenType::Equal);

            let expression_ast = parse_expression(lexer);

            next_token(lexer, TokenType::Semicolon);

            ast_vec.push(AST::Set {
                0: SetAST {
                    name: name_token,
                    value: expression_ast,
                },
            });

            return ast_vec;
        } else if let ParserStatus::StatementPrint = status {
            let mut expression_vec = Vec::new();

            while next_lookahead(lexer).token_type != TokenType::Semicolon {
                expression_vec.push(parse_expression(lexer));
            }

            next(lexer);

            ast_vec.push(AST::Print {
                0: PrintAST { expression_vec },
            });

            return ast_vec;
        } else if let ParserStatus::StatementPrintErr = status {
            let mut expression_vec = Vec::new();

            while next_lookahead(lexer).token_type != TokenType::Semicolon {
                expression_vec.push(parse_expression(lexer));
            }

            next(lexer);

            ast_vec.push(AST::PrintErr {
                0: PrintErrAST { expression_vec },
            });

            return ast_vec;
        } else if let ParserStatus::StatementAwait = status {
            let semicolon_or_string_or_all = next(lexer);

            ast_vec.push(match semicolon_or_string_or_all.token_type {
                TokenType::Semicolon => AST::Await {
                    0: AwaitAST { name: None },
                },
                TokenType::LiteralString => {
                    next_token(lexer, TokenType::Semicolon);
                    AST::Await {
                        0: AwaitAST {
                            name: Some(semicolon_or_string_or_all),
                        },
                    }
                }
                TokenType::KeywordAll => {
                    next_token(lexer, TokenType::Semicolon);
                    AST::AwaitAll
                }
                _ => panic!(
                    "unexpected token \"{:#?}\" found",
                    semicolon_or_string_or_all
                ),
            });

            return ast_vec;
        } else if let ParserStatus::StatementNonBlock = status {
            let name_token = next_lookahead(lexer);

            ast_vec.push(AST::NonBlock(NonBlockAST {
                name: if name_token.token_type == TokenType::LiteralString {
                    next(lexer);
                    Some(name_token)
                } else {
                    None
                },
                pipeline: if let AST::Pipeline(pipeline_ast) = parse_pipeline(lexer) {
                    pipeline_ast
                } else {
                    unreachable!()
                },
            }));

            return ast_vec;
        } else if let ParserStatus::StatementIf = status {
            status = ParserStatus::StatementIfNext(parse_if(lexer));
            continue 'parse;
        } else if let ParserStatus::StatementIfNext(if_ast) = status {
            let token = next_lookahead(lexer);

            if token.token_type == TokenType::At {
                next(lexer);
                status = ParserStatus::StatementIfNextStatement(if_ast);
                continue 'parse;
            }

            ast_vec.push(AST::If(if_ast));
            return ast_vec;
        } else if let ParserStatus::StatementIfNextStatement(if_ast) = status {
            let statement_token = next(lexer);

            match statement_token.token_type {
                TokenType::KeywordSet => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementSet;
                    continue 'parse;
                }
                TokenType::KeywordPrint => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementPrint;
                    continue 'parse;
                }
                TokenType::KeywordPrintErr => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementPrintErr;
                    continue 'parse;
                }
                TokenType::KeywordAwait => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementAwait;
                    continue 'parse;
                }
                TokenType::KeywordNonBlock => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementNonBlock;
                    continue 'parse;
                }
                TokenType::KeywordIf => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementIf;
                    continue 'parse;
                }
                TokenType::KeywordElse => {
                    status = ParserStatus::StatementIfNextElse(if_ast);
                    continue 'parse;
                }
                _ => panic!("unexpected token \"{:#?}\" found", statement_token),
            }
        } else if let ParserStatus::StatementIfNextElse(if_ast) = status {
            let mut if_ast = if_ast;
            let if_or_brace_token = next_lookahead(lexer);

            match if_or_brace_token.token_type {
                TokenType::KeywordIf => {
                    next(lexer);
                    let mut statement_vec = parse_statement(lexer, ParserStatus::StatementIf);

                    if statement_vec.is_empty() {
                        panic!("unexpected token \"{:#?}\" found", next(lexer));
                    }

                    if_ast.else_ast_vec = Some(vec![statement_vec.drain(0..).next().unwrap()]);
                    ast_vec.push(AST::If(if_ast));
                    ast_vec.extend(statement_vec);
                }
                TokenType::BraceL => {
                    if_ast.else_ast_vec = Some(parse_block(lexer));
                    ast_vec.push(AST::If(if_ast));
                }
                _ => panic!("unexpected token \"{:#?}\" found", if_or_brace_token),
            }

            return ast_vec;
        } else {
            unreachable!();
        }
    }
}

fn parse_if(lexer: &mut Lexer) -> IfAST {
    let criteria_left = parse_expression(lexer);
    next_token(lexer, TokenType::CompareEq);
    let criteria_right = parse_expression(lexer);

    IfAST {
        criteria_left,
        criteria_right,
        if_ast_vec: parse_block(lexer),
        else_ast_vec: None,
    }
}

fn parse_block(lexer: &mut Lexer) -> Vec<AST> {
    next_token(lexer, TokenType::BraceL);
    let ast_vec = parse_statement_vec(lexer);
    next_token(lexer, TokenType::BraceR);

    ast_vec
}

fn parse_pipeline(lexer: &mut Lexer) -> AST {
    let id_token = next_token(lexer, TokenType::Id);

    let mut argument_vec = Vec::new();

    loop {
        let name_token = next(lexer);

        if name_token.token_type == TokenType::Semicolon {
            break;
        }

        if name_token.token_type != TokenType::Id {
            panic!("unexpected token \"{:#?}\" found", name_token)
        }

        next_token(lexer, TokenType::Equal);

        let expression_ast = parse_expression(lexer);

        argument_vec.push((name_token, expression_ast));
    }

    AST::Pipeline {
        0: PipelineAST {
            name: id_token,
            argument_vec,
        },
    }
}

fn parse_expression(lexer: &mut Lexer) -> ExpressionAST {
    let expression_token = next_lookahead(lexer);

    match expression_token.token_type {
        TokenType::LiteralBool => ExpressionAST::Literal {
            0: LiteralAST::Bool { 0: next(lexer) },
        },
        TokenType::LiteralInteger => ExpressionAST::Literal {
            0: LiteralAST::Integer { 0: next(lexer) },
        },
        TokenType::LiteralString => ExpressionAST::Literal {
            0: LiteralAST::String { 0: next(lexer) },
        },
        TokenType::Id => ExpressionAST::Variable { 0: next(lexer) },
        TokenType::BracketL => parse_array(lexer),
        TokenType::BraceL => parse_dict(lexer),
        _ => panic!("unexpected token \"{:#?}\" found", expression_token),
    }
}

fn parse_array(lexer: &mut Lexer) -> ExpressionAST {
    next_token(lexer, TokenType::BracketL);

    let mut expression_vec = Vec::new();

    loop {
        let token = next_lookahead(lexer);

        if token.token_type == TokenType::BracketR {
            break;
        }

        expression_vec.push(parse_expression(lexer));

        let comma_or_bracket_token = next_lookahead(lexer);

        match comma_or_bracket_token.token_type {
            TokenType::Comma => {
                next(lexer);
            }
            TokenType::BracketR => {
                break;
            }
            _ => panic!("unexpected token \"{:#?}\" found", comma_or_bracket_token),
        }
    }

    next_token(lexer, TokenType::BracketR);

    ExpressionAST::Array { 0: expression_vec }
}

fn parse_dict(lexer: &mut Lexer) -> ExpressionAST {
    next_token(lexer, TokenType::BraceL);

    let mut expression_map = HashMap::new();

    loop {
        let brace_or_name_token = next_lookahead(lexer);

        if brace_or_name_token.token_type == TokenType::BraceR {
            break;
        }

        if brace_or_name_token.token_type != TokenType::LiteralString
            && brace_or_name_token.token_type != TokenType::Id
        {
            panic!("unexpected token \"{:#?}\" found", brace_or_name_token)
        }

        next(lexer);
        next_token(lexer, TokenType::Colon);

        expression_map.insert(
            brace_or_name_token.token_content.clone(),
            (brace_or_name_token, parse_expression(lexer)),
        );

        let comma_or_brace_token = next_lookahead(lexer);

        match comma_or_brace_token.token_type {
            TokenType::Comma => {
                next(lexer);
            }
            TokenType::BraceR => {
                break;
            }
            _ => panic!("unexpected token \"{:#?}\" found", comma_or_brace_token),
        }
    }

    next_token(lexer, TokenType::BraceR);

    ExpressionAST::Dictionary { 0: expression_map }
}

fn next_lookahead(lexer: &mut Lexer) -> Token {
    loop {
        let token = lexer.next_lookahead();

        if token.token_type == TokenType::Comment {
            lexer.next();
            continue;
        }

        return token;
    }
}

fn next(lexer: &mut Lexer) -> Token {
    loop {
        let token = lexer.next();

        if token.token_type == TokenType::Comment {
            continue;
        }

        return token;
    }
}

fn next_token(lexer: &mut Lexer, token_type: TokenType) -> Token {
    let token = next(lexer);

    if token.token_type != token_type {
        panic!(
            "{:#?} type is required instead of token \"{:#?}\"",
            token_type, token
        );
    }

    token
}
