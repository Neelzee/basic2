use crate::common::Primitive;
use crate::lexer::lex_expr::LexExpr;
use crate::lexer::lex_type::LexType;
use crate::lexer::utils::helper_parsers::parse_comment;
use crate::lexer::utils::helper_parsers::parse_identifier;
use crate::lexer::utils::helper_parsers::parse_parameters;
use crate::lexer::utils::helper_parsers::parse_poly_list_with;
use crate::lexer::utils::{B2Result, Span};
use nom::IResult;
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
fn test_parse_poly_list_with<P, O>(
    #[case] start: &str,
    #[case] delimiter: &str,
    #[case] end: &str,
    #[case] parser: P,
    #[case] input: &str,
    #[case] expected: Vec<O>,
    #[case] remainder: &str,
) where
    O: std::fmt::Debug + PartialEq,
    P: Fn(Span) -> B2Result<O> + Clone,
{
    let result = parse_poly_list_with(start, delimiter, end, parser).parse(Span::new(input));
    assert!(result.is_ok(), "{:?}", result.unwrap_err());
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
#[case("#daj\nfoo", true, "\nfoo")]
fn test_comment_parse(#[case] ident: &str, #[case] is_ok: bool, #[case] remainder: &str) {
    let res = parse_comment(Span::new(ident));
    let ok = if is_ok { res.is_ok() } else { res.is_err() };
    assert!(ok, "{res:?}");
    if res.is_ok() {
        assert_eq!(res.unwrap().0.to_string(), remainder.to_string());
    }
}
