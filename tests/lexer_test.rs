use melilang::lexer::Lexer;

// ==========================================================
// LEXER TEST
// ==========================================================

#[test]
fn test_lexer_initialization() {
    let lexer = Lexer::new("meli");

    assert_eq!(lexer.peek(), Some('m'));
    assert_eq!(lexer.position().offset, 0);
}

#[test]
fn test_lexer_advance() {
    let mut lexer = Lexer::new("meli");

    assert_eq!(lexer.advanced(), Some('m'));
    assert_eq!(lexer.peek(), Some('e'));
    assert_eq!(lexer.position().offset, 1);
}

#[test]
fn test_lexer_peek_next() {
    let lexer = Lexer::new("meli");

    assert_eq!(lexer.peek_next(), Some('e'));
}

#[test]
fn test_lexer_end_of_source() {
    let mut lexer = Lexer::new("M");

    lexer.advanced();

    assert!(lexer.is_at_end());
    assert_eq!(lexer.peek(), None);
}
