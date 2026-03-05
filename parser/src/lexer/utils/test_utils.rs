use crate::{
    common::{B2Op, binop::BinOp, primitive::Primitive},
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::LexStmt,
        lex_type::LexType,
        utils::{
            B2Result, Span, convert_error,
            helper_parsers::{
                consume_comments_and_multispace, parse_comment, parse_identifier, parse_parameters,
                parse_poly_list_with, parse_statements,
            },
        },
    },
};
use nom::Parser;
use rstest::rstest;

#[rstest]
#[case("(", ",", ")", LexType::parse_type, "()", Vec::new(), "")]
#[case(
    "(",
    ",",
    ")",
    LexType::parse_type,
    "(INT, INT)",
    vec![LexType::Int, LexType::Int],
    ""
)]
#[case(
    "(",
    ",",
    ")",
    parse_parameters,
    r##"(a = "foo", a)"##,
    vec![("a".to_string(), Some(LexExpr::Literal(Primitive::Str("foo".to_string())))), ("a".to_string(), None)],
    ""
)]
#[case(
    "(",
    ",",
    ")",
    LexExpr::parse_expr,
    r##"("Before: " + global)"##,
    vec![
        LexExpr::Op(Box::new(B2Op::Binary(
            LexExpr::Literal(Primitive::Str("Before: ".to_string())),
            BinOp::Add,
            LexExpr::Variable("global".to_string())
        )))
    ],
    ""
)]
fn test_parse_poly_list_with<P, O>(
    #[case] start: &'static str,
    #[case] delimiter: &'static str,
    #[case] end: &'static str,
    #[case] parser: P,
    #[case] input: &'static str,
    #[case] expected: Vec<O>,
    #[case] remainder: &'static str,
) where
    O: std::fmt::Debug + PartialEq,
    P: Fn(Span) -> B2Result<O> + Clone,
{
    let input = Span::new(input);
    let result = parse_poly_list_with(start, delimiter, end, parser).parse(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    let result = result.unwrap();
    assert_eq!(result.1, expected);
    assert_eq!(result.0.to_string(), remainder.to_string());
}

#[rstest]
#[case::valid_identifier_with_underscore("_foobar123", true)]
#[case::valid_identifier("foobar123", true)]
#[case::invalid_identifier("123foobar", false)]
fn test_identifier_parser(#[case] ident: &str, #[case] is_ok: bool) {
    let res = parse_identifier(Span::new(ident));
    let ok = if is_ok { res.is_ok() } else { res.is_err() };
    assert!(ok, "{res:?}")
}

#[rstest]
#[case("# foobar123", true, "")]
#[case("    # foobar123", true, "")]
#[case("daj", false, "")]
#[case("#daj", true, "")]
#[case("#daj\nfoo", true, "foo")]
#[case("# Everything in a module body is in a block", true, "")]
fn test_comment_parse(#[case] ident: &str, #[case] is_ok: bool, #[case] remainder: &str) {
    let res = parse_comment(Span::new(ident));
    let ok = if is_ok { res.is_ok() } else { res.is_err() };
    assert!(ok, "{res:?}");
    if res.is_ok() {
        assert_eq!(res.unwrap().0.to_string(), remainder.to_string());
    }
}

#[test]
fn test_parse_comments_only_consumes_single_line() {
    let input = Span::new(
        r##"    # Should be 2 lines after this comment is consumed

        "##,
    );
    let remainder = r##"
        "##;
    let res = parse_comment(input);
    assert!(res.is_ok(), "{}", convert_error(input, res.unwrap_err()));
    assert_eq!(res.unwrap().0.to_string(), remainder.to_string());
}

#[rstest]
#[case(
    r##"
    # Everything in a module body is in a block
    # This is a nested block
    DO
    # It has it's own scope
    # Statements have to end with ;
    END
    "##,
    vec![LexStmt::Block { body: Vec::new() }]
)]
fn test_parse_statements(#[case] input: &str, #[case] expected: Vec<LexStmt>) {
    let input = Span::new(input);
    let res = parse_statements(input);
    assert!(res.is_ok(), "{}", convert_error(input, res.unwrap_err()));
    let (remainder, stmt) = res.unwrap();
    assert_eq!(stmt, expected);
    assert_eq!(remainder.to_string(), String::new());
}
