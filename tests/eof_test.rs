use melilang::lexer::Cursor;

// ==========================================================
// EOF TEST
// ==========================================================

#[test]
fn test_cursor_eof() {
    let mut cursor = Cursor::new("meli");

    for _ in 0..4 {
        assert!(cursor.advanced().is_some());
    }

    let position_before = cursor.position();

    assert!(cursor.is_at_end());
    assert_eq!(cursor.peek(), None);
    assert_eq!(cursor.peek_next(), None);
    assert_eq!(cursor.advanced(), None);

    assert_eq!(cursor.position(), position_before);
}
