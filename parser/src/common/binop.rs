use crate::{
    common::ToB2,
    lexer::utils::{
        B2Result, Span,
        consts::{
            ADD_KW, AND_KW, DIV_KW, EQ_KW, GEQ_KW, GT_KW, LEQ_KW, LT_KW, MOD_KW, MUL_KW, NEQ_KW,
            OR_KW, POW_KW, SUB_KW,
        },
    },
};
use nom::{
    Parser, branch::alt, bytes::complete::tag, character::complete::space0, sequence::preceded,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOp {
    Add,
    Mul,
    Sub,
    Div,
    Pow,
    Eq,
    Geq,
    Gt,
    Leq,
    Lt,
    Neq,
    Mod,
    And,
    Or,
}

impl BinOp {
    pub fn parse_symbol(input: Span) -> B2Result<Self> {
        preceded(
            space0,
            alt((
                tag(ADD_KW).map(|_| Self::Add),
                tag(MUL_KW).map(|_| Self::Mul),
                tag(SUB_KW).map(|_| Self::Sub),
                tag(DIV_KW).map(|_| Self::Div),
                tag(POW_KW).map(|_| Self::Pow),
                tag(EQ_KW).map(|_| Self::Eq),
                tag(GEQ_KW).map(|_| Self::Geq),
                tag(LEQ_KW).map(|_| Self::Leq),
                tag(GT_KW).map(|_| Self::Gt),
                tag(LT_KW).map(|_| Self::Lt),
                tag(NEQ_KW).map(|_| Self::Neq),
                tag(MOD_KW).map(|_| Self::Mod),
                tag(AND_KW).map(|_| Self::And),
                tag(OR_KW).map(|_| Self::Or),
            )),
        )
        .parse(input)
    }
}

impl ToB2 for BinOp {
    fn to_b2(&self) -> String {
        match self {
            BinOp::Add => ADD_KW.to_string(),
            BinOp::Mul => MUL_KW.to_string(),
            BinOp::Sub => SUB_KW.to_string(),
            BinOp::Div => DIV_KW.to_string(),
            BinOp::Pow => POW_KW.to_string(),
            BinOp::Eq => EQ_KW.to_string(),
            BinOp::Geq => GEQ_KW.to_string(),
            BinOp::Gt => GT_KW.to_string(),
            BinOp::Leq => LEQ_KW.to_string(),
            BinOp::Lt => LT_KW.to_string(),
            BinOp::Neq => NEQ_KW.to_string(),
            BinOp::Mod => MOD_KW.to_string(),
            BinOp::And => AND_KW.to_string(),
            BinOp::Or => OR_KW.to_string(),
        }
    }
}
