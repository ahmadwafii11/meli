use melilang::lexer::TokenKind::*;
use melilang::lexer::{Lexer, LexerError, LexerErrorKind, Token, TokenKind};

// ==========================================================
// INDENTATION TEST
// ==========================================================
//
// Unlike the other lexer tests, these helpers keep every token, including the
// synthetic Newline that closes a last line without a line break.

// Lexes the whole source, excluding Eof. Panics on a lexical error.
fn tokens(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut output = Vec::new();

    for _ in 0..1000 {
        let token = lexer.next_token().expect("unexpected lexer error");

        if token.kind == Eof {
            return output;
        }

        output.push(token);
    }

    panic!("lexer did not reach Eof for {source:?}");
}

fn kinds(source: &str) -> Vec<TokenKind> {
    tokens(source).into_iter().map(|token| token.kind).collect()
}

// Runs the Lexer to the end and records every token kind or error kind.
fn results(source: &str) -> Vec<Result<TokenKind, LexerErrorKind>> {
    let mut lexer = Lexer::new(source);
    let mut output = Vec::new();

    for _ in 0..1000 {
        match lexer.next_token() {
            Ok(token) if token.kind == Eof => return output,
            Ok(token) => output.push(Ok(token.kind)),
            Err(error) => output.push(Err(error.kind)),
        }
    }

    panic!("lexer did not reach Eof for {source:?}");
}

// Returns the first lexical error produced for the source.
fn first_error(source: &str) -> LexerError {
    let mut lexer = Lexer::new(source);

    for _ in 0..1000 {
        match lexer.next_token() {
            Ok(token) if token.kind == Eof => {
                panic!("expected a lexer error for {source:?}")
            }
            Ok(_) => {}
            Err(error) => return error,
        }
    }

    panic!("lexer did not reach Eof for {source:?}");
}

fn count(kinds: &[TokenKind], wanted: TokenKind) -> usize {
    kinds.iter().filter(|&&kind| kind == wanted).count()
}

// ----------------------------------------------------------
// Blocks
// ----------------------------------------------------------

#[test]
fn test_token_spec_block_example() {
    // The example from Token Specification section 3.11, including its blank line.
    let source = "when accuracy is at least 0.90:\n    show \"Excellent\"\n    show \"Continue training\"\n\nshow \"Finished\"\n";

    assert_eq!(
        kinds(source),
        vec![
            When,
            Identifier,
            Is,
            At,
            Least,
            FloatLiteral,
            Colon,
            Newline,
            Indent,
            Show,
            StringLiteral,
            Newline,
            Show,
            StringLiteral,
            Newline,
            Dedent,
            Show,
            StringLiteral,
            Newline,
        ]
    );
}

#[test]
fn test_block_is_closed_by_dedent_at_end_of_source() {
    assert_eq!(
        kinds("when x:\n    show y\n"),
        vec![
            When, Identifier, Colon, Newline, Indent, Show, Identifier, Newline, Dedent
        ]
    );
}

