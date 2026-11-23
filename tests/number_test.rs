use melilang::lexer::{Lexer, LexerError, LexerErrorKind, TokenKind};

// ==========================================================
// NUMERIC LITERAL TEST
// ==========================================================

// Collects (kind, lexeme) pairs until Eof, excluding the Eof token itself.
fn lex(source: &str) -> Vec<(TokenKind, String)> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().expect("unexpected lexer error");

        if token.kind == TokenKind::Eof {
            break;
        }

        tokens.push((token.kind, token.lexeme));
    }

    tokens
}

fn kinds(source: &str) -> Vec<TokenKind> {
    lex(source).into_iter().map(|(kind, _)| kind).collect()
}

// Returns the first lexical error produced for the source.
fn first_error(source: &str) -> LexerError {
    let mut lexer = Lexer::new(source);

    loop {
        match lexer.next_token() {
            Ok(token) if token.kind == TokenKind::Eof => {
                panic!("expected a lexer error for {source:?}")
            }
            Ok(_) => {}
            Err(error) => return error,
        }
    }
}

fn assert_single(source: &str, kind: TokenKind) {
    assert_eq!(
        lex(source),
        vec![(kind, source.to_string())],
        "source: {source:?}"
    );
}

fn assert_invalid_number(source: &str) {
    let error = first_error(source);

    assert_eq!(
        error.kind,
        LexerErrorKind::InvalidNumber,
        "source: {source:?}"
    );
}

// ----------------------------------------------------------
// Integer literals
// ----------------------------------------------------------

#[test]
fn test_integer_literals() {
    for source in ["0", "7", "42", "100", "224", "007", "1234567890"] {
        assert_single(source, TokenKind::IntegerLiteral);
    }
}

#[test]
fn test_integer_is_not_range_checked_by_lexer() {
    // Overflow is reported later, when the Parser converts the lexeme.
    assert_single("99999999999999999999", TokenKind::IntegerLiteral);
}

// ----------------------------------------------------------
// Decimal (floating-point) literals
// ----------------------------------------------------------

#[test]
fn test_decimal_literals() {
    for source in ["0.001", "3.14159", "0.90", "10.0", "0.0", "123.456"] {
        assert_single(source, TokenKind::FloatLiteral);
    }
}

#[test]
fn test_decimal_keeps_original_lexeme() {
    assert_eq!(
        lex("0.90"),
        vec![(TokenKind::FloatLiteral, "0.90".to_string())]
    );
}

// ----------------------------------------------------------
// Scientific notation
// ----------------------------------------------------------

#[test]
fn test_scientific_notation() {
    for source in [
        "1.5e-4", "1.5e4", "1.5e+4", "1e5", "1e-5", "2E+3", "1.5E-4", "1e0", "0.1e10", "10e010",
    ] {
        assert_single(source, TokenKind::FloatLiteral);
    }
}

#[test]
fn test_integer_with_exponent_is_float() {
    assert_single("1e5", TokenKind::FloatLiteral);
}

// ----------------------------------------------------------
// Range operator interaction
// ----------------------------------------------------------

#[test]
fn test_integer_range() {
    assert_eq!(
        lex("0..100"),
        vec![
            (TokenKind::IntegerLiteral, "0".to_string()),
            (TokenKind::Range, "..".to_string()),
            (TokenKind::IntegerLiteral, "100".to_string()),
        ]
    );
}

#[test]
fn test_float_range() {
    assert_eq!(
        lex("0.5..1.5"),
        vec![
            (TokenKind::FloatLiteral, "0.5".to_string()),
            (TokenKind::Range, "..".to_string()),
            (TokenKind::FloatLiteral, "1.5".to_string()),
        ]
    );
}

