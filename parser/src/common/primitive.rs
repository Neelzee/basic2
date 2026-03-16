use std::ops::Neg;

use crate::{
    common::ToB2,
    lexer::utils::{
        B2Result, Span,
        consts::{FLOAT_DOT_KW, FLOAT_KW, STRING_CHAR, STRING_KW},
    },
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::{opt, recognize},
    error::context,
    number::complete::float,
    sequence::{delimited, terminated},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Primitive<'a> {
    Int(i32),
    Float(f32),
    Str(&'a str),
    Bool(bool),
}

impl<'a> Primitive<'a> {
    pub fn parse_primitive(input: Span<'a>) -> B2Result<'a, Self> {
        alt((
            Self::parse_str,
            Self::parse_float,
            Self::parse_int,
            Self::parse_bool,
        ))
        .parse(input)
    }

    pub fn parse_bool(input: Span) -> B2Result<Self> {
        alt((
            tag("TRUE").map(|_| Self::Bool(true)),
            tag("FALSE").map(|_| Self::Bool(false)),
        ))
        .parse(input)
    }

    pub fn parse_int(input: Span) -> B2Result<Self> {
        let (i, negative) = is_negative.parse(input)?;
        digit1
            .map(|i: Span| {
                Self::Int(
                    i.to_string()
                        .parse::<i32>()
                        .expect(&format!("Couldnt parse i32 of: {i}"))
                        * if negative { -1 } else { 1 },
                )
            })
            .parse(i)
    }

    pub fn parse_float(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "primitive-float",
            (
                is_negative,
                recognize(alt((
                    terminated(digit1, tag(FLOAT_KW)),
                    delimited(tag(FLOAT_DOT_KW), digit1, opt(tag(FLOAT_KW))),
                    terminated(
                        recognize((digit1, tag(FLOAT_DOT_KW), digit1)),
                        opt(tag(FLOAT_KW)),
                    ),
                )))
                .and_then(float),
            ),
        )
        .map(|(negative, val)| Self::Float(if negative { val.neg() } else { val }))
        .parse(input)
    }

    pub fn parse_str(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "string-primitive",
            delimited(
                context("string-start", tag(STRING_KW)),
                context("string-content", take_till(|c| c == STRING_CHAR)),
                context("string-end", tag(STRING_KW)),
            ),
        )
        .map(|s: Span<'a>| Self::Str(&s))
        .parse(input)
    }
}

impl<'a> ToB2 for Primitive<'a> {
    fn to_b2(&self) -> String {
        match self {
            Primitive::Int(i) => i.to_string(),
            Primitive::Float(i) => i.to_string(),
            Primitive::Str(i) => format!("\"{i}\""),
            Primitive::Bool(i) if !i => "FALSE".to_string(),
            Primitive::Bool(_) => "TRUE".to_string(),
        }
    }
}

fn is_negative(input: Span) -> B2Result<bool> {
    opt(alt((tag("+"), tag("-"))))
        .map(|o| o.is_some_and(|s: Span| s.to_string() == "-"))
        .parse(input)
}

impl<'a> From<i32> for Primitive<'a> {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl<'a> From<&'a str> for Primitive<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl<'a> From<bool> for Primitive<'a> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
