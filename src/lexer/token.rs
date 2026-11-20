use super::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Declaration and control-flow keywords
    Let,
    Var,
    Set,
    Define,
    Function,
    Returns,
    When,
    Otherwise,
    Return,
    Show,
    Use,
    Repeat,
    For,
    Each,
    While,
    In,
    Be,
    To,
    As,

    // Comparison keywords
    Is,
    Equal,
    Greater,
    Less,
    Than,
    At,
    Least,
    Most,

    // Logical keywords
    And,
    Or,
    Not,

    // Boolean literals
    True,
    False,

    // Identifiers and literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    MatrixMultiply,

    // Comparison operators
    // Reserved for parsed / AST representation
    // The Lexer emits individual comparison keywords tokens
    EqualEqual,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,

    // Asignment
    Assign,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Colon,
    Comma,
    Dot,
    Range,
    Arrow,

    // Structural tokens
    Newline,
    Indent,
    Dedent,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}
