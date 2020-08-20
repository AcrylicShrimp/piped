use super::lexer::{LexerError, Token, TokenType};
use super::lookahead_lexer::LookaheadLexer as Lexer;
use std::cmp::max;
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Debug)]
pub enum AST {
    Import(ImportAST),
    Set(SetAST),
    Print(PrintAST),
    PrintErr(PrintErrAST),
    Return(ReturnAST),
    Await(AwaitAST),
    AwaitAll,
    NonBlock(NonBlockAST),
    If(IfAST),
    Pipeline(PipelineAST),
    Call(CallAST),
}

#[derive(Debug)]
pub struct ImportAST {
    pub name: Token,
    pub path: ExpressionAST,
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
pub struct ReturnAST {
    pub value: Option<ExpressionAST>,
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
    pub criteria: ExpressionAST,
    pub if_ast_vec: Vec<AST>,
    pub else_ast_vec: Option<Vec<AST>>,
}

#[derive(Debug)]
pub struct PipelineAST {
    pub result_as: Option<Token>,
    pub name: Token,
    pub argument_vec: Vec<(Token, ExpressionAST)>,
}

#[derive(Debug)]
pub enum ExpressionAST {
    Array(Vec<ExpressionAST>),
    Dictionary(HashMap<String, (Token, ExpressionAST)>),
    Literal(LiteralAST),
    Variable(Token),
    Call(CallAST),
}

#[derive(Debug)]
pub enum LiteralAST {
    Bool(Token),
    Integer(Token),
    String(Token),
}

#[derive(Debug)]
pub struct CallAST {
    pub name: Token,
    pub argument_vec: Vec<ExpressionAST>,
}

enum ParserStatus {
    TopLevel,
    Statement,
    StatementImport,
    StatementSet,
    StatementPrint,
    StatementPrintErr,
    StatementAwait,
    StatementReturn,
    StatementResult,
    StatementNonBlock,
    StatementIf,
    StatementIfNext(IfAST),
    StatementIfNextStatement(IfAST),
    StatementIfNextElse(IfAST),
}

pub fn parse(lexer: &mut Lexer) -> Result<Vec<AST>, ()> {
    let ast_vec = parse_statement_vec(lexer)?;
    next_token(lexer, TokenType::Eof)?;
    Ok(ast_vec)
}

fn parse_statement_vec(lexer: &mut Lexer) -> Result<Vec<AST>, ()> {
    let mut ast_vec = Vec::new();

    loop {
        let statement_vec = parse_statement(lexer, ParserStatus::TopLevel)?;

        if statement_vec.is_empty() {
            break;
        }

        ast_vec.extend(statement_vec);
    }

    Ok(ast_vec)
}

fn parse_statement(lexer: &mut Lexer, status: ParserStatus) -> Result<Vec<AST>, ()> {
    let mut ast_vec = Vec::new();
    let mut status = status;

    'parse: loop {
        if let ParserStatus::TopLevel = status {
            let token = next_lookahead(lexer)?;

            if token.token_type == TokenType::Eof {
                return Ok(ast_vec);
            }

            if token.token_type == TokenType::At {
                next(lexer)?;
                status = ParserStatus::Statement;
                continue 'parse;
            }

            if token.token_type == TokenType::Id {
                ast_vec.push(parse_pipeline(lexer)?);
                return Ok(ast_vec);
            }

            return Ok(ast_vec);
        } else if let ParserStatus::Statement = status {
            let statement_token = next(lexer)?;

            match statement_token.token_type {
                TokenType::KeywordImport => {
                    status = ParserStatus::StatementImport;
                    continue 'parse;
                }
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
                TokenType::KeywordReturn => {
                    status = ParserStatus::StatementReturn;
                    continue 'parse;
                }
                TokenType::KeywordAwait => {
                    status = ParserStatus::StatementAwait;
                    continue 'parse;
                }
                TokenType::KeywordResult => {
                    status = ParserStatus::StatementResult;
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
                _ => {
                    print_last_line_of_token(
                        lexer,
                        &statement_token,
                        "A 'import', 'set', 'print', 'printErr', 'return', 'await', 'result', 'nonblock' and 'if' keyword only can be used here.",
                    );
                    return Err(());
                }
            }
        } else if let ParserStatus::StatementImport = status {
            let expression_ast = parse_expression(lexer)?;

            next_token(lexer, TokenType::KeywordAs)?;

            let name_token = next_token(lexer, TokenType::Id)?;

            next_token(lexer, TokenType::Semicolon)?;

            ast_vec.push(AST::Import {
                0: ImportAST {
                    name: name_token,
                    path: expression_ast,
                },
            });

            return Ok(ast_vec);
        } else if let ParserStatus::StatementSet = status {
            let name_token = next_token(lexer, TokenType::Id)?;

            next_token(lexer, TokenType::Equal)?;

            let expression_ast = parse_expression(lexer)?;

            next_token(lexer, TokenType::Semicolon)?;

            ast_vec.push(AST::Set {
                0: SetAST {
                    name: name_token,
                    value: expression_ast,
                },
            });

            return Ok(ast_vec);
        } else if let ParserStatus::StatementPrint = status {
            let mut expression_vec = Vec::new();

            while next_lookahead(lexer)?.token_type != TokenType::Semicolon {
                expression_vec.push(parse_expression(lexer)?);
            }

            next(lexer)?;

            ast_vec.push(AST::Print {
                0: PrintAST { expression_vec },
            });

            return Ok(ast_vec);
        } else if let ParserStatus::StatementPrintErr = status {
            let mut expression_vec = Vec::new();

            while next_lookahead(lexer)?.token_type != TokenType::Semicolon {
                expression_vec.push(parse_expression(lexer)?);
            }

            next(lexer)?;

            ast_vec.push(AST::PrintErr {
                0: PrintErrAST { expression_vec },
            });

            return Ok(ast_vec);
        } else if let ParserStatus::StatementReturn = status {
            ast_vec.push(AST::Return(ReturnAST {
                value: if next_lookahead(lexer)?.token_type == TokenType::Semicolon {
                    next(lexer)?;
                    None
                } else {
                    let value = Some(parse_expression(lexer)?);
                    next_token(lexer, TokenType::Semicolon)?;
                    value
                },
            }));
            return Ok(ast_vec);
        } else if let ParserStatus::StatementAwait = status {
            let semicolon_or_string_or_all = next(lexer)?;

            ast_vec.push(match semicolon_or_string_or_all.token_type {
                TokenType::Semicolon => AST::Await {
                    0: AwaitAST { name: None },
                },
                TokenType::LiteralString => {
                    next_token(lexer, TokenType::Semicolon)?;
                    AST::Await {
                        0: AwaitAST {
                            name: Some(semicolon_or_string_or_all),
                        },
                    }
                }
                TokenType::KeywordAll => {
                    next_token(lexer, TokenType::Semicolon)?;
                    AST::AwaitAll
                }
                _ => {
                    print_last_line_of_token(
                        lexer,
                        &semicolon_or_string_or_all,
                        "An await statement should be followed by a semicolon, a string literal or an 'all' keyword.",
                    );
                    return Err(());
                }
            });

            return Ok(ast_vec);
        } else if let ParserStatus::StatementResult = status {
            ast_vec.push(parse_pipeline_result(lexer)?);
            return Ok(ast_vec);
        } else if let ParserStatus::StatementNonBlock = status {
            let name_token = next_lookahead(lexer)?;

            ast_vec.push(AST::NonBlock(NonBlockAST {
                name: if name_token.token_type == TokenType::LiteralString {
                    next(lexer)?;
                    Some(name_token)
                } else {
                    None
                },
                pipeline: match if next_lookahead(lexer)?.token_type == TokenType::At {
                    next(lexer)?;
                    next_token(lexer, TokenType::KeywordResult)?;
                    parse_pipeline_result(lexer)
                } else {
                    parse_pipeline(lexer)
                }? {
                    AST::Pipeline(pipeline_ast) => pipeline_ast,
                    AST::Call(call_ast) => {
                        print_last_line_of_token(
                            lexer,
                            &call_ast.name,
                            "A non-block statement should be followed by a pipeline statement.",
                        );
                        return Err(());
                    }
                    _ => unreachable!(),
                },
            }));

            return Ok(ast_vec);
        } else if let ParserStatus::StatementIf = status {
            status = ParserStatus::StatementIfNext(parse_if(lexer)?);
            continue 'parse;
        } else if let ParserStatus::StatementIfNext(if_ast) = status {
            let token = next_lookahead(lexer)?;

            if token.token_type == TokenType::At {
                next(lexer)?;
                status = ParserStatus::StatementIfNextStatement(if_ast);
                continue 'parse;
            }

            ast_vec.push(AST::If(if_ast));
            return Ok(ast_vec);
        } else if let ParserStatus::StatementIfNextStatement(if_ast) = status {
            let statement_token = next(lexer)?;

            match statement_token.token_type {
                TokenType::KeywordImport => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementImport;
                    continue 'parse;
                }
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
                TokenType::KeywordReturn => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementReturn;
                    continue 'parse;
                }
                TokenType::KeywordAwait => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementAwait;
                    continue 'parse;
                }
                TokenType::KeywordResult => {
                    ast_vec.push(AST::If(if_ast));
                    status = ParserStatus::StatementResult;
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
                _ => {
                    print_last_line_of_token(
                        lexer,
                        &statement_token,
                        "A 'import', 'set', 'print', 'printErr', 'return', 'await', 'result', 'nonblock', 'if' and 'else' keyword only can be used here.",
                    );
                    return Err(());
                }
            }
        } else if let ParserStatus::StatementIfNextElse(mut if_ast) = status {
            let if_or_brace_token = next_lookahead(lexer)?;

            match if_or_brace_token.token_type {
                TokenType::KeywordIf => {
                    next(lexer)?;
                    let mut statement_vec = parse_statement(lexer, ParserStatus::StatementIf)?;

                    if statement_vec.is_empty() {
                        print_last_line_of_token(
                            lexer,
                            &if_or_brace_token,
                            "This if statement is not fully closed; terminated unexpectedly.",
                        );
                        return Err(());
                    }

                    if_ast.else_ast_vec = Some(vec![statement_vec.drain(0..).next().unwrap()]);
                    ast_vec.push(AST::If(if_ast));
                    ast_vec.extend(statement_vec);
                }
                TokenType::BraceL => {
                    if_ast.else_ast_vec = Some(parse_block(lexer)?);
                    ast_vec.push(AST::If(if_ast));
                }
                _ => {
                    print_last_line_of_token(
                        lexer,
                        &if_or_brace_token,
                        "An else statement should be followed by an if statement or a block statement.",
                    );
                    return Err(());
                }
            }

            return Ok(ast_vec);
        } else {
            unreachable!();
        }
    }
}

