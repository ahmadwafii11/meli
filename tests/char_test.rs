use melilang::lexer::{Lexer, LexerError, LexerErrorKind, TokenKind};

// ==========================================================
// CHARACTER LITERAL TEST
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

fn assert_char(source: &str) {
    assert_eq!(
        lex(source),
        vec![(TokenKind::CharLiteral, source.to_string())],
        "source: {source:?}"
    );
}

fn invalid_escape(sequence: &str) -> LexerErrorKind {
    LexerErrorKind::InvalidEscapeSequence {
        sequence: sequence.to_string(),
    }
}

// ----------------------------------------------------------
// Valid character literals
// ----------------------------------------------------------

#[test]
fn test_simple_characters() {
    for source in ["'M'", "'a'", "'Z'", "'0'", "'+'", "'#'", "'_'"] {
        assert_char(source);
    }
}

#[test]
fn test_space_and_tab_characters() {
    assert_char("' '");
    assert_char("'\t'");
}

#[test]
fn test_double_quote_needs_no_escape_in_character() {
    assert_char("'\"'");
}

#[test]
fn test_unicode_characters() {
    // Each of these is exactly one Unicode scalar value.
    for source in ["'é'", "'🧡'", "'日'"] {
        assert_char(source);
    }
}

#[test]
fn test_character_lexeme_keeps_quotes() {
    let token = Lexer::new("'M'").next_token().unwrap();

    assert_eq!(token.kind, TokenKind::CharLiteral);
    assert_eq!(token.lexeme, "'M'");
}

// ----------------------------------------------------------
// Escape sequences
// ----------------------------------------------------------

#[test]
fn test_each_valid_escape_sequence() {
    for escape in VALID_ESCAPES {
        assert_char(&format!("'\\{escape}'"));
    }
}

#[test]
fn test_escape_lexeme_is_not_decoded() {
    let token = Lexer::new(r"'\n'").next_token().unwrap();

    assert_eq!(token.lexeme, r"'\n'");
    assert_eq!(token.lexeme.len(), 4);
}

#[test]
fn test_escaped_single_quote() {
    assert_char(r"'\''");
}

// ----------------------------------------------------------
// Characters next to other tokens
// ----------------------------------------------------------

#[test]
fn test_adjacent_characters_are_two_tokens() {
    assert_eq!(
        kinds("'a''b'"),
        vec![TokenKind::CharLiteral, TokenKind::CharLiteral]
    );
}

#[test]
fn test_character_and_string_side_by_side() {
    assert_eq!(
        kinds("'a' \"b\""),
        vec![TokenKind::CharLiteral, TokenKind::StringLiteral]
    );
}

#[test]
fn test_variable_declaration_with_character() {
    assert_eq!(
        kinds("let initial be 'M'\n"),
        vec![
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Be,
            TokenKind::CharLiteral,
            TokenKind::Newline,
        ]
    );
}

#[test]
fn test_apostrophe_in_comment_is_ignored() {
    assert_eq!(kinds("x # it's fine"), vec![TokenKind::Identifier]);
    assert_eq!(kinds("# don't\n"), vec![TokenKind::Newline]);
}

#[test]
fn test_hash_character_is_not_a_comment() {
    assert_eq!(kinds("'#' # real comment"), vec![TokenKind::CharLiteral]);
}

// ----------------------------------------------------------
// Spans
// ----------------------------------------------------------

#[test]
fn test_character_span() {
    let token = Lexer::new("'M'").next_token().unwrap();

    assert_eq!(token.span.start.offset, 0);
    assert_eq!(token.span.start.column, 1);
    assert_eq!(token.span.end.offset, 3);
    assert_eq!(token.span.end.column, 4);
}

#[test]
fn test_multibyte_character_span() {
    let token = Lexer::new("'é'").next_token().unwrap();

    assert_eq!(token.span.len(), 4);
    assert_eq!(token.span.end.column, 4);

    let emoji = Lexer::new("'🧡'").next_token().unwrap();

    assert_eq!(emoji.span.len(), 6);
    assert_eq!(emoji.span.end.column, 4);
}

#[test]
fn test_character_position_on_second_line() {
    let mut lexer = Lexer::new("a\n  'x'");

    lexer.next_token().unwrap();
    lexer.next_token().unwrap();

    let character = lexer.next_token().unwrap();

    assert_eq!(character.kind, TokenKind::CharLiteral);
    assert_eq!(character.span.start.line, 2);
    assert_eq!(character.span.start.column, 3);
}

// ----------------------------------------------------------
// Invalid character literals
// ----------------------------------------------------------

