use super::span::Span;

// Represents the category of a lexical error in MeliLang
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerErrorKind {
    UnexpectedCharacter { character: char }, // An unexpected character was encountered

    UnterminatedString,    // A string literal was not properly terminated
    UnterminatedCharacter, // A character literal was not properly terminated

    InvalidCharacterLiteral, // A character literal contains an invalid number of characters
    InvalidNumber,           // A numeric literal has an invalid format
    InvalidEscapeSequence { sequence: String }, // A string or character literal contains an unsupported escape sequence
    InvalidIndentation, // An indentitation level does not match a previoud indentation level

    MixedIndentation, // Tabs and spaces are mixed within the same source file
}

// Represents a lexical error detected by the MeliLang lexer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub kind: LexerErrorKind, // The category of the lexical error
    pub span: Span,           // The source location associated with the error
}

impl LexerError {
    // Creates a new lexical error
    pub fn new(kind: LexerErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match &self.kind {
            LexerErrorKind::UnexpectedCharacter { character } => {
                format!("unexpected character '{}'", character)
            }

            LexerErrorKind::UnterminatedString => "unterminated string literal".to_string(),

            LexerErrorKind::UnterminatedCharacter => "unterminated character literal".to_string(),

            LexerErrorKind::InvalidCharacterLiteral => "invalid character literal".to_string(),

            LexerErrorKind::InvalidNumber => "invalid numeric literal".to_string(),

            LexerErrorKind::InvalidEscapeSequence { sequence } => {
                format!("invalid escape sequence '{}'", sequence)
            }

            LexerErrorKind::InvalidIndentation => "invalid indentation level".to_string(),

            LexerErrorKind::MixedIndentation => "mixed tabs and spaces in indentation".to_string(),
        };

        write!(
            f,
            "{} at line {}, column {}",
            message, self.span.start.line, self.span.start.column
        )
    }
}

impl std::error::Error for LexerError {}