fn parse_if(lexer: &mut Lexer) -> Result<IfAST, ()> {
    Ok(IfAST {
        criteria: parse_expression(lexer)?,
        if_ast_vec: parse_block(lexer)?,
        else_ast_vec: None,
    })
}

fn parse_block(lexer: &mut Lexer) -> Result<Vec<AST>, ()> {
    next_token(lexer, TokenType::BraceL)?;
    let ast_vec = parse_statement_vec(lexer)?;
    next_token(lexer, TokenType::BraceR)?;

    Ok(ast_vec)
}

fn parse_pipeline_result(lexer: &mut Lexer) -> Result<AST, ()> {
    next_token(lexer, TokenType::KeywordAs)?;
    let result_as = Some(next_token(lexer, TokenType::Id)?);

    let mut ast = parse_pipeline(lexer)?;

    if let AST::Pipeline(pipeline_ast) = &mut ast {
        pipeline_ast.result_as = result_as;
    } else if let AST::Call(call_ast) = &ast {
        print_last_line_of_token(
            lexer,
            &call_ast.name,
            "An result statement should be followed by an pipeline statement.",
        );
        return Err(());
    } else {
        unreachable!();
    }

    Ok(ast)
}

fn parse_pipeline(lexer: &mut Lexer) -> Result<AST, ()> {
    let id_token = next_token(lexer, TokenType::Id)?;

    if next_lookahead(lexer)?.token_type == TokenType::ParenL {
        let call_ast = AST::Call(parse_call(lexer, id_token)?);
        next_token(lexer, TokenType::Semicolon)?;
        return Ok(call_ast);
    }

    let mut argument_vec = Vec::new();

    loop {
        let name_or_semicolon_token = next(lexer)?;

        if name_or_semicolon_token.token_type == TokenType::Semicolon {
            break;
        }

        if name_or_semicolon_token.token_type != TokenType::Id {
            print_last_line_of_token(
                lexer,
                &name_or_semicolon_token,
                "An identifier or a semicolon only can be placed here.",
            );
            return Err(());
        }

        next_token(lexer, TokenType::Equal)?;

        let expression_ast = parse_expression(lexer)?;

        argument_vec.push((name_or_semicolon_token, expression_ast));
    }

    Ok(AST::Pipeline {
        0: PipelineAST {
            result_as: None,
            name: id_token,
            argument_vec,
        },
    })
}