#[test]
fn test_mixed_range() {
    assert_eq!(
        kinds("1.2..3"),
        vec![
            TokenKind::FloatLiteral,
            TokenKind::Range,
            TokenKind::IntegerLiteral
        ]
    );
    assert_eq!(
        kinds("1..2.5"),
        vec![
            TokenKind::IntegerLiteral,
            TokenKind::Range,
            TokenKind::FloatLiteral
        ]
    );
}

#[test]
fn test_range_with_identifier_bounds() {
    assert_eq!(
        kinds("0..count"),
        vec![
            TokenKind::IntegerLiteral,
            TokenKind::Range,
            TokenKind::Identifier
        ]
    );
    assert_eq!(
        kinds("start..10"),
        vec![
            TokenKind::Identifier,
            TokenKind::Range,
            TokenKind::IntegerLiteral
        ]
    );
}

#[test]
fn test_range_with_arithmetic_bound() {
    assert_eq!(
        kinds("0..count-1"),
        vec![
            TokenKind::IntegerLiteral,
            TokenKind::Range,
            TokenKind::Identifier,
            TokenKind::Minus,
            TokenKind::IntegerLiteral,
        ]
    );
}

#[test]
fn test_for_each_header_tokens() {
    assert_eq!(
        kinds("repeat for each epoch in 0..100:\n"),
        vec![
            TokenKind::Repeat,
            TokenKind::For,
            TokenKind::Each,
            TokenKind::Identifier,
            TokenKind::In,
            TokenKind::IntegerLiteral,
            TokenKind::Range,
            TokenKind::IntegerLiteral,
            TokenKind::Colon,
            TokenKind::Newline,
        ]
    );
}

// ----------------------------------------------------------
// Sign handling: `-` is always a separate Minus token
// ----------------------------------------------------------

#[test]
fn test_negative_integer_is_minus_then_literal() {
    assert_eq!(
        lex("-10"),
        vec![
            (TokenKind::Minus, "-".to_string()),
            (TokenKind::IntegerLiteral, "10".to_string()),
        ]
    );
}

#[test]
fn test_negative_float_and_scientific() {
    assert_eq!(
        kinds("-0.5"),
        vec![TokenKind::Minus, TokenKind::FloatLiteral]
    );
    assert_eq!(
        kinds("-1e-3"),
        vec![TokenKind::Minus, TokenKind::FloatLiteral]
    );
}

#[test]
fn test_negative_exponent_sign_stays_inside_literal() {
    // The `-` after `e` belongs to the exponent, not to a subtraction.
    assert_eq!(
        lex("1.5e-4"),
        vec![(TokenKind::FloatLiteral, "1.5e-4".to_string())]
    );
}

#[test]
fn test_positive_sign_is_plus_token() {
    assert_eq!(
        kinds("+5"),
        vec![TokenKind::Plus, TokenKind::IntegerLiteral]
    );
}

// ----------------------------------------------------------
// Numbers inside expressions and statements
// ----------------------------------------------------------

#[test]
fn test_arithmetic_without_whitespace() {
    assert_eq!(
        kinds("10+20*3"),
        vec![
            TokenKind::IntegerLiteral,
            TokenKind::Plus,
            TokenKind::IntegerLiteral,
            TokenKind::Star,
            TokenKind::IntegerLiteral,
        ]
    );
}

#[test]
fn test_subtraction_of_numbers() {
    assert_eq!(
        kinds("10-5"),
        vec![
            TokenKind::IntegerLiteral,
            TokenKind::Minus,
            TokenKind::IntegerLiteral
        ]
    );
}

#[test]
fn test_grouping_expression() {
    assert_eq!(
        kinds("(10 + 20) * 2"),
        vec![
            TokenKind::LeftParen,
            TokenKind::IntegerLiteral,
            TokenKind::Plus,
            TokenKind::IntegerLiteral,
            TokenKind::RightParen,
            TokenKind::Star,
            TokenKind::IntegerLiteral,
        ]
    );
}

