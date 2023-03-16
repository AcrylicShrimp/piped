use piped_span::Span;
use piped_symbol::Symbol;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Id {
    pub symbol: Symbol,
    pub span: Span,
}
