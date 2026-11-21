use melilang::lexer::{Cursor, Position};

// ==========================================================
// UNICODE TEST
// ==========================================================

#[test]
fn test_unicode_position_tracking() {
    let mut cursor = Cursor::new("aé🧡b");

    assert_eq!(cursor.advanced(), Some('a'));
    assert_eq!(
        cursor.position(),
        Position {
            offset: 1,
            line: 1,
            column: 2,
        }
    );

    assert_eq!(cursor.advanced(), Some('é'));
    assert_eq!(
        cursor.position(),
        Position {
            offset: 3,
            line: 1,
            column: 3,
        }
    );

    assert_eq!(cursor.advanced(), Some('🧡'));
    assert_eq!(
        cursor.position(),
        Position {
            offset: 7,
            line: 1,
            column: 4,
        }
    );

    assert_eq!(cursor.advanced(), Some('b'));
    assert_eq!(
        cursor.position(),
        Position {
            offset: 8,
            line: 1,
            column: 5,
        }
    );
}
