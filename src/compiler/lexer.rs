#[derive(PartialEq)]
enum AdvanceMode {
    Pre,
    Post,
    NoAdvance,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Unknown,
    Eof,
    Id,
    At,              // @
    Comma,           // ,
    Colon,           // :
    Semicolon,       // ;
    Equal,           // =
    ParenL,          // (
    ParenR,          // )
    BraceL,          // {
    BraceR,          // }
    BracketL,        // [
    BracketR,        // ]
    CompareEq,       // ==
    LiteralBool,     // true false
    LiteralInteger,  // 0123456789
    LiteralString,   // "..."
    KeywordImport,   // import
    KeywordFrom,     // from
    KeywordSet,      // set
    KeywordPrint,    // print
    KeywordPrintErr, // printErr
    KeywordNonBlock, // nonblock
    KeywordAwait,    // await
    KeywordAll,      // all
    KeywordIf,       // if
    KeywordElse,     // else
    Comment,         // // ...
}

#[derive(Debug)]
pub struct Token {
    pub file_path: String,
    pub token_type: TokenType,
    pub token_content: String,
    pub line_number: usize,
    pub line_offset: usize,
}

pub struct Lexer {
    file_path: String,
    src_content: Vec<String>,
    content: Vec<char>,
    max_index: usize,
    pub index: usize,
    pub line_offset: usize,
    pub line_number: usize,
}

pub enum LexerError {
    StringNotClosed(Token),
    UnexpectedCharacter(Token),
}

impl Lexer {
    pub fn new(file_path: String, content: String) -> Lexer {
        let content = content.replace("\r\n", "\n");
        let content_chars = content.chars().collect::<Vec<char>>();
        let max_index = content_chars.len();

        Lexer {
            file_path,
            src_content: content
                .split_terminator('\n')
                .map(|line| line.to_owned())
                .collect(),
            content: content_chars,
            max_index,
            index: 0,
            line_offset: 1,
            line_number: 1,
        }
    }

    pub fn src_content(&self) -> &Vec<String> {
        &self.src_content
    }

    fn ch(&self) -> char {
        self.content[self.index]
    }

    fn is_eof(&self) -> bool {
        self.max_index <= self.index
    }

    fn is_whitespace(&self) -> bool {
        self.ch().is_whitespace()
    }

    fn is_punctuation(&self) -> bool {
        self.ch() != '_' && self.ch().is_ascii_punctuation()
    }

    fn is_newline(&self) -> bool {
        self.ch() == '\n'
    }

    fn pick_blackspace(&mut self) -> char {
        loop {
            if self.is_eof() {
                return '\0';
            }

            if !self.is_whitespace() {
                return self.ch();
            }

            self.line_offset += 1;

            if self.is_newline() {
                self.line_offset = 1;
                self.line_number += 1;
            }

            self.index += 1;
        }
    }

    fn next_character(&mut self, advance_mode: AdvanceMode) -> char {
        if self.is_eof() {
            return '\0';
        }

        if advance_mode == AdvanceMode::Pre {
            if !self.is_eof() {
                self.line_offset += 1;

                if self.is_newline() {
                    self.line_offset = 1;
                    self.line_number += 1;
                }

                self.index += 1;
            }

            return if self.is_eof() { '\0' } else { self.ch() };
        }

        let character = self.ch();

        if advance_mode == AdvanceMode::Post {
            if !self.is_eof() {
                self.line_offset += 1;

                if self.is_newline() {
                    self.line_offset = 1;
                    self.line_number += 1;
                }

                self.index += 1;
            }
        }

        character
    }

