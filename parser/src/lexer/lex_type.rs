use crate::lexer::utils::{
    B2Result, Span,
    consts::{
        BOOL_TYPE_KW, INT_TYPE_KW, LIST_END, LIST_START, STR_TYPE_KW, TUPLE_DELIMITER, TUPLE_START,
    },
};
use nom::{
    Parser,
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::space0,
    error::context,
    multi::many0,
    sequence::{delimited, pair, preceded, separated_pair},
};

#[derive(Debug, Clone, PartialEq)]
pub enum LexType {
    Str,
    Int,
    Bool,
    Tuple { fst: Box<Self>, snd: Box<Self> },
    List(Box<Self>),
}

impl LexType {
    pub fn parse_type(input: Span) -> B2Result<Self> {
        alt((
            tag(STR_TYPE_KW).map(|_| Self::Str),
            tag(INT_TYPE_KW).map(|_| Self::Int),
            tag(BOOL_TYPE_KW).map(|_| Self::Bool),
            Self::parse_tuple_type,
            Self::parse_list,
        ))
        .parse(input)
    }

    pub fn parse_tuple_type(input: Span) -> B2Result<Self> {
        context(
            "tuple-type-parsing",
            separated_pair(
                preceded(
                    permutation((many0(space0), tag(TUPLE_START))),
                    Self::parse_type,
                ),
                preceded(many0(space0), tag(TUPLE_DELIMITER)),
                preceded(
                    permutation((many0(space0), tag(TUPLE_START))),
                    Self::parse_type,
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
}