fn parse_call(lexer: &mut Lexer, name_token: Token) -> Result<CallAST, ()> {
    next_token(lexer, TokenType::ParenL)?;

    let mut expression_vec = Vec::new();

    loop {
        let token = next_lookahead(lexer)?;

        if token.token_type == TokenType::ParenR {
            break;
        }

        expression_vec.push(parse_expression(lexer)?);

        let comma_or_parent_token = next_lookahead(lexer)?;

        match comma_or_parent_token.token_type {
            TokenType::Comma => {
                next(lexer)?;
            }
            TokenType::ParenR => {
                break;
            }
            _ => {
                print_last_line_of_token(
                    lexer,
                    &comma_or_parent_token,
                    "A comma or a right parenthesis only can be placed here.",
                );
                return Err(());
            }
        }
    }

    next_token(lexer, TokenType::ParenR)?;

    Ok(CallAST {
        name: name_token,
        argument_vec: expression_vec,
    })
}

fn parse_expression(lexer: &mut Lexer) -> Result<ExpressionAST, ()> {
    let expression_token = next_lookahead(lexer)?;

    Ok(match expression_token.token_type {
        TokenType::LiteralBool => ExpressionAST::Literal {
            0: LiteralAST::Bool { 0: next(lexer)? },
        },
        TokenType::LiteralInteger => ExpressionAST::Literal {
            0: LiteralAST::Integer { 0: next(lexer)? },
        },
        TokenType::LiteralString => ExpressionAST::Literal {
            0: LiteralAST::String { 0: next(lexer)? },
        },
        TokenType::Id => {
            let name_token = next(lexer)?;

            if next_lookahead(lexer)?.token_type == TokenType::ParenL {
                ExpressionAST::Call {
                    0: parse_call(lexer, name_token)?,
                }
            } else {
                ExpressionAST::Variable { 0: name_token }
            }
        }
        TokenType::BracketL => parse_array(lexer)?,
        TokenType::BraceL => parse_dict(lexer)?,
        _ => {
            print_last_line_of_token(
                lexer,
                &expression_token,
                "An expression only can be placed here.",
            );
            return Err(());
        }
    })
}

