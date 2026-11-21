use melilang::lexer::{Cursor, Position};

// ==========================================================
// NEWLINE TEST
// ==========================================================

#[test]
fn test_lf_line_ending() {
    let mut cursor = Cursor::new("a\nb");

    cursor.advanced();
    cursor.advanced();

    assert_eq!(
        cursor.position(),
        Position {
            offset: 2,
            line: 2,
            column: 1,
        }
    );

    cursor.advanced();

    assert_eq!(
        cursor.position(),
        Position {
            offset: 3,
            line: 2,
            column: 2,
        }
    );
}

#[test]
fn test_cr_line_ending() {
    let mut cursor = Cursor::new("a\rb");

    cursor.advanced();
    cursor.advanced();

    assert_eq!(
        cursor.position(),
        Position {
            offset: 2,
            line: 2,
            column: 1,
        }
    );

    cursor.advanced();

    assert_eq!(
        cursor.position(),
        Position {
            offset: 3,
            line: 2,
            column: 2,
        }
    );
}

#[test]
fn test_crlf_line_ending() {
    let mut cursor = Cursor::new("a\r\nb");

    cursor.advanced();
    cursor.advanced();

    assert_eq!(
        cursor.position(),
        Position {
            offset: 2,
            line: 2,
            column: 1,
        }
    );

    cursor.advanced();

    assert_eq!(
        cursor.position(),
        Position {
            offset: 3,
            line: 2,
            column: 1,
        }
    );

    cursor.advanced();

    assert_eq!(
        cursor.position(),
        Position {
            offset: 4,
            line: 2,
            column: 2,
        }
    );
}
