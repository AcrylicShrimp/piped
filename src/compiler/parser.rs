use super::lexer::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum AST {
    Set(SetAST),
    Print(PrintAST),
    PrintErr(PrintErrAST),
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
pub enum ExpressionAST {
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
    Pipeline {
        name_token: Token,
    },
    PipelineArgument {
        name_token: Token,
        arguemnt_token_vec: Vec<Token>,
    },
}

pub fn parse(lexer: &mut Lexer) -> Vec<AST> {
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

    let mut ast_vec = Vec::new();
    let mut status = ParserStatus::TopLevel;

    'parse: loop {
        if let ParserStatus::TopLevel = &status {
            let token = next(lexer);

            if token.token_type == TokenType::Eof {
                return ast_vec;
            }

            if token.token_type == TokenType::At {
                status = ParserStatus::Statement;
                continue 'parse;
            }

            if token.token_type == TokenType::Id {
                status = ParserStatus::Pipeline { name_token: token };
                continue 'parse;
            }

            panic!("unexpected token \"{:#?}\" found", token);
        } else if let ParserStatus::Statement = &status {
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
                _ => panic!("unexpected token \"{:#?}\" found", statement_token),
            }
        } else if let ParserStatus::StatementSet = &status {
            let name_token = next_token(lexer, TokenType::Id);

            next_token(lexer, TokenType::Equal);

            let expression_token = next(lexer);

            if expression_token.token_type != TokenType::LiteralBool
                && expression_token.token_type != TokenType::LiteralInteger
                && expression_token.token_type != TokenType::LiteralString
                && expression_token.token_type != TokenType::Id
            {
                panic!("unexpected token \"{:#?}\" found", expression_token)
            }

            next_token(lexer, TokenType::Semicolon);

            ast_vec.push(AST::Set {
                0: SetAST {
                    name: name_token,
                    value: match expression_token.token_type {
                        TokenType::LiteralBool => ExpressionAST::Literal {
                            0: LiteralAST::Bool {
                                0: expression_token,
                            },
                        },
                        TokenType::LiteralInteger => ExpressionAST::Literal {
                            0: LiteralAST::Bool {
                                0: expression_token,
                            },
                        },
                        TokenType::LiteralString => ExpressionAST::Literal {
                            0: LiteralAST::Bool {
                                0: expression_token,
                            },
                        },
                        TokenType::Id => ExpressionAST::Variable {
                            0: expression_token,
                        },
                        _ => unreachable!(),
                    },
                },
            });

            status = ParserStatus::TopLevel;
            continue 'parse;
        } else if let ParserStatus::StatementPrint = &status {
            let mut expression_vec = Vec::new();

            loop {
                let expression_token = next(lexer);

                if expression_token.token_type == TokenType::Semicolon {
                    break;
                }

                if expression_token.token_type != TokenType::LiteralBool
                    && expression_token.token_type != TokenType::LiteralInteger
                    && expression_token.token_type != TokenType::LiteralString
                    && expression_token.token_type != TokenType::Id
                {
                    panic!("unexpected token \"{:#?}\" found", expression_token)
                }

                expression_vec.push(match expression_token.token_type {
                    TokenType::LiteralBool => ExpressionAST::Literal {
                        0: LiteralAST::Bool {
                            0: expression_token,
                        },
                    },
                    TokenType::LiteralInteger => ExpressionAST::Literal {
                        0: LiteralAST::Bool {
                            0: expression_token,
                        },
                    },
                    TokenType::LiteralString => ExpressionAST::Literal {
                        0: LiteralAST::Bool {
                            0: expression_token,
                        },
                    },
                    TokenType::Id => ExpressionAST::Variable {
                        0: expression_token,
                    },
                    _ => unreachable!(),
                });
            }

            ast_vec.push(AST::Print {
                0: PrintAST { expression_vec },
            });

            status = ParserStatus::TopLevel;
            continue 'parse;
        } else if let ParserStatus::StatementPrintErr = &status {
            let mut expression_vec = Vec::new();

            loop {
                let expression_token = next(lexer);

                if expression_token.token_type == TokenType::Semicolon {
                    break;
                }

                if expression_token.token_type != TokenType::LiteralBool
                    && expression_token.token_type != TokenType::LiteralInteger
                    && expression_token.token_type != TokenType::LiteralString
                    && expression_token.token_type != TokenType::Id
                {
                    panic!("unexpected token \"{:#?}\" found", expression_token)
                }

                expression_vec.push(match expression_token.token_type {
                    TokenType::LiteralBool => ExpressionAST::Literal {
                        0: LiteralAST::Bool {
                            0: expression_token,
                        },
                    },
                    TokenType::LiteralInteger => ExpressionAST::Literal {
                        0: LiteralAST::Bool {
                            0: expression_token,
                        },
                    },
                    TokenType::LiteralString => ExpressionAST::Literal {
                        0: LiteralAST::Bool {
                            0: expression_token,
                        },
                    },
                    TokenType::Id => ExpressionAST::Variable {
                        0: expression_token,
                    },
                    _ => unreachable!(),
                });
            }

            ast_vec.push(AST::PrintErr {
                0: PrintErrAST { expression_vec },
            });

            status = ParserStatus::TopLevel;
            continue 'parse;
        } else if let ParserStatus::Pipeline { name_token } = &status {
        } else if let ParserStatus::PipelineArgument {
            name_token,
            arguemnt_token_vec,
        } = &mut status
        {
        }
    }
}