#[test]
fn test_array_of_numbers() {
    assert_eq!(
        kinds("[10, 20, 30]"),
        vec![
            TokenKind::LeftBracket,
            TokenKind::IntegerLiteral,
            TokenKind::Comma,
            TokenKind::IntegerLiteral,
            TokenKind::Comma,
            TokenKind::IntegerLiteral,
            TokenKind::RightBracket,
        ]
    );
}

#[test]
fn test_variable_declaration_with_float() {
    assert_eq!(
        kinds("let learning_rate be 0.001\n"),
        vec![
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Be,
            TokenKind::FloatLiteral,
            TokenKind::Newline,
        ]
    );
}

#[test]
fn test_comparison_with_number() {
    assert_eq!(
        kinds("accuracy is at least 0.90"),
        vec![
            TokenKind::Identifier,
            TokenKind::Is,
            TokenKind::At,
            TokenKind::Least,
            TokenKind::FloatLiteral,
        ]
    );
}

#[test]
fn test_tensor_type_dimensions() {
    assert_eq!(
        kinds("Tensor[f32, 224, 224, 3]"),
        vec![
            TokenKind::Identifier,
            TokenKind::LeftBracket,
            TokenKind::Identifier,
            TokenKind::Comma,
            TokenKind::IntegerLiteral,
            TokenKind::Comma,
            TokenKind::IntegerLiteral,
            TokenKind::Comma,
            TokenKind::IntegerLiteral,
            TokenKind::RightBracket,
        ]
    );
}

#[test]
fn test_number_before_comment() {
    assert_eq!(kinds("42 # the answer"), vec![TokenKind::IntegerLiteral]);
}

#[test]
fn test_digits_inside_identifiers_are_not_numbers() {
    for source in ["layer2", "x1", "_1", "conv2d", "a1b2"] {
        assert_single(source, TokenKind::Identifier);
    }
}

#[test]
fn test_float_followed_by_dot_and_identifier() {
    // Not an error at lexical level: `1.5` then `.` then `abs`.
    // Member access is rejected by the Parser in v0.1.0.
    assert_eq!(
        kinds("1.5.abs"),
        vec![
            TokenKind::FloatLiteral,
            TokenKind::Dot,
            TokenKind::Identifier
        ]
    );
}

// ----------------------------------------------------------
// Spans
// ----------------------------------------------------------

#[test]
fn test_integer_span() {
    let token = Lexer::new("100").next_token().unwrap();

    assert_eq!(token.span.start.offset, 0);
    assert_eq!(token.span.start.column, 1);
    assert_eq!(token.span.end.offset, 3);
    assert_eq!(token.span.end.column, 4);
}

#[test]
fn test_float_span_after_whitespace() {
    let mut lexer = Lexer::new("ab  3.14");

    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Identifier);

    let number = lexer.next_token().unwrap();

    assert_eq!(number.kind, TokenKind::FloatLiteral);
    assert_eq!(number.lexeme, "3.14");
    assert_eq!(number.span.start.offset, 4);
    assert_eq!(number.span.start.column, 5);
    assert_eq!(number.span.end.offset, 8);
    assert_eq!(number.span.end.column, 9);
}

#[test]
fn test_number_position_on_second_line() {
    let mut lexer = Lexer::new("a\n  1.5e-4");

    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Newline);

    let number = lexer.next_token().unwrap();

    assert_eq!(number.kind, TokenKind::FloatLiteral);
    assert_eq!(number.span.start.line, 2);
    assert_eq!(number.span.start.column, 3);
    assert_eq!(number.span.len(), 6);
}

#[test]
fn test_range_token_spans_after_number() {
    let mut lexer = Lexer::new("0..100");

    let first = lexer.next_token().unwrap();
    let range = lexer.next_token().unwrap();
    let last = lexer.next_token().unwrap();

    assert_eq!((first.span.start.offset, first.span.end.offset), (0, 1));
    assert_eq!((range.span.start.offset, range.span.end.offset), (1, 3));
    assert_eq!((last.span.start.offset, last.span.end.offset), (3, 6));
}

