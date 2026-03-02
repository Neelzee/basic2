use crate::lexer::utils::{
    B2Result, Span,
    consts::{DECR_KW, INCR_KW},
};
use nom::{Parser, branch::alt, bytes::complete::tag, error::context};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Postfix {
    Incr,
    Decr,
}

impl Postfix {
    pub fn parse_postfix(input: Span) -> B2Result<Self> {
        context(
            "post-fix",
            alt((
                tag(INCR_KW).map(|_| Self::Incr),
                tag(DECR_KW).map(|_| Self::Decr),
            )),
        )
        .parse(input)
    }
}
