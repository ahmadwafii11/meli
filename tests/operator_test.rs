use melilang::lexer::{Lexer, LexerErrorKind, TokenKind};

// ==========================================================
// OPERATOR AND DELIMITER TEST
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

// ----------------------------------------------------------
// Single-character symbols
// ----------------------------------------------------------

#[test]
fn test_arithmetic_operators() {
    let cases = [
        ("+", TokenKind::Plus),
        ("-", TokenKind::Minus),
        ("*", TokenKind::Star),
        ("/", TokenKind::Slash),
        ("%", TokenKind::Percent),
        ("@", TokenKind::MatrixMultiply),
    ];

    for (source, expected) in cases {
        assert_eq!(lex(source), vec![(expected, source.to_string())]);
    }
}

#[test]
fn test_assign_operator() {
    assert_eq!(lex("="), vec![(TokenKind::Assign, "=".to_string())]);
}

#[test]
fn test_delimiters() {
    let cases = [
        ("(", TokenKind::LeftParen),
        (")", TokenKind::RightParen),
        ("[", TokenKind::LeftBracket),
        ("]", TokenKind::RightBracket),
        ("{", TokenKind::LeftBrace),
        ("}", TokenKind::RightBrace),
        (":", TokenKind::Colon),
        (",", TokenKind::Comma),
        (".", TokenKind::Dot),
    ];

    for (source, expected) in cases {
        assert_eq!(lex(source), vec![(expected, source.to_string())]);
    }
}

// ----------------------------------------------------------
// Multi-character symbols are recognized before shorter ones
// ----------------------------------------------------------

#[test]
fn test_arrow_is_one_token() {
    assert_eq!(lex("->"), vec![(TokenKind::Arrow, "->".to_string())]);
}

#[test]
fn test_minus_is_not_arrow_without_greater_than() {
    assert_eq!(kinds("- x"), vec![TokenKind::Minus, TokenKind::Identifier]);
    assert_eq!(kinds("-("), vec![TokenKind::Minus, TokenKind::LeftParen]);
}

#[test]
fn test_arrow_without_whitespace() {
    assert_eq!(
        kinds("a->b"),
        vec![
            TokenKind::Identifier,
            TokenKind::Arrow,
            TokenKind::Identifier
        ]
    );
}

#[test]
fn test_minus_followed_by_space_then_greater_than_is_error() {
    // `>` alone is not a MeliLang symbol; comparisons use words.
    let mut lexer = Lexer::new("- >");

    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Minus);

    let error = lexer.next_token().unwrap_err();

    assert_eq!(
        error.kind,
        LexerErrorKind::UnexpectedCharacter { character: '>' }
    );
}

#[test]
fn test_range_is_one_token() {
    assert_eq!(lex(".."), vec![(TokenKind::Range, "..".to_string())]);
}

#[test]
fn test_range_between_identifiers() {
    assert_eq!(
        kinds("start..end"),
        vec![
            TokenKind::Identifier,
            TokenKind::Range,
            TokenKind::Identifier
        ]
    );
}

#[test]
fn test_dot_in_module_path() {
    assert_eq!(
        kinds("ml.training"),
        vec![TokenKind::Identifier, TokenKind::Dot, TokenKind::Identifier]
    );
}

#[test]
fn test_three_dots_are_range_then_dot() {
    // Longest match first: `..` is consumed, then the remaining `.`.
    assert_eq!(kinds("..."), vec![TokenKind::Range, TokenKind::Dot]);
}

#[test]
fn test_separated_dots_are_two_dot_tokens() {
    assert_eq!(kinds(". ."), vec![TokenKind::Dot, TokenKind::Dot]);
}

#[test]
fn test_four_dots_are_two_ranges() {
    assert_eq!(kinds("...."), vec![TokenKind::Range, TokenKind::Range]);
}

// ----------------------------------------------------------
// `at` keyword versus `@` symbol (lexical rule 12)
// ----------------------------------------------------------

#[test]
fn test_at_keyword_and_matrix_multiply_are_different_tokens() {
    assert_eq!(kinds("at"), vec![TokenKind::At]);
    assert_eq!(kinds("@"), vec![TokenKind::MatrixMultiply]);
    assert_eq!(
        kinds("a @ b"),
        vec![
            TokenKind::Identifier,
            TokenKind::MatrixMultiply,
            TokenKind::Identifier
        ]
    );
    assert_eq!(
        kinds("is at least"),
        vec![TokenKind::Is, TokenKind::At, TokenKind::Least]
    );
}

