use super::span::Position;

// Reads characters from MeliLang source code
// Cursor is responsible only for source navigation and position tracking.
// It doses not recognize tokens.
pub struct Cursor<'src> {
    source: &'src str,
    position: Position,
    previous_was_cr: bool,
}

impl<'src> Cursor<'src> {
    // Create a new Cursor
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            position: Position {
                offset: 0,
                line: 1,
                column: 1,
            },
            previous_was_cr: false,
        }
    }

    // Returns the current position
    pub fn position(&self) -> Position {
        self.position
    }

    // Returns true when the source has been fully consumed
    pub fn is_at_end(&self) -> bool {
        self.position.offset >= self.source.len()
    }

    // Returns the current character without comsuming it
    pub fn peek(&self) -> Option<char> {
        self.source[self.position.offset..].chars().next()
    }

    // Returns the next character without consuming it
    pub fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.position.offset..].chars();

        chars.next()?;
        chars.next()
    }

    // Consumes and returns the current character
    pub fn advanced(&mut self) -> Option<char> {
        let ch = self.peek()?;

        self.position.offset += ch.len_utf8();

        match ch {
            '\r' => {
                self.position.line += 1;
                self.position.column = 1;
                self.previous_was_cr = true;
            }

            '\n' => {
                if !self.previous_was_cr {
                    self.position.line += 1;
                }

                self.position.column = 1;
                self.previous_was_cr = false;
            }

            _ => {
                self.position.column += 1;
                self.previous_was_cr = false;
            }
        }

        Some(ch)
    }

    // Returns a source slice using a Span
    pub fn slice(&self, start: Position, end: Position) -> Option<&'src str> {
        self.source.get(start.offset..end.offset)
    }
}
