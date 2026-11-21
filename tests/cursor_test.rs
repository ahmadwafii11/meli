use melilang::lexer::{Cursor, Position};

// ==========================================================
// CURSOR TEST
// ==========================================================
#[test]
fn test_cursor_initial_position() {
    let cursor = Cursor::new("hello");

    assert_eq!(
        cursor.position(),
        Position {
            offset: 0,
            line: 1,
            column: 1,
        }
    );
}

#[test]
fn test_peek_does_not_advance() {
    let cursor = Cursor::new("meli");

    assert_eq!(cursor.peek(), Some('m'));
    assert_eq!(cursor.position().offset, 0);
}

#[test]
fn test_advance_updates_position() {
    let mut cursor = Cursor::new("meli");

    assert_eq!(cursor.advanced(), Some('m'));

    assert_eq!(
        cursor.position(),
        Position {
            offset: 1,
            line: 1,
            column: 2,
        }
    );
}

#[test]
fn test_peek_next() {
    let cursor = Cursor::new("meli");

    assert_eq!(cursor.peek(), Some('m'));
    assert_eq!(cursor.peek_next(), Some('e'));
}
