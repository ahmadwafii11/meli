use melilang::lexer::{Cursor, Position, Span};

// ==========================================================
// SOURCE SLICING TEST
// ==========================================================

#[test]
fn test_cursor_slice() {
    let cursor = Cursor::new("aébc");

    let start = Position {
        offset: 1,
        line: 1,
        column: 2,
    };

    let end = Position {
        offset: 3,
        line: 1,
        column: 3,
    };

    assert_eq!(cursor.slice(start, end), Some("é"));
}

#[test]
fn test_cursor_slice_invalid_boundary() {
    let cursor = Cursor::new("aébc");

    let start = Position {
        offset: 2,
        line: 1,
        column: 3,
    };

    let end = Position {
        offset: 3,
        line: 1,
        column: 4,
    };

    // Offset 2 berada di tengah byte karakter "é".
    assert_eq!(cursor.slice(start, end), None);
}

#[test]
fn test_cursor_slice_empty_span() {
    let cursor = Cursor::new("meli");

    let position = Position {
        offset: 2,
        line: 1,
        column: 3,
    };

    let span = Span::empty(position);

    assert_eq!(cursor.slice(span.start, span.end), Some(""));
}