#[test]
fn test_otherwise_branch() {
    assert_eq!(
        kinds("when a:\n    show b\notherwise:\n    show c\n"),
        vec![
            When, Identifier, Colon, Newline, Indent, Show, Identifier, Newline, Dedent, Otherwise,
            Colon, Newline, Indent, Show, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_function_definition_block() {
    assert_eq!(
        kinds("define function f(a: i32) returns f32:\n    return a\n"),
        vec![
            Define, Function, Identifier, LeftParen, Identifier, Colon, Identifier, RightParen,
            Returns, Identifier, Colon, Newline, Indent, Return, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_nested_blocks() {
    assert_eq!(
        kinds("a:\n    b:\n        c\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Colon, Newline, Indent, Identifier,
            Newline, Dedent, Dedent,
        ]
    );
}

#[test]
fn test_several_dedents_at_once() {
    assert_eq!(
        kinds("a:\n    b:\n        c\nd\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Colon, Newline, Indent, Identifier,
            Newline, Dedent, Dedent, Identifier, Newline,
        ]
    );
}

#[test]
fn test_partial_dedent_returns_to_the_middle_level() {
    assert_eq!(
        kinds("a:\n    b:\n        c\n    d\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Colon, Newline, Indent, Identifier,
            Newline, Dedent, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_same_indentation_adds_no_tokens() {
    assert_eq!(
        kinds("a:\n    b\n    c\n    d\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Identifier, Newline,
            Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_indentation_width_is_not_fixed() {
    let expected = vec![
        Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent,
    ];

    for source in [
        "a:\n b\n",
        "a:\n  b\n",
        "a:\n   b\n",
        "a:\n        b\n",
        "a:\n\tb\n",
        "a:\n\t\tb\n",
    ] {
        assert_eq!(kinds(source), expected, "source: {source:?}");
    }
}

#[test]
fn test_each_level_may_use_a_different_width() {
    assert_eq!(
        kinds("a:\n  b:\n      c\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Colon, Newline, Indent, Identifier,
            Newline, Dedent, Dedent,
        ]
    );
}

#[test]
fn test_indent_and_dedent_are_balanced() {
    let program = "let epochs be 3\n\
                   var current_epoch be 0\n\
                   \n\
                   repeat while current_epoch is less than epochs:\n\
                   \x20   when current_epoch is at least 1:\n\
                   \x20       show \"warm\"\n\
                   \x20   set current_epoch to current_epoch + 1\n\
                   \n\
                   show \"done\"\n";

    let result = kinds(program);

    assert_eq!(count(&result, Indent), 2);
    assert_eq!(count(&result, Dedent), 2);
    // One Newline per line with code; blank lines add none.
    assert_eq!(count(&result, Newline), 7);

    for source in [
        "a:\n    b:\n        c\n",
        "a:\n    b:\n        c\nd\n",
        "a:\n    b:\n        c\n    d\n",
        "a:\n    b",
        "a:\n    b:\n        c",
        "  a\n",
    ] {
        let result = kinds(source);

        assert_eq!(
            count(&result, Indent),
            count(&result, Dedent),
            "source: {source:?}"
        );
    }
}

// ----------------------------------------------------------
// Indent and Dedent tokens
// ----------------------------------------------------------

#[test]
fn test_indent_token_lexeme_and_span() {
    let indent = tokens("a:\n    b\n")
        .into_iter()
        .find(|token| token.kind == Indent)
        .unwrap();

    assert_eq!(indent.lexeme, "    ");
    assert_eq!(indent.span.start.offset, 3);
    assert_eq!(indent.span.start.line, 2);
    assert_eq!(indent.span.start.column, 1);
    assert_eq!(indent.span.end.offset, 7);
    assert_eq!(indent.span.end.column, 5);
}

#[test]
fn test_indent_lexeme_for_tabs() {
    let indent = tokens("a:\n\t\tb\n")
        .into_iter()
        .find(|token| token.kind == Indent)
        .unwrap();

    assert_eq!(indent.lexeme, "\t\t");
    assert_eq!(indent.span.len(), 2);
}

#[test]
fn test_dedent_is_empty_and_sits_at_the_first_token_of_the_line() {
    let dedent = tokens("a:\n    b\nc\n")
        .into_iter()
        .find(|token| token.kind == Dedent)
        .unwrap();

    assert_eq!(dedent.lexeme, "");
    assert!(dedent.span.is_empty());
    assert_eq!(dedent.span.start.offset, 9);
    assert_eq!(dedent.span.start.line, 3);
    assert_eq!(dedent.span.start.column, 1);
}

#[test]
fn test_dedent_to_a_nonzero_level_sits_after_the_remaining_indentation() {
    let dedent = tokens("a:\n    b:\n        c\n    d\n")
        .into_iter()
        .find(|token| token.kind == Dedent)
        .unwrap();

    assert!(dedent.span.is_empty());
    assert_eq!(dedent.span.start.offset, 24);
    assert_eq!(dedent.span.start.line, 4);
    assert_eq!(dedent.span.start.column, 5);
}

#[test]
fn test_first_token_position_after_indentation() {
    let token = tokens("a:\n    b")
        .into_iter()
        .find(|token| token.kind == Identifier && token.lexeme == "b")
        .unwrap();

    assert_eq!(token.span.start.offset, 7);
    assert_eq!(token.span.start.line, 2);
    assert_eq!(token.span.start.column, 5);
}

#[test]
fn test_dedents_at_end_of_source_sit_at_the_end_of_the_source() {
    let result = tokens("a:\n    b:\n        c");
    let tail = &result[result.len() - 3..];

    assert_eq!(
        tail.iter().map(|token| token.kind).collect::<Vec<_>>(),
        vec![Newline, Dedent, Dedent]
    );

    for token in tail {
        assert!(token.span.is_empty());
        assert_eq!(token.span.start.offset, 19);
        assert_eq!(token.span.start.line, 3);
        assert_eq!(token.span.start.column, 10);
    }
}

#[test]
fn test_eof_is_stable_after_dedents() {
    let mut lexer = Lexer::new("a:\n    b\n");

    for expected in [
        Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent, Eof, Eof, Eof,
    ] {
        assert_eq!(lexer.next_token().unwrap().kind, expected);
    }
}

// ----------------------------------------------------------
// Blank lines
// ----------------------------------------------------------

#[test]
fn test_blank_lines_produce_no_tokens() {
    assert_eq!(
        kinds("a:\n    b\n\n\n    c\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_blank_lines_between_top_level_statements() {
    assert_eq!(
        kinds("let a be 1\n\nlet b be 2\n"),
        vec![
            Let,
            Identifier,
            Be,
            IntegerLiteral,
            Newline,
            Let,
            Identifier,
            Be,
            IntegerLiteral,
            Newline,
        ]
    );
}

#[test]
fn test_whitespace_only_lines_are_blank_lines() {
    assert_eq!(
        kinds("a:\n    b\n  \n        \n    c\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_leading_blank_lines() {
    assert_eq!(
        kinds("\n\nlet x be 1\n"),
        vec![Let, Identifier, Be, IntegerLiteral, Newline]
    );
}

#[test]
fn test_blank_line_before_dedent() {
    assert_eq!(
        kinds("a:\n    b\n\nc\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent, Identifier, Newline,
        ]
    );
}

#[test]
fn test_blank_lines_at_end_of_source() {
    assert_eq!(
        kinds("a:\n    b\n\n\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_blank_line_may_use_the_other_indentation_character() {
    // Blank lines are never checked, so a tab-only line in a space-indented file is fine.
    assert_eq!(
        kinds("a:\n    b\n\t\n    c\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_source_without_code_has_no_tokens() {
    for source in [
        "",
        "\n",
        "\n\n",
        "   ",
        "  \n  ",
        "# only a comment",
        "# comment\n",
        "  # indented comment\n# another",
    ] {
        assert_eq!(kinds(source), vec![], "source: {source:?}");
    }
}

// ----------------------------------------------------------
// Comments
// ----------------------------------------------------------

#[test]
fn test_comment_only_lines_do_not_affect_indentation() {
    let source = "a:\n    b\n# at level zero\n        # deeper than the block\n  # shallower than the block\n    c\n";

    assert_eq!(
        kinds(source),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_comment_line_may_use_the_other_indentation_character() {
    assert_eq!(
        kinds("a:\n    b\n\t# indented with a tab\n    c\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Identifier, Newline, Dedent,
        ]
    );
}

#[test]
fn test_comment_between_blocks_at_top_level() {
    assert_eq!(
        kinds("a:\n    b\n# note\nc\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent, Identifier, Newline,
        ]
    );
}

#[test]
fn test_trailing_comments_on_code_lines() {
    assert_eq!(
        kinds("a: # header\n    b # body\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent,
        ]
    );
}

// ----------------------------------------------------------
// Synthetic Newline at the end of the source
// ----------------------------------------------------------

#[test]
fn test_synthetic_newline_closes_the_last_line() {
    let result = tokens("let x be 1");

    assert_eq!(
        result.iter().map(|token| token.kind).collect::<Vec<_>>(),
        vec![Let, Identifier, Be, IntegerLiteral, Newline]
    );

    let newline = result.last().unwrap();

    assert_eq!(newline.lexeme, "");
    assert!(newline.span.is_empty());
    assert_eq!(newline.span.start.offset, 10);
    assert_eq!(newline.span.start.line, 1);
    assert_eq!(newline.span.start.column, 11);
}

#[test]
fn test_no_synthetic_newline_after_a_real_one() {
    let result = tokens("let x be 1\n");
    let newlines: Vec<&Token> = result
        .iter()
        .filter(|token| token.kind == Newline)
        .collect();

    assert_eq!(newlines.len(), 1);
    assert_eq!(newlines[0].lexeme, "\n");
}

#[test]
fn test_synthetic_newline_comes_before_the_dedents() {
    let result = tokens("a:\n    b");

    assert_eq!(
        result.iter().map(|token| token.kind).collect::<Vec<_>>(),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent
        ]
    );
    assert_eq!(result[5].lexeme, "");
}

#[test]
fn test_synthetic_newline_after_a_trailing_comment() {
    assert_eq!(kinds("a # note"), vec![Identifier, Newline]);
}

#[test]
fn test_synthetic_newline_after_trailing_spaces() {
    assert_eq!(kinds("a   "), vec![Identifier, Newline]);
}

#[test]
fn test_no_synthetic_newline_after_a_whitespace_only_last_line() {
    assert_eq!(kinds("a\n   "), vec![Identifier, Newline]);
}

// ----------------------------------------------------------
// Line endings
// ----------------------------------------------------------

#[test]
fn test_crlf_line_endings() {
    let result = tokens("a:\r\n    b\r\nc\r\n");

    assert_eq!(
        result.iter().map(|token| token.kind).collect::<Vec<_>>(),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent, Identifier, Newline,
        ]
    );

    for token in result.iter().filter(|token| token.kind == Newline) {
        assert_eq!(token.lexeme, "\r\n");
    }
}

#[test]
fn test_cr_line_endings() {
    assert_eq!(
        kinds("a:\r    b\rc\r"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Dedent, Identifier, Newline,
        ]
    );
}

#[test]
fn test_crlf_blank_lines() {
    assert_eq!(
        kinds("a:\r\n    b\r\n\r\n    c\r\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Newline, Identifier, Newline, Dedent,
        ]
    );
}

// ----------------------------------------------------------
// What the Lexer leaves to the Parser
// ----------------------------------------------------------

#[test]
fn test_first_line_may_be_indented() {
    // The Parser rejects an unexpected Indent; the Lexer only reports the layout.
    assert_eq!(kinds("  a\n"), vec![Indent, Identifier, Newline, Dedent]);
}

#[test]
fn test_indent_without_a_block_header() {
    assert_eq!(
        kinds("let x be 1\n    let y be 2\n"),
        vec![
            Let,
            Identifier,
            Be,
            IntegerLiteral,
            Newline,
            Indent,
            Let,
            Identifier,
            Be,
            IntegerLiteral,
            Newline,
            Dedent,
        ]
    );
}

#[test]
fn test_no_implicit_line_continuation_inside_brackets() {
    // Line breaks inside brackets still end the logical line (Token Specification, rule 10).
    assert_eq!(
        kinds("f(\n    1\n)\n"),
        vec![
            Identifier,
            LeftParen,
            Newline,
            Indent,
            IntegerLiteral,
            Newline,
            Dedent,
            RightParen,
            Newline,
        ]
    );
}

#[test]
fn test_whitespace_inside_a_line_is_not_indentation() {
    assert_eq!(
        kinds("a:\n    b\twith\ttabs\n"),
        vec![
            Identifier, Colon, Newline, Indent, Identifier, Identifier, Identifier, Newline,
            Dedent,
        ]
    );
    assert_eq!(
        kinds("let\tx\tbe\t1\n"),
        vec![Let, Identifier, Be, IntegerLiteral, Newline]
    );
}

// ----------------------------------------------------------
// MixedIndentation
// ----------------------------------------------------------

#[test]
fn test_tab_after_space_indented_lines_is_mixed_indentation() {
    // The offending line stays at the current level: no Indent or Dedent for it.
    assert_eq!(
        results("a:\n    b\n\tc\n"),
        vec![
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Newline),
            Err(LexerErrorKind::MixedIndentation),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
        ]
    );
}

#[test]
fn test_spaces_after_tab_indented_lines_is_mixed_indentation() {
    assert_eq!(
        results("a:\n\tb\n    c\n"),
        vec![
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Newline),
            Err(LexerErrorKind::MixedIndentation),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
        ]
    );
}

#[test]
fn test_mixed_indentation_span_covers_the_indentation() {
    let error = first_error("a:\n    b\n\tc\n");

    assert_eq!(error.kind, LexerErrorKind::MixedIndentation);
    assert_eq!(error.span.start.offset, 9);
    assert_eq!(error.span.start.line, 3);
    assert_eq!(error.span.start.column, 1);
    assert_eq!(error.span.end.offset, 10);
    assert_eq!(error.span.end.column, 2);
}

#[test]
fn test_tab_and_space_on_one_line_is_mixed_indentation() {
    for source in ["a:\n \tb\n", "a:\n\t b\n", "a:\n  \t  b\n"] {
        let error = first_error(source);

        assert_eq!(
            error.kind,
            LexerErrorKind::MixedIndentation,
            "source: {source:?}"
        );
        assert_eq!(error.span.start.line, 2, "source: {source:?}");
        assert_eq!(error.span.start.column, 1, "source: {source:?}");
    }
}

#[test]
fn test_mixed_indentation_applies_to_the_whole_file() {
    // Each block is consistent by itself, but the file mixes tabs and spaces.
    let error = first_error("a:\n\tb\nc:\n    d\n");

    assert_eq!(error.kind, LexerErrorKind::MixedIndentation);
    assert_eq!(error.span.start.line, 4);
}

#[test]
fn test_line_after_mixed_indentation_continues_at_the_current_level() {
    assert_eq!(
        results("a:\n    b\n\tc\n    d\n"),
        vec![
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Newline),
            Err(LexerErrorKind::MixedIndentation),
            Ok(Identifier),
            Ok(Newline),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
        ]
    );
}

#[test]
fn test_unindented_lines_never_conflict() {
    assert_eq!(
        kinds("\ta\nb\n\tc\n"),
        vec![
            Indent, Identifier, Newline, Dedent, Identifier, Newline, Indent, Identifier, Newline,
            Dedent,
        ]
    );
}

#[test]
fn test_mixed_indentation_error_message() {
    assert_eq!(
        first_error("a:\n    b\n\tc\n").to_string(),
        "mixed tabs and spaces in indentation at line 3, column 1"
    );
}

// ----------------------------------------------------------
// InvalidIndentation
// ----------------------------------------------------------

#[test]
fn test_dedent_to_a_level_that_was_never_opened() {
    // Levels are 0, 4, and 8; a line indented by 6 matches none of them.
    assert_eq!(
        results("a:\n    b:\n        c\n      d\n"),
        vec![
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
            Err(LexerErrorKind::InvalidIndentation),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
        ]
    );
}

#[test]
fn test_invalid_indentation_span_covers_the_indentation() {
    let error = first_error("a:\n    b:\n        c\n      d\n");

    assert_eq!(error.kind, LexerErrorKind::InvalidIndentation);
    assert_eq!(error.span.start.offset, 20);
    assert_eq!(error.span.start.line, 4);
    assert_eq!(error.span.start.column, 1);
    assert_eq!(error.span.end.offset, 26);
    assert_eq!(error.span.end.column, 7);
}

#[test]
fn test_dedent_between_two_levels_is_invalid() {
    assert_eq!(
        results("a:\n        b\n    c\n"),
        vec![
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
            Err(LexerErrorKind::InvalidIndentation),
            Ok(Identifier),
            Ok(Newline),
        ]
    );
}

#[test]
fn test_dedent_to_a_level_between_zero_and_the_first_block_is_invalid() {
    assert_eq!(
        results("a:\n    b\n  c\n"),
        vec![
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
            Err(LexerErrorKind::InvalidIndentation),
            Ok(Identifier),
            Ok(Newline),
        ]
    );
}

#[test]
fn test_lexer_continues_after_invalid_indentation() {
    assert_eq!(
        results("a:\n    b:\n        c\n      d\ne\n"),
        vec![
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
            Err(LexerErrorKind::InvalidIndentation),
            Ok(Identifier),
            Ok(Newline),
            Ok(Dedent),
            Ok(Identifier),
            Ok(Newline),
        ]
    );
}

#[test]
fn test_invalid_indentation_error_message() {
    assert_eq!(
        first_error("a:\n        b\n    c\n").to_string(),
        "invalid indentation level at line 3, column 1"
    );
}

// ----------------------------------------------------------
// Interaction with other lexical errors
// ----------------------------------------------------------

#[test]
fn test_unterminated_string_on_an_indented_line() {
    assert_eq!(
        results("a:\n    \"abc\nb\n"),
        vec![
            Ok(Identifier),
            Ok(Colon),
            Ok(Newline),
            Ok(Indent),
            Err(LexerErrorKind::UnterminatedString),
            Ok(Newline),
            Ok(Dedent),
            Ok(Identifier),
            Ok(Newline),
        ]
    );
}

#[test]
fn test_line_with_an_unexpected_character_still_ends_with_newline() {
    assert_eq!(
        results("$\nx\n"),
        vec![
            Err(LexerErrorKind::UnexpectedCharacter { character: '$' }),
            Ok(Newline),
            Ok(Identifier),
            Ok(Newline),
        ]
    );
}
