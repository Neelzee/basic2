use crate::lexer::{
    lex_expr::LexExpr,
    utils::{
        B2Result, Span,
        consts::{DECR_KW, INCR_KW, LIST_END, LIST_START},
    },
};
use nom::{
    Parser, branch::alt, bytes::complete::tag, character::complete::multispace0, error::context,
    sequence::delimited,
};

#[derive(Debug, PartialEq)]
pub enum Postfix {
    Incr,
    Decr,
    Index(LexExpr),
}

impl Postfix {
    pub fn parse_postfix(input: Span) -> B2Result<Self> {
        context(
            "post-fix",
            alt((
                tag(INCR_KW).map(|_| Self::Incr),
                tag(DECR_KW).map(|_| Self::Decr),
                Self::parse_index,
            )),
        )
        .parse(input)
    }

    pub fn parse_index(input: Span) -> B2Result<Self> {
        context(
            "post-fix-index",
            delimited(
                tag(LIST_START).and(multispace0),
                LexExpr::parse_expr,
                multispace0.and(tag(LIST_END)),
            ),
        )
        .map(|expr| Self::Index(expr))
        .parse(input)
    }
}
