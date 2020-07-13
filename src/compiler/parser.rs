use super::lexer::{Lexer, Token, TokenType};

pub enum AST {
    Set(SetAST),
}

pub struct SetAST {
    pub name: String,
    pub value: ExpressionAST,
}

pub enum ExpressionAST {
    Literal(LiteralAST),
    Variable(Token),
}

pub enum LiteralAST {
    Bool(Token),
    Integer(Token),
    String(Token),
}

enum ParserStatus {
    TopLevel,
    Statement { atToken: Token },
    Pipeline { nameToken: Token },
}

fn parse(lexer: &mut Lexer) -> Vec<AST> {
    fn next_token(lexer: &mut Lexer, token_type: TokenType) -> Token {
        let token = lexer.next();

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
            let token = lexer.next();

            if token.token_type == TokenType::Eof {
                return ast_vec;
            }

            if token.token_type == TokenType::At {
                status = ParserStatus::Statement { atToken: token };
                continue 'parse;
            }

            if token.token_type == TokenType::Id {
                status = ParserStatus::Pipeline { nameToken: token };
                continue 'parse;
            }

            panic!("unexpected token \"{:#?}\" found", token);
        } else if let ParserStatus::Statement { atToken } = &status {
            let token = lexer.next();

            match token.token_type {
                TokenType::KeywordSet => {
                    let nameToken = next_token(lexer, TokenType::Id);
                    next_token(lexer, TokenType::Equal);
                }
                _ => panic!("unexpected token \"{:#?}\" found", token),
            }
        } else if let ParserStatus::Pipeline { nameToken } = &status {
        }
    }
}
