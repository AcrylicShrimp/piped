use super::LowTokenNumberLiteralKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LowTokenLiteralKind {
    Number {
        kind: LowTokenNumberLiteralKind,
        suffix_start: u32,
    },
    SingleQuotedString {
        terminated: bool,
    },
    DoubleQuotedString {
        terminated: bool,
    },
}
