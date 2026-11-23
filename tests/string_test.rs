use melilang::lexer::{Lexer, LexerError, LexerErrorKind, TokenKind};

// ==========================================================
// STRING LITERAL TEST
// ==========================================================

const VALID_ESCAPES: [char; 7] = ['n', 't', 'r', '\\', '"', '\'', '0'];

// Collects (kind, lexeme) pairs until Eof, excluding the Eof token itself.
fn lex(source: &str) -> Vec<(TokenKind, String)> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().expect("unexpected lexer error");

        if token.kind == TokenKind::Eof {
            break;
        }

        // The Newline that closes a last line without a line break has an empty
        // lexeme. These tests are not about it; indentation_test.rs covers it.
        if token.kind == TokenKind::Newline && token.lexeme.is_empty() {
            continue;
        }

        tokens.push((token.kind, token.lexeme));
    }

    tokens
}

fn kinds(source: &str) -> Vec<TokenKind> {
    lex(source).into_iter().map(|(kind, _)| kind).collect()
}

// Runs the Lexer to the end and records every token kind or error kind.
fn results(source: &str) -> Vec<Result<TokenKind, LexerErrorKind>> {
    let mut lexer = Lexer::new(source);
    let mut output = Vec::new();

    for _ in 0..1000 {
        match lexer.next_token() {
            Ok(token) if token.kind == TokenKind::Eof => return output,
            Ok(token) if token.kind == TokenKind::Newline && token.lexeme.is_empty() => {} // Skip the synthetic end-of-file Newline (empty lexeme), as in `lex`.
            Ok(token) => output.push(Ok(token.kind)),
            Err(error) => output.push(Err(error.kind)),
        }
    }

    panic!("lexer did not reach Eof for {source:?}");
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

fn assert_string(source: &str) {
    assert_eq!(
        lex(source),
        vec![(TokenKind::StringLiteral, source.to_string())],
        "source: {source:?}"
    );
}

fn invalid_escape(sequence: &str) -> LexerErrorKind {
    LexerErrorKind::InvalidEscapeSequence {
        sequence: sequence.to_string(),
    }
}

// ----------------------------------------------------------
// Valid string literals
// ----------------------------------------------------------

#[test]
fn test_simple_string() {
    assert_string("\"Hello\"");
}

#[test]
fn test_empty_string() {
    assert_string("\"\"");
}

#[test]
fn test_string_with_spaces_and_punctuation() {
    assert_string("\"Hello, MeliLang!\"");
    assert_string("\"  leading and trailing  \"");
}

#[test]
fn test_string_lexeme_keeps_quotes() {
    let token = Lexer::new("\"abc\"").next_token().unwrap();

    assert_eq!(token.kind, TokenKind::StringLiteral);
    assert_eq!(token.lexeme, "\"abc\"");
}

#[test]
fn test_string_lexeme_is_not_decoded() {
    // The Lexer validates escapes but leaves them as written;
    // the Parser decodes them when building the AST literal.
    let source = r#""a\nb""#;
    let token = Lexer::new(source).next_token().unwrap();

    assert_eq!(token.lexeme, source);
    assert_eq!(token.lexeme.len(), 6);
}

#[test]
fn test_string_may_contain_a_real_tab() {
    assert_string("\"a\tb\"");
}

#[test]
fn test_string_may_contain_single_quote() {
    assert_string("\"it's\"");
    assert_string("\"'\"");
}

// ----------------------------------------------------------
// Escape sequences
// ----------------------------------------------------------

#[test]
fn test_each_valid_escape_sequence() {
    for escape in VALID_ESCAPES {
        assert_string(&format!("\"\\{escape}\""));
    }
}

#[test]
fn test_all_valid_escapes_in_one_string() {
    assert_string(r#""\n\t\r\\\"\'\0""#);
}

#[test]
fn test_escaped_quote_does_not_end_string() {
    assert_string("\"say \\\"hi\\\"\"");
}

#[test]
fn test_escaped_backslash_before_closing_quote() {
    // `\\` is one escaped backslash, so the next `"` closes the string.
    assert_string("\"a\\\\\"");
    assert_eq!(kinds("\"a\\\\\" + b").len(), 3);
}

// ----------------------------------------------------------
// Content that must not be interpreted
// ----------------------------------------------------------

#[test]
fn test_unicode_content() {
    assert_string("\"é🧡 日本語\"");
}

#[test]
fn test_hash_inside_string_is_not_a_comment() {
    assert_string("\"a # b\"");
    assert_eq!(
        kinds("\"a # b\" # real comment"),
        vec![TokenKind::StringLiteral]
    );
}

#[test]
fn test_operators_inside_string() {
    assert_string("\"+ - -> .. @ ( ) [ ]\"");
}

#[test]
fn test_keywords_inside_string() {
    assert_string("\"let var when is at least\"");
}

#[test]
fn test_numbers_inside_string() {
    assert_string("\"1.2.3 and 1e\"");
}

// ----------------------------------------------------------
// Strings next to other tokens
// ----------------------------------------------------------

#[test]
fn test_adjacent_strings_are_two_tokens() {
    assert_eq!(
        kinds("\"a\"\"b\""),
        vec![TokenKind::StringLiteral, TokenKind::StringLiteral]
    );
}

#[test]
fn test_string_followed_by_identifier_without_space() {
    assert_eq!(
        kinds("\"a\"b"),
        vec![TokenKind::StringLiteral, TokenKind::Identifier]
    );
}

#[test]
fn test_show_statement_tokens() {
    assert_eq!(
        kinds("show \"Training started\"\n"),
        vec![
            TokenKind::Show,
            TokenKind::StringLiteral,
            TokenKind::Newline
        ]
    );
}

#[test]
fn test_variable_declaration_with_string() {
    assert_eq!(
        kinds("let message be \"Hello, MeliLang!\"\n"),
        vec![
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Be,
            TokenKind::StringLiteral,
            TokenKind::Newline,
        ]
    );
}

#[test]
fn test_string_as_function_argument() {
    assert_eq!(
        kinds("log(\"hi\", 3)"),
        vec![
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::StringLiteral,
            TokenKind::Comma,
            TokenKind::IntegerLiteral,
            TokenKind::RightParen,
        ]
    );
}

// ----------------------------------------------------------
// Spans
// ----------------------------------------------------------

#[test]
fn test_string_span() {
    let token = Lexer::new("\"abc\"").next_token().unwrap();

    assert_eq!(token.span.start.offset, 0);
    assert_eq!(token.span.start.column, 1);
    assert_eq!(token.span.end.offset, 5);
    assert_eq!(token.span.end.column, 6);
}

#[test]
fn test_string_span_counts_bytes_for_offset_and_chars_for_column() {
    let token = Lexer::new("\"é🧡\"").next_token().unwrap();

    // 1 + 2 + 4 + 1 bytes, but only 4 characters.
    assert_eq!(token.span.len(), 8);
    assert_eq!(token.span.end.offset, 8);
    assert_eq!(token.span.end.column, 5);
}

#[test]
fn test_token_position_after_multibyte_string() {
    let mut lexer = Lexer::new("\"é\" +");

    lexer.next_token().unwrap();
    let plus = lexer.next_token().unwrap();

    assert_eq!(plus.kind, TokenKind::Plus);
    assert_eq!(plus.span.start.offset, 5);
    assert_eq!(plus.span.start.column, 5);
}

#[test]
fn test_string_position_on_second_line() {
    let mut lexer = Lexer::new("a\n  \"hi\"");

    lexer.next_token().unwrap(); // Identifier
    lexer.next_token().unwrap(); // Newline
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Indent);

    let string = lexer.next_token().unwrap();

    assert_eq!(string.kind, TokenKind::StringLiteral);
    assert_eq!(string.span.start.line, 2);
    assert_eq!(string.span.start.column, 3);
}

// ----------------------------------------------------------
// Unterminated strings
// ----------------------------------------------------------

#[test]
fn test_unterminated_string_at_end_of_source() {
    let error = first_error("\"Hello MeliLang");

    assert_eq!(error.kind, LexerErrorKind::UnterminatedString);
    assert_eq!(error.span.start.offset, 0);
    assert_eq!(error.span.len(), 15);
}

#[test]
fn test_lone_quote_is_unterminated() {
    let error = first_error("\"");

    assert_eq!(error.kind, LexerErrorKind::UnterminatedString);
    assert_eq!(error.span.len(), 1);
}

#[test]
fn test_unterminated_string_span_starts_at_opening_quote() {
    let error = first_error("let x be \"abc");

    assert_eq!(error.kind, LexerErrorKind::UnterminatedString);
    assert_eq!(error.span.start.offset, 9);
    assert_eq!(error.span.start.line, 1);
    assert_eq!(error.span.start.column, 10);
    assert_eq!(error.span.len(), 4);
}

#[test]
fn test_unterminated_string_ends_at_newline() {
    assert_eq!(
        results("\"abc\nlet"),
        vec![
            Err(LexerErrorKind::UnterminatedString),
            Ok(TokenKind::Newline),
            Ok(TokenKind::Let),
        ]
    );
}

#[test]
fn test_unterminated_string_before_crlf() {
    let error = first_error("\"abc\r\nx");

    assert_eq!(error.kind, LexerErrorKind::UnterminatedString);
    assert_eq!(error.span.len(), 4);

    assert_eq!(
        results("\"abc\r\nx"),
        vec![
            Err(LexerErrorKind::UnterminatedString),
            Ok(TokenKind::Newline),
            Ok(TokenKind::Identifier),
        ]
    );
}

#[test]
fn test_string_cannot_span_lines() {
    assert_eq!(
        results("\"a\nb\""),
        vec![
            Err(LexerErrorKind::UnterminatedString),
            Ok(TokenKind::Newline),
            Ok(TokenKind::Identifier),
            Err(LexerErrorKind::UnterminatedString),
        ]
    );
}

#[test]
fn test_trailing_backslash_at_end_of_source_is_unterminated() {
    let error = first_error("\"abc\\");

    assert_eq!(error.kind, LexerErrorKind::UnterminatedString);
    assert_eq!(error.span.len(), 5);
}

#[test]
fn test_backslash_before_newline_is_unterminated() {
    // A backslash cannot continue a string onto the next line.
    assert_eq!(
        results("\"abc\\\nx"),
        vec![
            Err(LexerErrorKind::UnterminatedString),
            Ok(TokenKind::Newline),
            Ok(TokenKind::Identifier),
        ]
    );
}

#[test]
fn test_valid_string_then_unterminated_string() {
    assert_eq!(
        results("\"ok\" \"bad"),
        vec![
            Ok(TokenKind::StringLiteral),
            Err(LexerErrorKind::UnterminatedString),
        ]
    );
}

#[test]
fn test_unterminated_string_error_message() {
    assert_eq!(
        first_error("\"abc").to_string(),
        "unterminated string literal at line 1, column 1"
    );
}

#[test]
fn test_unterminated_string_error_on_later_line() {
    let error = first_error("let a be 1\nlet b be \"oops");

    assert_eq!(error.kind, LexerErrorKind::UnterminatedString);
    assert_eq!(error.span.start.line, 2);
    assert_eq!(error.span.start.column, 10);
}

// ----------------------------------------------------------
// Invalid escape sequences
// ----------------------------------------------------------

#[test]
fn test_invalid_escape_sequences() {
    for (source, sequence) in [
        ("\"\\x\"", "\\x"),
        ("\"\\u\"", "\\u"),
        ("\"\\a\"", "\\a"),
        ("\"\\1\"", "\\1"),
        ("\"\\ \"", "\\ "),
        ("\"\\é\"", "\\é"),
    ] {
        let error = first_error(source);

        assert_eq!(error.kind, invalid_escape(sequence), "source: {source:?}");
    }
}

#[test]
fn test_invalid_escape_span_covers_only_the_escape() {
    let error = first_error("\"\\x\"");

    assert_eq!(error.span.start.offset, 1);
    assert_eq!(error.span.start.column, 2);
    assert_eq!(error.span.len(), 2);
}

#[test]
fn test_invalid_escape_span_counts_bytes_of_multibyte_character() {
    let error = first_error("\"\\é\"");

    assert_eq!(error.span.len(), 3);
}

#[test]
fn test_invalid_escape_position_and_message() {
    let error = first_error("show \"ab\\q\"");

    assert_eq!(error.span.start.line, 1);
    assert_eq!(error.span.start.column, 9);
    assert_eq!(
        error.to_string(),
        "invalid escape sequence '\\q' at line 1, column 9"
    );
}

#[test]
fn test_invalid_escape_consumes_rest_of_literal() {
    // No second error for the remaining `41"`; the Lexer resumes after the string.
    assert_eq!(
        results("\"\\x41\" + 1"),
        vec![
            Err(invalid_escape("\\x")),
            Ok(TokenKind::Plus),
            Ok(TokenKind::IntegerLiteral),
        ]
    );
}

#[test]
fn test_only_the_first_invalid_escape_is_reported() {
    assert_eq!(
        results("\"\\x\\y\" + 1"),
        vec![
            Err(invalid_escape("\\x")),
            Ok(TokenKind::Plus),
            Ok(TokenKind::IntegerLiteral),
        ]
    );
}

#[test]
fn test_invalid_escape_is_reported_before_missing_quote() {
    // The earliest problem in the source wins, and the lexer still ends cleanly.
    assert_eq!(results("\"\\x abc"), vec![Err(invalid_escape("\\x"))]);
}

#[test]
fn test_valid_string_then_invalid_escape() {
    assert_eq!(
        results("\"ok\" \"\\q\""),
        vec![Ok(TokenKind::StringLiteral), Err(invalid_escape("\\q"))]
    );
}

// ----------------------------------------------------------
// Characters that are not MeliLang quotes
// ----------------------------------------------------------

#[test]
fn test_curly_quotes_are_not_string_delimiters() {
    assert_eq!(
        results("“hi”"),
        vec![
            Err(LexerErrorKind::UnexpectedCharacter { character: '“' }),
            Ok(TokenKind::Identifier),
            Err(LexerErrorKind::UnexpectedCharacter { character: '”' }),
        ]
    );
}

#[test]
fn test_eof_after_string_is_stable() {
    let mut lexer = Lexer::new("\"a\"");

    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::StringLiteral);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Newline); // A last line without a line break is closed by a synthetic Newline.
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
}