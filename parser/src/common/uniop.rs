use crate::{
    common::ToB2,
    lexer::utils::{B2LexResult, Span, consts::NEGATION_KW},
};
use nom::{Parser, bytes::complete::tag};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UniOp {
    /// Expression that starts with !
    Neg,
}

impl UniOp {
    pub fn parse_unary_operation_symbol(input: Span) -> B2LexResult<Self> {
        tag(NEGATION_KW).map(|_| Self::Neg).parse(input)
    }
}

impl ToB2 for UniOp {
    fn to_b2(&self) -> String {
        match self {
            UniOp::Neg => NEGATION_KW.to_string(),
        }
    }
}
