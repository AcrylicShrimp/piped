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
}

pub fn parse(lexer: &mut Lexer) -> Vec<AST> {
    let mut ast_vec = Vec::new();
    let mut status = ParserStatus::TopLevel;

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
                continue 'parse;
            }

            panic!("unexpected token \"{:#?}\" found", token);
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

            status = ParserStatus::TopLevel;
            continue 'parse;
        } else if let ParserStatus::StatementPrint = status {
            let mut expression_vec = Vec::new();

            while next_lookahead(lexer).token_type != TokenType::Semicolon {
                expression_vec.push(parse_expression(lexer));
            }

            next(lexer);

            ast_vec.push(AST::Print {
                0: PrintAST { expression_vec },
            });

            status = ParserStatus::TopLevel;
            continue 'parse;
        } else if let ParserStatus::StatementPrintErr = status {
            let mut expression_vec = Vec::new();

            while next_lookahead(lexer).token_type != TokenType::Semicolon {
                expression_vec.push(parse_expression(lexer));
            }

            next(lexer);

            ast_vec.push(AST::PrintErr {
                0: PrintErrAST { expression_vec },
            });

            status = ParserStatus::TopLevel;
            continue 'parse;
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

            status = ParserStatus::TopLevel;
            continue 'parse;
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

            status = ParserStatus::TopLevel;
            continue 'parse;
        } else {
            unreachable!();
        }
    }
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
