// Represents a position within the MeliLang source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub offset: usize, // Byte offset in the UTF-8 source code.
    pub line: usize,   // Line number, starting from 1.
    pub column: usize, // Column number, starting from 1.
}

// Represents a soure range
// Start is inclusive and end is exclusive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub const fn empty(position: Position) -> Self {
        Self {
            start: position,
            end: position,
        }
    }

    pub const fn len(&self) -> usize {
        self.end.offset - self.start.offset
    }

    pub const fn is_empty(&self) -> bool {
        self.start.offset == self.end.offset
    }
}