    fn parse_integer(&mut self) -> Option<Token> {
        let line_offset = self.line_offset;

        let mut integer = "".to_string();

        while self.next_character(AdvanceMode::NoAdvance).is_digit(10) {
            integer.push(self.next_character(AdvanceMode::Post));
        }

        if integer.is_empty() {
            None
        } else {
            Some(Token {
                file_path: self.file_path.clone(),
                token_type: TokenType::LiteralInteger,
                token_content: integer,
                line_offset: line_offset,
                line_number: self.line_number,
            })
        }
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        let blackspace = self.pick_blackspace();

        let mut token = Token {
            file_path: self.file_path.clone(),
            token_type: TokenType::Unknown,
            token_content: "".to_string(),
            line_number: self.line_number,
            line_offset: self.line_offset,
        };

        let return_token =
            |token_type: TokenType, token_content: String| -> Result<Token, LexerError> {
                token.token_type = token_type;
                token.token_content = token_content;

                Ok(token)
            };

        if blackspace == '\0' {
            return return_token(TokenType::Eof, "".to_string());
        }

        match blackspace {
            '@' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::At, blackspace.to_string());
            }
            ',' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::Comma, blackspace.to_string());
            }
            ':' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::Colon, blackspace.to_string());
            }
            ';' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::Semicolon, blackspace.to_string());
            }
            '(' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::ParenL, blackspace.to_string());
            }
            ')' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::ParenR, blackspace.to_string());
            }
            '{' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::BraceL, blackspace.to_string());
            }
            '}' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::BraceR, blackspace.to_string());
            }
            '[' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::BracketL, blackspace.to_string());
            }
            ']' => {
                self.next_character(AdvanceMode::Pre);
                return return_token(TokenType::BracketR, blackspace.to_string());
            }
            '/' => {
                if self.next_character(AdvanceMode::Pre) == '/' {
                    self.next_character(AdvanceMode::Pre);
                    let mut string = "//".to_owned();

                    while !self.is_eof() && !self.is_newline() {
                        string.push(self.next_character(AdvanceMode::Post));
                    }

                    return return_token(TokenType::Comment, string);
                } else {
                    self.index -= 1;
                    self.line_offset -= 1;
                }
            }
            '=' => match self.next_character(AdvanceMode::Pre) {
                '=' => {
                    self.next_character(AdvanceMode::Pre);
                    return return_token(TokenType::CompareEq, blackspace.to_string());
                }
                _ => {
                    return return_token(TokenType::Equal, blackspace.to_string());
                }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                match self.parse_integer() {
                    Some(token) => {
                        return Ok(token);
                    }
                    None => (),
                }
            }
            '"' => {
                self.next_character(AdvanceMode::Pre);

                let mut string = String::new();

                while !self.is_eof() {
                    match self.ch() {
                        '\\' => {
                            self.next_character(AdvanceMode::Pre);

                            if self.is_eof() {
                                return Err(LexerError::StringNotClosed {
                                    0: return_token(TokenType::Unknown, string).ok().unwrap(),
                                });
                            }

                            match self.ch() {
                                'n' => string.push('\n'),
                                'r' => string.push('\r'),
                                't' => string.push('\t'),
                                '\\' => string.push('\\'),
                                '0' => string.push('\0'),
                                '\'' => string.push('\''),
                                '"' => string.push('"'),
                                '`' => string.push('`'),
                                _ => string.push(self.ch()),
                            }

                            self.next_character(AdvanceMode::Pre);
                        }
                        '"' => {
                            break;
                        }
                        _ => string.push(self.next_character(AdvanceMode::Post)),
                    }
                }

                if self.ch() != '"' {
                    return Err(LexerError::StringNotClosed {
                        0: return_token(TokenType::Unknown, string).ok().unwrap(),
                    });
                }

                self.next_character(AdvanceMode::Pre);

                return return_token(TokenType::LiteralString, string);
            }
            _ => (),
        }

        if self.is_punctuation() {
            return Err(LexerError::UnexpectedCharacter {
                0: {
                    return_token(
                        TokenType::Unknown,
                        self.next_character(AdvanceMode::Post).to_string(),
                    )
                    .ok()
                    .unwrap()
                },
            });
        }

        let mut content = String::new();

        while !self.is_eof() && !self.is_whitespace() && !self.is_punctuation() {
            content.push(self.next_character(AdvanceMode::Post));
        }

        match content.as_ref() {
            "true" => return_token(TokenType::LiteralBool, content),
            "false" => return_token(TokenType::LiteralBool, content),
            "import" => return_token(TokenType::KeywordImport, content),
            "from" => return_token(TokenType::KeywordFrom, content),
            "set" => return_token(TokenType::KeywordSet, content),
            "print" => return_token(TokenType::KeywordPrint, content),
            "printErr" => return_token(TokenType::KeywordPrintErr, content),
            "nonblock" => return_token(TokenType::KeywordNonBlock, content),
            "await" => return_token(TokenType::KeywordAwait, content),
            "all" => return_token(TokenType::KeywordAll, content),
            "if" => return_token(TokenType::KeywordIf, content),
            "else" => return_token(TokenType::KeywordElse, content),
            _ => return_token(TokenType::Id, content),
        }
    }
}
