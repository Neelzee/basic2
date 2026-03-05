use crate::lexer::utils::{
    B2Result, Span,
    consts::{
        BOOL_TYPE_KW, INT_TYPE_KW, LIST_END, LIST_START, STR_TYPE_KW, TUPLE_DELIMITER, TUPLE_END,
        TUPLE_START,
    },
    helper_parsers::parse_identifier,
};
use nom::{
    Parser,
    branch::{alt, permutation},
    bytes::complete::tag,
    character::{complete::space0, streaming::multispace0},
    error::context,
    multi::many0,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};

#[derive(Debug, Clone, PartialEq)]
pub enum LexType {
    Str,
    Int,
    Bool,
    Tuple { fst: Box<Self>, snd: Box<Self> },
    List(Box<Self>),
    TypeAlias(String),
}

impl LexType {
    pub fn parse_type(input: Span) -> B2Result<Self> {
        alt((
            tag(STR_TYPE_KW).map(|_| Self::Str),
            tag(INT_TYPE_KW).map(|_| Self::Int),
            tag(BOOL_TYPE_KW).map(|_| Self::Bool),
            Self::parse_tuple_type,
            Self::parse_list,
            Self::parse_type_alias,
        ))
        .parse(input)
    }

    pub fn parse_tuple_type(input: Span) -> B2Result<Self> {
        context(
            "tuple-type-parsing",
            separated_pair(
                context(
                    "tuple-type-fst",
                    preceded(
                        preceded(multispace0, tag(TUPLE_START)),
                        preceded(multispace0, Self::parse_type),
                    ),
                ),
                preceded(multispace0, tag(TUPLE_DELIMITER)),
                context(
                    "tuple-type-snd",
                    terminated(
                        preceded(multispace0, Self::parse_type),
                        preceded(multispace0, tag(TUPLE_END)),
                    ),
                ),
            )
            .map(|(fst, snd)| Self::Tuple {
                fst: Box::new(fst),
                snd: Box::new(snd),
            }),
        )
        .parse(input)
    }

    pub fn parse_list(input: Span) -> B2Result<Self> {
        context(
            "list-type-parsing",
            delimited(
                pair(tag(LIST_START), space0),
                Self::parse_type,
                pair(space0, tag(LIST_END)),
            )
            .map(|t| Self::List(Box::new(t))),
        )
        .parse(input)
    }

    pub fn parse_type_alias(input: Span) -> B2Result<Self> {
        context("type-alias", parse_identifier)
            .map(|s| Self::TypeAlias(s.to_string()))
            .parse(input)
    }
}
