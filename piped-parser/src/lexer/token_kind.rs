use super::TokenLiteral;
use piped_symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Unknown { symbol: Symbol },
    Comment,      // "#"
    OpenParen,    // "("
    CloseParen,   // ")"
    OpenBrace,    // "{"
    CloseBrace,   // "}"
    OpenBracket,  // "["
    CloseBracket, // "]"
    Dot,          // "."
    Comma,        // ","
    Colon,        // ":"
    Semicolon,    // ";"
    Arrow,        // "->"
    // Assignment operators
    Assign,       // "="
    AssignAdd,    // "+="
    AssignSub,    // "-="
    AssignMul,    // "*="
    AssignDiv,    // "/="
    AssignMod,    // "%="
    AssignPow,    // "**="
    AssignShl,    // "<<="
    AssignShr,    // ">>="
    AssignBitOr,  // "|="
    AssignBitAnd, // "&="
    AssignBitXor, // "^="
    AssignBitNot, // "~="
    // Range operators
    Rng,          // ".."
    RngInclusive, // "..="
    // Cmp operators
    Eq, // "=="
    Ne, // "!="
    Lt, // "<"
    Gt, // ">"
    Le, // "<="
    Ge, // ">="
    // Binary operators
    Add,    // "+"
    Sub,    // "-"
    Mul,    // "*"
    Div,    // "/"
    Mod,    // "%"
    Pow,    // "**"
    Shl,    // "<<"
    Shr,    // ">>"
    BitOr,  // "|"
    BitAnd, // "&"
    BitXor, // "^"
    LogOr,  // "||"
    LogAnd, // "&&"
    // Unary operators
    BitNot, // "~"
    LogNot, // "!"
    // Module access operators
    ModuleMember, // "::"
    Id { symbol: Symbol },
    Literal(TokenLiteral),
}

impl TokenKind {
    pub fn into_symbol(self) -> Symbol {
        match self {
            TokenKind::Unknown { .. } => *super::UNKNOWN,
            TokenKind::Comment => *super::COMMENT,
            TokenKind::OpenParen => *super::OPEN_PAREN,
            TokenKind::CloseParen => *super::CLOSE_PAREN,
            TokenKind::OpenBrace => *super::OPEN_BRACE,
            TokenKind::CloseBrace => *super::CLOSE_BRACE,
            TokenKind::OpenBracket => *super::OPEN_BRACKET,
            TokenKind::CloseBracket => *super::CLOSE_BRACKET,
            TokenKind::Dot => *super::DOT,
            TokenKind::Comma => *super::COMMA,
            TokenKind::Colon => *super::COLON,
            TokenKind::Semicolon => *super::SEMICOLON,
            TokenKind::Arrow => *super::ARROW,
            TokenKind::Assign => *super::ASSIGN,
            TokenKind::AssignAdd => *super::ASSIGN_ADD,
            TokenKind::AssignSub => *super::ASSIGN_SUB,
            TokenKind::AssignMul => *super::ASSIGN_MUL,
            TokenKind::AssignDiv => *super::ASSIGN_DIV,
            TokenKind::AssignMod => *super::ASSIGN_MOD,
            TokenKind::AssignPow => *super::ASSIGN_POW,
            TokenKind::AssignShl => *super::ASSIGN_SHL,
            TokenKind::AssignShr => *super::ASSIGN_SHR,
            TokenKind::AssignBitOr => *super::ASSIGN_BIT_OR,
            TokenKind::AssignBitAnd => *super::ASSIGN_BIT_AND,
            TokenKind::AssignBitXor => *super::ASSIGN_BIT_XOR,
            TokenKind::AssignBitNot => *super::ASSIGN_BIT_NOT,
            TokenKind::Rng => *super::RNG,
            TokenKind::RngInclusive => *super::RNG_INCLUSIVE,
            TokenKind::Eq => *super::EQ,
            TokenKind::Ne => *super::NE,
            TokenKind::Lt => *super::LT,
            TokenKind::Gt => *super::GT,
            TokenKind::Le => *super::LE,
            TokenKind::Ge => *super::GE,
            TokenKind::Add => *super::ADD,
            TokenKind::Sub => *super::SUB,
            TokenKind::Mul => *super::MUL,
            TokenKind::Div => *super::DIV,
            TokenKind::Mod => *super::MOD,
            TokenKind::Pow => *super::POW,
            TokenKind::Shl => *super::SHL,
            TokenKind::Shr => *super::SHR,
            TokenKind::BitOr => *super::BIT_OR,
            TokenKind::BitAnd => *super::BIT_AND,
            TokenKind::BitXor => *super::BIT_XOR,
            TokenKind::LogOr => *super::LOG_OR,
            TokenKind::LogAnd => *super::LOG_AND,
            TokenKind::BitNot => *super::BIT_NOT,
            TokenKind::LogNot => *super::LOG_NOT,
            TokenKind::ModuleMember => *super::MODULE_MEMBER,
            TokenKind::Id { .. } => *super::ID,
            TokenKind::Literal(..) => *super::LITERAL,
        }
    }
}
