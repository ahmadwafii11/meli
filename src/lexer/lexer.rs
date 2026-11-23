use std::collections::VecDeque;

use super::cursor::Cursor;
use super::{LexerError, LexerErrorKind};
use super::{Token, TokenKind};

use crate::lexer::span::{Position, Span};

// Converts MeliLang source code into a token stream
pub struct Lexer<'src> {
    cursor: Cursor<'src>,
    pending_tokens: VecDeque<Token>, // Pending tokens, especially INDENT and DEDENT
    indentation_stack: Vec<usize>,   // Tracks indentation levels
    at_line_start: bool,             // Tracks whether the current logical line has code
    eof_emitted: bool,               // Prevents repeated EOF token generation if needed
}

impl<'src> Lexer<'src> {
    // Creates a new Lexer from source code
    pub fn new(source: &'src str) -> Self {
        Self {
            cursor: Cursor::new(source),
            pending_tokens: VecDeque::new(),
            indentation_stack: vec![0],
            at_line_start: true,
            eof_emitted: false,
        }
    }

    // Returns the current source position
    pub fn position(&self) -> Position {
        self.cursor.position()
    }

    // Returns true if the source has been fully consumed
    pub fn is_at_end(&self) -> bool {
        self.cursor.is_at_end()
    }

    // Retuns the current character without consuming it
    pub fn peek(&self) -> Option<char> {
        self.cursor.peek()
    }

    // Returns the next character without consuming it
    pub fn peek_next(&self) -> Option<char> {
        self.cursor.peek_next()
    }

    // Consumes and returns the current character
    pub fn advanced(&mut self) -> Option<char> {
        self.cursor.advanced()
    }

    // Produces the next token or a lexical error
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        // Return pending tokens first.
        if let Some(token) = self.pending_tokens.pop_front() {
            return Ok(token);
        }

        // EOF is stable after being emitted.
        if self.eof_emitted {
            return Ok(self.make_empty_token(TokenKind::Eof));
        }