#[test]
fn test_at_inside_identifier_is_not_keyword() {
    assert_eq!(kinds("attention"), vec![TokenKind::Identifier]);
    assert_eq!(kinds("at_least"), vec![TokenKind::Identifier]);
}

// ----------------------------------------------------------
// Symbols adjacent to other tokens
// ----------------------------------------------------------

#[test]
fn test_operators_without_whitespace() {
    assert_eq!(
        kinds("a+b*c-d/e%f"),
        vec![
            TokenKind::Identifier,
            TokenKind::Plus,
            TokenKind::Identifier,
            TokenKind::Star,
            TokenKind::Identifier,
            TokenKind::Minus,
            TokenKind::Identifier,
            TokenKind::Slash,
            TokenKind::Identifier,
            TokenKind::Percent,
            TokenKind::Identifier,
        ]
    );
}

#[test]
fn test_function_call_tokens() {
    assert_eq!(
        kinds("calculate_accuracy(correct, total)"),
        vec![
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::Identifier,
            TokenKind::Comma,
            TokenKind::Identifier,
            TokenKind::RightParen,
        ]
    );
}

#[test]
fn test_type_annotation_tokens() {
    assert_eq!(
        kinds("let learning_rate: f32 be"),
        vec![
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Colon,
            TokenKind::Identifier,
            TokenKind::Be,
        ]
    );
}

#[test]
fn test_generic_type_brackets() {
    assert_eq!(
        kinds("List[i32]"),
        vec![
            TokenKind::Identifier,
            TokenKind::LeftBracket,
            TokenKind::Identifier,
            TokenKind::RightBracket,
        ]
    );
}

#[test]
fn test_block_header_ends_with_colon_then_newline() {
    assert_eq!(
        kinds("when ready:\n"),
        vec![
            TokenKind::When,
            TokenKind::Identifier,
            TokenKind::Colon,
            TokenKind::Newline,
        ]
    );
}

#[test]
fn test_symbols_inside_comment_are_ignored() {
    assert_eq!(kinds("a # + - -> .. @ ( )"), vec![TokenKind::Identifier]);
}

// ----------------------------------------------------------
// Spans and lexemes
// ----------------------------------------------------------

#[test]
fn test_arrow_span() {
    let token = Lexer::new("->").next_token().unwrap();

    assert_eq!(token.span.start.offset, 0);
    assert_eq!(token.span.start.column, 1);
    assert_eq!(token.span.end.offset, 2);
    assert_eq!(token.span.end.column, 3);
    assert_eq!(token.span.len(), 2);
}

#[test]
fn test_symbol_span_after_whitespace() {
    let mut lexer = Lexer::new("ab  ..");

    let ident = lexer.next_token().unwrap();
    assert_eq!(ident.kind, TokenKind::Identifier);

    let range = lexer.next_token().unwrap();

    assert_eq!(range.kind, TokenKind::Range);
    assert_eq!(range.lexeme, "..");
    assert_eq!(range.span.start.offset, 4);
    assert_eq!(range.span.start.column, 5);
    assert_eq!(range.span.end.offset, 6);
    assert_eq!(range.span.end.column, 7);
}

#[test]
fn test_symbol_position_on_second_line() {
    let mut lexer = Lexer::new("a\n  +");

    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Identifier);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Newline);

    let plus = lexer.next_token().unwrap();

    assert_eq!(plus.kind, TokenKind::Plus);
    assert_eq!(plus.span.start.line, 2);
    assert_eq!(plus.span.start.column, 3);
}

// ----------------------------------------------------------
// Characters that are still invalid
// ----------------------------------------------------------

#[test]
fn test_comparison_symbols_are_not_supported() {
    for character in ['<', '>', '!', '&', '|', '^', '~', '?', ';'] {
        let source = character.to_string();
        let error = Lexer::new(&source).next_token().unwrap_err();

        assert_eq!(
            error.kind,
            LexerErrorKind::UnexpectedCharacter { character }
        );
    }
}

#[test]
fn test_double_equals_is_two_assign_tokens() {
    // MeliLang has no `==`; comparison uses `is equal to`.
    // The Parser, not the Lexer, rejects this sequence.
    assert_eq!(kinds("=="), vec![TokenKind::Assign, TokenKind::Assign]);
}

#[test]
fn test_eof_after_symbols_is_stable() {
    let mut lexer = Lexer::new("+");

    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Plus);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
}
