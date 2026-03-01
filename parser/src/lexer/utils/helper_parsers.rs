use std::process::Output;

use crate::{
    common::{BinOp, Primitive, UniOp},
    lexer::{
        lex_expr::LexExpr,
        utils::{
            B2Result, Span, VerboseError, VerboseErrorKind,
            consts::{
                FUNCTION_CALL_DELIMITER, FUNCTION_CALL_END_CHAR, FUNCTION_CALL_START, GROUP_END,
                GROUP_START, LIST_DELIMITER, LIST_END_CHAR, LIST_START, SINGLE_LINE_COMMENT,
                TUPLE_DELIMITER, TUPLE_DELIMITER_CHAR, TUPLE_END_CHAR, TUPLE_START,
            },
        },
    },
};
use nom::{
    IResult, Parser,
    branch::{alt, permutation},
    bytes::complete::{tag, take, take_till, take_until},
    character::complete::{
        alpha1, alphanumeric0, char, digit1, multispace0, none_of, one_of, space0,
    },
    combinator::{cut, opt},
    error::context,
    multi::{many0, separated_list0},
    number::complete::float,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};

pub fn parse_identifier(input: Span) -> B2Result<Span> {
    alt((tag("_"), alpha1))
        .and_then(alt((alphanumeric0, tag("_"))))
        .parse(input)
}

/// Parses a list-like input, with a specified start, end and delimiter
/// ```rust
/// use parser::lexer::{utils::helper_parsers::parse_poly_list_with, lex_type::LexType};
/// use nom::Parser;
///
/// let input = "(INT, STR, BOOL)";
/// let result = parse_poly_list_with("(", ",", ')', LexType::parse_type).parse(input);
/// assert!(result.is_ok(), "{result:?}");
/// assert_eq!(result.unwrap().0, "");
///
/// let other_input = "(INT, STR, BOOL) some-other-string";
/// let result = parse_poly_list_with("(", ",", ')', LexType::parse_type).parse(other_input);
/// assert!(result.is_ok(), "{result:?}");
/// assert_eq!(result.unwrap().0, " some-other-string");
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

pub fn till_end_of_stmt(input: Span) -> B2Result<Span> {
    take_until(";").parse(input)
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
                opt(preceded(space0, preceded(tag("="), LexExpr::parse_expr))),
            ),
        ),
    )
    .parse(input)
}

pub fn parse_comment(input: Span) -> B2Result<Span> {
    preceded(
        preceded(multispace0, tag(SINGLE_LINE_COMMENT)),
        take_till(|c| c == '\n'),
    )
    .parse(input)
}
