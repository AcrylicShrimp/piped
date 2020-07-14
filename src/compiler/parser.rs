use super::lexer::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum AST {
    Set(SetAST),
    Print(SetAST),
    PrintErr(SetAST),
}

#[derive(Debug)]
pub struct SetAST {
    pub name: Token,
    pub value: ExpressionAST,
}

#[derive(Debug)]
pub struct PrintAST {
    pub expressionVec: Vec<ExpressionAST>,
}

#[derive(Debug)]
pub struct PrintErrAST {
    pub expressionVec: Vec<ExpressionAST>,
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
    Pipeline {
        nameToken: Token,
    },
    PipelineArgument {
        nameToken: Token,
        arguemntTokenVec: Vec<Token>,
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
                status = ParserStatus::Pipeline { nameToken: token };
                continue 'parse;
            }

            panic!("unexpected token \"{:#?}\" found", token);
        } else if let ParserStatus::Statement = &status {
            let statementToken = next(lexer);

            match statementToken.token_type {
                TokenType::KeywordSet => {
                    status = ParserStatus::StatementSet;
                    continue 'parse;
                }
                _ => panic!("unexpected token \"{:#?}\" found", statementToken),
            }
        } else if let ParserStatus::StatementSet = &status {
            let nameToken = next_token(lexer, TokenType::Id);

            next_token(lexer, TokenType::Equal);

            let expressionToken = next(lexer);

            if expressionToken.token_type != TokenType::LiteralBool
                && expressionToken.token_type != TokenType::LiteralInteger
                && expressionToken.token_type != TokenType::LiteralString
                && expressionToken.token_type != TokenType::Id
            {
                panic!("unexpected token \"{:#?}\" found", expressionToken)
            }

            next_token(lexer, TokenType::Semicolon);

            ast_vec.push(AST::Set {
                0: SetAST {
                    name: nameToken,
                    value: match expressionToken.token_type {
                        TokenType::LiteralBool => ExpressionAST::Literal {
                            0: LiteralAST::Bool { 0: expressionToken },
                        },
                        TokenType::LiteralInteger => ExpressionAST::Literal {
                            0: LiteralAST::Bool { 0: expressionToken },
                        },
                        TokenType::LiteralString => ExpressionAST::Literal {
                            0: LiteralAST::Bool { 0: expressionToken },
                        },
                        TokenType::Id => ExpressionAST::Variable { 0: expressionToken },
                        _ => unreachable!(),
                    },
                },
            });

            status = ParserStatus::TopLevel;
            continue 'parse;
        } else if let ParserStatus::Pipeline { nameToken } = &status {
        } else if let ParserStatus::PipelineArgument {
            nameToken,
            arguemntTokenVec,
        } = &mut status
        {
        }
    }
}
