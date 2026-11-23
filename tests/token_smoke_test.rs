use melilang::lexer::span::Span;
use melilang::lexer::{Lexer, TokenKind};

// ==========================================================
// TOKEN SMOKE TEST
// ==========================================================

#[test]
fn test_keywords_and_identifiers() {
    let mut lexer = Lexer::new("let learning_rate be\n");

    let expected = [
        (TokenKind::Let, "let"),
        (TokenKind::Identifier, "learning_rate"),
        (TokenKind::Be, "be"),
        (TokenKind::Newline, "\n"),
        (TokenKind::Eof, ""),
    ];

    for (kind, lexeme) in expected {
        let token = lexer.next_token().unwrap();

        assert_eq!(token.kind, kind);
        assert_eq!(token.lexeme, lexeme);
    }
}

#[test]
fn test_comparison_words_are_individual_tokens() {
    let mut lexer = Lexer::new("accuracy is at least");

    let kinds: Vec<TokenKind> = (0..4).map(|_| lexer.next_token().unwrap().kind).collect();

    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier,
            TokenKind::Is,
            TokenKind::At,
            TokenKind::Least,
        ]
    );
}

#[test]
fn test_unexpected_character_error_has_span() {
    let mut lexer = Lexer::new("$");

    let error = lexer.next_token().unwrap_err();

    assert_eq!(error.span.start.line, 1);
    assert_eq!(error.span.start.column, 1);
}

#[test]
fn test_span_is_shared_type() {
    // lexer::Span and span::Span must be the very same type.
    let token = Lexer::new("meli").next_token().unwrap();
    let span: Span = token.span;

    assert_eq!(span.len(), 4);
}