// ----------------------------------------------------------
// Invalid numbers
// ----------------------------------------------------------

#[test]
fn test_multiple_decimal_points_are_invalid() {
    assert_invalid_number("1.2.3");
    assert_invalid_number("1.2.3.4");
}

#[test]
fn test_trailing_decimal_point_is_invalid() {
    assert_invalid_number("1.");
    assert_invalid_number("1.)");
    assert_invalid_number("1. + 2");
    assert_invalid_number("100.");
}

#[test]
fn test_decimal_point_before_exponent_is_invalid() {
    assert_invalid_number("1.e5");
}

#[test]
fn test_incomplete_exponent_is_invalid() {
    assert_invalid_number("1e");
    assert_invalid_number("1e+");
    assert_invalid_number("1e-");
    assert_invalid_number("1.5e");
    assert_invalid_number("1.5E+");
    assert_invalid_number("1e + 2");
}

#[test]
fn test_decimal_point_inside_exponent_is_invalid() {
    assert_invalid_number("1e5.5");
    assert_invalid_number("1.5e2.5");
}

#[test]
fn test_number_followed_by_letters_is_invalid() {
    assert_invalid_number("123epoch");
    assert_invalid_number("10x");
    assert_invalid_number("3.14abc");
    assert_invalid_number("1e5x");
}

#[test]
fn test_unsupported_number_formats_are_invalid() {
    assert_invalid_number("0x10");
    assert_invalid_number("0b101");
    assert_invalid_number("1_000");
    assert_invalid_number("1__0");
}

#[test]
fn test_invalid_number_span_covers_whole_literal() {
    for (source, length) in [
        ("1.2.3", 5),
        ("1.", 2),
        ("1e", 2),
        ("1e+", 3),
        ("123epoch", 8),
        ("0x10", 4),
        ("1_000", 5),
    ] {
        let error = first_error(source);

        assert_eq!(error.span.start.offset, 0, "source: {source:?}");
        assert_eq!(error.span.len(), length, "source: {source:?}");
    }
}

#[test]
fn test_invalid_number_error_has_line_and_column() {
    let error = first_error("let x be 1.2.3");

    assert_eq!(error.kind, LexerErrorKind::InvalidNumber);
    assert_eq!(error.span.start.line, 1);
    assert_eq!(error.span.start.column, 10);
    assert_eq!(
        error.to_string(),
        "invalid numeric literal at line 1, column 10"
    );
}

#[test]
fn test_invalid_number_error_on_later_line() {
    let error = first_error("let a be 1\nlet b be 1e");

    assert_eq!(error.kind, LexerErrorKind::InvalidNumber);
    assert_eq!(error.span.start.line, 2);
    assert_eq!(error.span.start.column, 10);
}

#[test]
fn test_lexer_recovers_after_invalid_number() {
    // The malformed literal is consumed as a whole, so no cascade of errors follows.
    let mut lexer = Lexer::new("1.2.3 + 4");

    let error = lexer.next_token().unwrap_err();

    assert_eq!(error.kind, LexerErrorKind::InvalidNumber);
    assert_eq!(error.span.len(), 5);

    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Plus);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::IntegerLiteral);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
}

#[test]
fn test_lexer_recovers_after_trailing_decimal_point() {
    let mut lexer = Lexer::new("1. + 2");

    assert_eq!(
        lexer.next_token().unwrap_err().kind,
        LexerErrorKind::InvalidNumber
    );
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Plus);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::IntegerLiteral);
}

// ----------------------------------------------------------
// Characters that are not ASCII digits
// ----------------------------------------------------------

#[test]
fn test_non_ascii_digit_is_unexpected_character() {
    let error = first_error("٣");

    assert_eq!(
        error.kind,
        LexerErrorKind::UnexpectedCharacter { character: '٣' }
    );
}
