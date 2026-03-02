use crate::lexer::{
    lex_expr::LexExpr,
    utils::{
        B2Result, Span,
        consts::{ASSIGNMENT_KW, SINGLE_LINE_COMMENT},
    },
};
use nom::{
    Parser,
    branch::{alt, permutation},
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, alphanumeric0, multispace0, space0},
    combinator::opt,
    error::context,
    multi::separated_list0,
    sequence::{pair, preceded, terminated},
};

pub fn parse_identifier(input: Span) -> B2Result<Span> {
    alt((tag("_"), alpha1))
        .and_then(alt((alphanumeric0, tag("_"))))
        .parse(input)
}

/// Parses a list-like input, with a specified start, end and delimiter
/// ```rust
/// use parser::lexer::{utils::{helper_parsers::parse_poly_list_with, Span}, lex_type::LexType};
/// use nom::Parser;
///
/// let input = "(INT, STR, BOOL)";
/// let result = parse_poly_list_with("(", ",", ")", LexType::parse_type).parse(Span::new(input));
/// assert!(result.is_ok(), "{result:?}");
/// assert_eq!(result.unwrap().0.to_string(), "");
///
/// let other_input = "(INT, STR, BOOL) some-other-string";
/// let result = parse_poly_list_with("(", ",", ")", LexType::parse_type).parse(Span::new(other_input));
/// assert!(result.is_ok(), "{result:?}");
/// assert_eq!(result.unwrap().0.to_string(), " some-other-string");
/// ```
pub fn parse_poly_list_with<'a, P>(
    start: &'static str,
    delimiter: &'static str,
    end: &'static str,
    parser: P,
) -> impl Parser<Span<'a>, Output = Vec<P::Output>, Error = P::Error>
where
    P: Parser<Span<'a>>,
{
    terminated(
        preceded(
            tag(start),
            separated_list0(permutation((tag(delimiter), multispace0)), parser),
        ),
        tag(end),
    )
}

pub fn parse_parameters(input: Span) -> B2Result<(String, Option<LexExpr>)> {
    context(
        "parse-parameters",
        pair(
            context(
                "parse-parameter-identifier",
                parse_identifier.map(|s| s.to_string()),
            ),
            context(
                "parse-parameter-optional-default-argument",
                opt(preceded(
                    space0,
                    preceded(
                        context("parameter-default-argument-assignment", tag(ASSIGNMENT_KW)),
                        preceded(space0, LexExpr::parse_expr),
                    ),
                )),
            ),
        ),
    )
    .parse(input)
}

pub fn parse_comment(input: Span) -> B2Result<Span> {
    preceded(
        preceded(space0, tag(SINGLE_LINE_COMMENT)),
        take_till(|c| c == '\n'),
    )
    .parse(input)
}