fn parse_array(lexer: &mut Lexer) -> Result<ExpressionAST, ()> {
    next_token(lexer, TokenType::BracketL)?;

    let mut expression_vec = Vec::new();

    loop {
        let token = next_lookahead(lexer)?;

        if token.token_type == TokenType::BracketR {
            break;
        }

        expression_vec.push(parse_expression(lexer)?);

        let comma_or_bracket_token = next_lookahead(lexer)?;

        match comma_or_bracket_token.token_type {
            TokenType::Comma => {
                next(lexer)?;
            }
            TokenType::BracketR => {
                break;
            }
            _ => {
                print_last_line_of_token(
                    lexer,
                    &comma_or_bracket_token,
                    "A comma or a right bracket only can be placed here.",
                );
                return Err(());
            }
        }
    }

    next_token(lexer, TokenType::BracketR)?;

    Ok(ExpressionAST::Array { 0: expression_vec })
}

fn parse_dict(lexer: &mut Lexer) -> Result<ExpressionAST, ()> {
    next_token(lexer, TokenType::BraceL)?;

    let mut expression_map = HashMap::new();

    loop {
        let brace_or_name_token = next_lookahead(lexer)?;

        if brace_or_name_token.token_type == TokenType::BraceR {
            break;
        }

        if brace_or_name_token.token_type != TokenType::LiteralString
            && brace_or_name_token.token_type != TokenType::Id
        {
            print_last_line_of_token(
                lexer,
                &brace_or_name_token,
                "An identifier or a string literal only can be placed here.",
            );
            return Err(());
        }

        next(lexer)?;
        next_token(lexer, TokenType::Colon)?;

        expression_map.insert(
            brace_or_name_token.token_content.clone(),
            (brace_or_name_token, parse_expression(lexer)?),
        );

        let comma_or_brace_token = next_lookahead(lexer)?;

        match comma_or_brace_token.token_type {
            TokenType::Comma => {
                next(lexer)?;
            }
            TokenType::BraceR => {
                break;
            }
            _ => {
                print_last_line_of_token(
                    lexer,
                    &comma_or_brace_token,
                    "A comma or a right brace only can be placed here.",
                );
                return Err(());
            }
        }
    }

    next_token(lexer, TokenType::BraceR)?;

    Ok(ExpressionAST::Dictionary { 0: expression_map })
}

