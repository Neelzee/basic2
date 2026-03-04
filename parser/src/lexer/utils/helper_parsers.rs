use crate::lexer::{
    lex_expr::LexExpr,
    lex_stmt::LexStmt,
    utils::{
        B2Result, Span,
        consts::{ASSIGNMENT_KW, SINGLE_LINE_COMMENT, SINGLE_LINE_COMMENT_END},
    },
};
use nom::{
    Err::Error, Parser, branch::{alt, permutation}, bytes::complete::{tag, take_till}, character::complete::{alpha1, alphanumeric0, multispace0, multispace1, space0}, combinator::{eof, opt}, error::{ErrorKind, ParseError, context}, multi::{many0, many1, separated_list0}, sequence::{pair, preceded, terminated}
};
use nom_language::error::VerboseError;

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
    context(
        "parse-comment",
        terminated(
            preceded(
                space0,
                preceded(
                    tag(SINGLE_LINE_COMMENT),
                    alt((take_till(|c| c == '\n'), eof)),
                ),
            ),
            alt((tag(SINGLE_LINE_COMMENT_END), eof)),
        ),
    )
    .parse(input)
}

pub fn parse_statements(input: Span) -> B2Result<Vec<LexStmt>> {
    context(
        "multi-statement",
        many0(alt((
            consume_comments_and_multispace.map(|_| None),
            context("-statement", LexStmt::parse_statement).map(|x| Some(x))
        ))
        ),
    )
    .map(|xs| xs.into_iter().filter_map(|x| x).collect())
    .parse(input)
}

pub fn consume_comments_and_multispace(input: Span) -> B2Result<()> {
    context(
        "consume-comments-multiline",
        alt((
            multispace1,
            parse_comment
        ))
    )
    .map(|_| ())
    .parse(input)
}