#[test]
fn test_empty_character_literal_is_invalid() {
    let error = first_error("''");

    assert_eq!(error.kind, LexerErrorKind::InvalidCharacterLiteral);
    assert_eq!(error.span.start.offset, 0);
    assert_eq!(error.span.len(), 2);
}

#[test]
fn test_character_literal_with_several_characters_is_invalid() {
    for source in ["'AB'", "'abc'", "'é🧡'", r"'\n\t'", r"'\n\n'", "'a b'"] {
        let error = first_error(source);

        assert_eq!(
            error.kind,
            LexerErrorKind::InvalidCharacterLiteral,
            "source: {source:?}"
        );
        assert_eq!(error.span.start.offset, 0, "source: {source:?}");
        assert_eq!(error.span.len(), source.len(), "source: {source:?}");
    }
}

#[test]
fn test_invalid_character_literal_error_message() {
    assert_eq!(
        first_error("let c be 'AB'").to_string(),
        "invalid character literal at line 1, column 10"
    );
}

#[test]
fn test_lexer_recovers_after_invalid_character_literal() {
    assert_eq!(
        results("'AB' + 1"),
        vec![
            Err(LexerErrorKind::InvalidCharacterLiteral),
            Ok(TokenKind::Plus),
            Ok(TokenKind::IntegerLiteral),
        ]
    );
}

#[test]
fn test_three_quotes_are_empty_literal_then_unterminated() {
    // A single quote must be escaped: write '\'' instead of '''.
    assert_eq!(
        results("'''"),
        vec![
            Err(LexerErrorKind::InvalidCharacterLiteral),
            Err(LexerErrorKind::UnterminatedCharacter),
        ]
    );
}

// ----------------------------------------------------------
// Unterminated character literals
// ----------------------------------------------------------

#[test]
fn test_unterminated_character_literals() {
    for source in ["'", "'M", "'ab", r"'\n"] {
        let error = first_error(source);

        assert_eq!(
            error.kind,
            LexerErrorKind::UnterminatedCharacter,
            "source: {source:?}"
        );
        assert_eq!(error.span.start.offset, 0, "source: {source:?}");
        assert_eq!(error.span.len(), source.len(), "source: {source:?}");
    }
}

#[test]
fn test_escaped_quote_does_not_close_character() {
    // The final quote is consumed as the escaped character, so nothing closes the literal.
    let error = first_error(r"'\'");

    assert_eq!(error.kind, LexerErrorKind::UnterminatedCharacter);
    assert_eq!(error.span.len(), 3);
}

#[test]
fn test_unterminated_character_ends_at_newline() {
    assert_eq!(
        results("'a\nlet"),
        vec![
            Err(LexerErrorKind::UnterminatedCharacter),
            Ok(TokenKind::Newline),
            Ok(TokenKind::Let),
        ]
    );
}

#[test]
fn test_character_cannot_span_lines() {
    assert_eq!(
        results("'\n'"),
        vec![
            Err(LexerErrorKind::UnterminatedCharacter),
            Ok(TokenKind::Newline),
            Err(LexerErrorKind::UnterminatedCharacter),
        ]
    );
}

#[test]
fn test_unterminated_character_error_message() {
    assert_eq!(
        first_error("'a").to_string(),
        "unterminated character literal at line 1, column 1"
    );
}

#[test]
fn test_unterminated_character_error_on_later_line() {
    let error = first_error("let a be 1\nlet b be 'x");

    assert_eq!(error.kind, LexerErrorKind::UnterminatedCharacter);
    assert_eq!(error.span.start.line, 2);
    assert_eq!(error.span.start.column, 10);
}

// ----------------------------------------------------------
// Invalid escape sequences in characters
// ----------------------------------------------------------

#[test]
fn test_invalid_escape_in_character() {
    for (source, sequence) in [
        (r"'\x'", r"\x"),
        (r"'\q'", r"\q"),
        (r"'\u'", r"\u"),
        (r"'\1'", r"\1"),
    ] {
        let error = first_error(source);

        assert_eq!(error.kind, invalid_escape(sequence), "source: {source:?}");
    }
}

#[test]
fn test_invalid_escape_in_character_span() {
    let error = first_error(r"'\x'");

    assert_eq!(error.span.start.offset, 1);
    assert_eq!(error.span.start.column, 2);
    assert_eq!(error.span.len(), 2);
}

#[test]
fn test_lexer_recovers_after_invalid_escape_in_character() {
    assert_eq!(
        results(r"'\x' + 1"),
        vec![
            Err(invalid_escape(r"\x")),
            Ok(TokenKind::Plus),
            Ok(TokenKind::IntegerLiteral),
        ]
    );
}

#[test]
fn test_eof_after_character_is_stable() {
    let mut lexer = Lexer::new("'a'");

    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::CharLiteral);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
}
