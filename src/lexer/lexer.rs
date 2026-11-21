use std::collections::VecDeque;

use crate::lexer::Token;

use super::cursor::Cursor;
use super::span::Position;

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
}