fn next_lookahead(lexer: &mut Lexer) -> Result<Token, ()> {
    loop {
        match lexer.next_lookahead() {
            Ok(token) => {
                if token.token_type == TokenType::Comment {
                    lexer.next().map_err(|_| ())?;
                    continue;
                }

                return Ok(token);
            }
            Err(err) => {
                handle_lexer_error(lexer, err);
                return Err(());
            }
        }
    }
}

fn next(lexer: &mut Lexer) -> Result<Token, ()> {
    loop {
        match lexer.next() {
            Ok(token) => {
                if token.token_type == TokenType::Comment {
                    lexer.next().map_err(|_| ())?;
                    continue;
                }

                return Ok(token);
            }
            Err(err) => {
                handle_lexer_error(lexer, err);
                return Err(());
            }
        }
    }
}

fn next_token(lexer: &mut Lexer, token_type: TokenType) -> Result<Token, ()> {
    let token = next(lexer)?;

    if token.token_type != token_type {
        print_last_line_of_token(
            lexer,
            &token,
            &format!("It is not allowed here; {:#?} expected.", token_type),
        )
    }

    Ok(token)
}

fn handle_lexer_error(lexer: &Lexer, err: LexerError) {
    match err {
        LexerError::StringNotClosed(token) => {
            print_last_line_of_token(lexer, &token, "String literals should be closed with \".");
        }
        LexerError::UnexpectedCharacter(token) => {
            print_last_line_of_token(lexer, &token, "Remove it, this character is not allowed.");
        }
    }
}

fn print_last_line_of_token(lexer: &Lexer, token: &Token, message: &str) {
    let actual_token_content = token.token_content.trim_end();
    let mut actual_len = actual_token_content.len();
    let begin_index = match actual_token_content.rfind('\n') {
        Some(index) => index + 1,
        None => 0,
    };

    if token.token_type == TokenType::LiteralString {
        actual_len += 2;
    }

    let max_line_number = lexer.src_content().len();
    let max_line_number_width = (max_line_number as f64).log(10f64).ceil() as usize;

    println!(
        "{}:{}:{}",
        token.file_path, token.line_number, token.line_offset
    );
    if 2 <= token.line_number {
        println!(
            "{:>width$} | {}",
            token.line_number - 1,
            lexer.src_content()[token.line_number - 2],
            width = max_line_number_width
        );
    }
    if token.line_number <= lexer.src_content().len() {
        println!(
            "{:>width$} | {}",
            token.line_number,
            lexer.src_content()[token.line_number - 1],
            width = max_line_number_width
        );
    } else {
        println!(
            "{:>width$} | ",
            token.line_number,
            width = max_line_number_width
        );
    }
    println!(
        "{}{} {}",
        &repeat(" ")
            .take(max_line_number_width + 3 + token.line_offset + begin_index - 1)
            .collect::<String>(),
        &repeat("^")
            .take(max(1, actual_len - begin_index))
            .collect::<String>(),
        message
    );
    if token.line_number < lexer.src_content().len() {
        println!(
            "{:>width$} | {}",
            token.line_number + 1,
            lexer.src_content()[token.line_number],
            width = max_line_number_width
        );
    }
    println!("");
}