        loop {
            match self.cursor.peek() {
                // Ignore horizontal whitespace for this initial stage.
                Some(' ' | '\t') => {
                    self.cursor.advanced();
                }

                // Logical line separator.
                Some('\n' | '\r') => {
                    return Ok(self.scan_newline());
                }

                // Single-line comment.
                Some('#') => {
                    self.skip_comment();
                }

                // Identifier or reserved keyword.
                Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                    return Ok(self.scan_identifier());
                }

                // Numeric literal (integer, float, or scientific notation).
                Some(ch) if ch.is_ascii_digit() => {
                    return self.scan_number();
                }

                // String literal.
                Some('"') => {
                    return self.scan_quoted('"');
                }

                // Character literal.
                Some('\'') => {
                    return self.scan_quoted('\'');
                }

                // Operators and delimiters.
                Some(
                    '+' | '-' | '*' | '/' | '%' | '@' | '=' | '(' | ')' | '[' | ']' | '{' | '}'
                    | ':' | ',' | '.',
                ) => {
                    return Ok(self.scan_symbol());
                }

                // End of source.
                None => {
                    return Ok(self.finish_eof());
                }

                // Unsupported character in this implementation stage.
                Some(character) => {
                    let start = self.cursor.position();

                    self.cursor.advanced();

                    let end = self.cursor.position();

                    let span = Span::new(start, end);

                    return Err(LexerError::new(
                        LexerErrorKind::UnexpectedCharacter { character },
                        span,
                    ));
                }
            }
        }
    }

    fn make_token(&self, kind: TokenKind, start: Position, end: Position) -> Token {
        let lexeme = self.cursor.slice(start, end).unwrap_or("").to_string();

        Token {
            kind,
            lexeme,
            span: Span::new(start, end),
        }
    }

    fn make_empty_token(&self, kind: TokenKind) -> Token {
        let position = self.cursor.position();

        Token {
            kind,
            lexeme: String::new(),
            span: Span::empty(position),
        }
    }

    fn scan_newline(&mut self) -> Token {
        let start = self.cursor.position();

        match self.cursor.peek() {
            Some('\r') => {
                self.cursor.advanced();

                if self.cursor.peek() == Some('\n') {
                    self.cursor.advanced();
                }
            }

            Some('\n') => {
                self.cursor.advanced();
            }

            _ => unreachable!("Expected newline character"),
        }

        let end = self.cursor.position();

        self.at_line_start = true;

        self.make_token(TokenKind::Newline, start, end)
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.cursor.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }

            self.cursor.advanced();
        }
    }

    fn scan_identifier(&mut self) -> Token {
        let start = self.cursor.position();

        self.cursor.advanced();

        while let Some(ch) = self.cursor.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.cursor.advanced();
            } else {
                break;
            }
        }

        let end = self.cursor.position();

        let lexeme = self.cursor.slice(start, end).unwrap_or("");

        let kind = Self::keyword_kind(lexeme).unwrap_or(TokenKind::Identifier);

        self.at_line_start = false;

        self.make_token(kind, start, end)
    }

    // Scans a numeric literal.
    fn scan_number(&mut self) -> Result<Token, LexerError> {
        let start = self.cursor.position();

        self.consume_digits();

        let mut is_float = false;

        // Fractional part.
        if self.cursor.peek() == Some('.') {
            match self.cursor.peek_next() {
                Some(next) if next.is_ascii_digit() => {
                    self.cursor.advanced(); // '.'
                    self.consume_digits();
                    is_float = true;
                }

                // `..` is the range operator and ends the number here.
                Some('.') => {}

                // A decimal point must be followed by a digit (`1.` is invalid).
                _ => {
                    self.cursor.advanced(); // '.'
                    return Err(self.invalid_number(start));
                }
            }
        }

        // Exponent part.
        if matches!(self.cursor.peek(), Some('e' | 'E')) {
            self.cursor.advanced();

            if matches!(self.cursor.peek(), Some('+' | '-')) {
                self.cursor.advanced();
            }

            if !matches!(self.cursor.peek(), Some(ch) if ch.is_ascii_digit()) {
                return Err(self.invalid_number(start));
            }

            self.consume_digits();
            is_float = true;
        }

        // A number must not run into a letter, underscore, or second decimal point.
        let glued = match self.cursor.peek() {
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => true,
            Some('.') => matches!(self.cursor.peek_next(), Some(next) if next.is_ascii_digit()),
            _ => false,
        };

        if glued {
            return Err(self.invalid_number(start));
        }

        let end = self.cursor.position();

        self.at_line_start = false;

        let kind = if is_float {
            TokenKind::FloatLiteral
        } else {
            TokenKind::IntegerLiteral
        };

        Ok(self.make_token(kind, start, end))
    }

    // Consumes zero or more ASCII digits.
    fn consume_digits(&mut self) {
        while matches!(self.cursor.peek(), Some(ch) if ch.is_ascii_digit()) {
            self.cursor.advanced();
        }
    }

    // Builds an InvalidNumber error for the literal that began at `start`.
    fn invalid_number(&mut self, start: Position) -> LexerError {
        while let Some(ch) = self.cursor.peek() {
            let continues = ch.is_ascii_alphanumeric()
                || ch == '_'
                || (ch == '.'
                    && matches!(self.cursor.peek_next(), Some(next) if next.is_ascii_digit()));

            if !continues {
                break;
            }

            self.cursor.advanced();
        }

        let end = self.cursor.position();

        self.at_line_start = false;

        LexerError::new(LexerErrorKind::InvalidNumber, Span::new(start, end))
    }

    // Scans a string literal (`"..."`) or a character literal (`'...'`).
    fn scan_quoted(&mut self, quote: char) -> Result<Token, LexerError> {
        let start = self.cursor.position();
        let is_char = quote == '\'';

        self.cursor.advanced(); // opening quote

        let mut character_count = 0usize; // an escape sequence counts as one character
        let mut first_error: Option<LexerError> = None;
        let mut terminated = false;

        loop {
            match self.cursor.peek() {
                // A literal cannot span lines.
                None | Some('\n' | '\r') => break,

                Some(ch) if ch == quote => {
                    self.cursor.advanced();
                    terminated = true;
                    break;
                }

                Some('\\') => {
                    let escape_start = self.cursor.position();

                    self.cursor.advanced(); // backslash

                    // A backslash at the end of the line leaves the literal unterminated.
                    let Some(escaped) = self.cursor.peek() else {
                        break;
                    };

                    if escaped == '\n' || escaped == '\r' {
                        break;
                    }

                    self.cursor.advanced();
                    character_count += 1;

                    if !Self::is_valid_escape(escaped) && first_error.is_none() {
                        let escape_end = self.cursor.position();
                        let sequence = self
                            .cursor
                            .slice(escape_start, escape_end)
                            .unwrap_or("")
                            .to_string();

                        first_error = Some(LexerError::new(
                            LexerErrorKind::InvalidEscapeSequence { sequence },
                            Span::new(escape_start, escape_end),
                        ));
                    }
                }

                Some(_) => {
                    self.cursor.advanced();
                    character_count += 1;
                }
            }
        }

        let end = self.cursor.position();

        self.at_line_start = false;

        if let Some(error) = first_error {
            return Err(error);
        }

        let span = Span::new(start, end);

        if !terminated {
            let kind = if is_char {
                LexerErrorKind::UnterminatedCharacter
            } else {
                LexerErrorKind::UnterminatedString
            };

            return Err(LexerError::new(kind, span));
        }

        if is_char && character_count != 1 {
            return Err(LexerError::new(
                LexerErrorKind::InvalidCharacterLiteral,
                span,
            ));
        }

        let kind = if is_char {
            TokenKind::CharLiteral
        } else {
            TokenKind::StringLiteral
        };

        Ok(self.make_token(kind, start, end))
    }

    // Returns true for the character that may follow a backslash.
    fn is_valid_escape(ch: char) -> bool {
        matches!(ch, 'n' | 't' | 'r' | '\\' | '"' | '\'' | '0')
    }

    // Scans an operator or delimiter.
    fn scan_symbol(&mut self) -> Token {
        let start = self.cursor.position();

        let first = self
            .cursor
            .advanced()
            .expect("scan_symbol requires a current character");

        let kind = match first {
            '+' => TokenKind::Plus,
            '-' => {
                if self.cursor.peek() == Some('>') {
                    self.cursor.advanced();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '@' => TokenKind::MatrixMultiply,
            '=' => TokenKind::Assign,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => {
                if self.cursor.peek() == Some('.') {
                    self.cursor.advanced();
                    TokenKind::Range
                } else {
                    TokenKind::Dot
                }
            }
            _ => unreachable!("scan_symbol called on a non-symbol character"),
        };

        let end = self.cursor.position();

        self.at_line_start = false;

        self.make_token(kind, start, end)
    }

    fn keyword_kind(keyword: &str) -> Option<TokenKind> {
        Some(match keyword {
            "let" => TokenKind::Let,
            "var" => TokenKind::Var,
            "set" => TokenKind::Set,
            "define" => TokenKind::Define,
            "function" => TokenKind::Function,
            "returns" => TokenKind::Returns,
            "when" => TokenKind::When,
            "otherwise" => TokenKind::Otherwise,
            "return" => TokenKind::Return,
            "show" => TokenKind::Show,
            "use" => TokenKind::Use,
            "repeat" => TokenKind::Repeat,
            "for" => TokenKind::For,
            "each" => TokenKind::Each,
            "while" => TokenKind::While,
            "in" => TokenKind::In,
            "be" => TokenKind::Be,
            "to" => TokenKind::To,
            "as" => TokenKind::As,

            "is" => TokenKind::Is,
            "equal" => TokenKind::Equal,
            "greater" => TokenKind::Greater,
            "less" => TokenKind::Less,
            "than" => TokenKind::Than,
            "at" => TokenKind::At,
            "least" => TokenKind::Least,
            "most" => TokenKind::Most,

            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,

            "true" => TokenKind::True,
            "false" => TokenKind::False,

            _ => return None,
        })
    }

    fn finish_eof(&mut self) -> Token {
        if self.indentation_stack.len() > 1 {
            self.indentation_stack.pop();

            return self.make_empty_token(TokenKind::Dedent);
        }

        self.eof_emitted = true;

        self.make_empty_token(TokenKind::Eof)
    }
}
