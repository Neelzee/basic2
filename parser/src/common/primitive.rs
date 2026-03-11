use crate::lexer::utils::{
    B2Result, Span,
    consts::{STRING_CHAR, STRING_KW},
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{char, digit1},
    combinator::{map, opt, recognize},
    error::context,
    number::complete::float,
    sequence::{delimited, pair},
};

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

impl Primitive {
    pub fn parse_primitive(input: Span) -> B2Result<Self> {
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
                    i.parse::<i32>()
                        .expect(&format!("Couldnt parse i32 of: {i}"))
                        * if negative { -1 } else { 1 },
                )
            })
            .parse(i)
    }

    pub fn parse_float(input: Span) -> B2Result<Self> {
        let (i, negative) = is_negative(input)?;

        recognize(alt((
            map((digit1, pair(char('.'), opt(digit1))), |_| ()),
            map((char('.'), digit1), |_| ()),
        )))
        .and_then(float)
        .map(|f| Self::Float(f * if negative { -1f32 } else { 1f32 }))
        .parse(i)
    }

    pub fn parse_str(input: Span) -> B2Result<Self> {
        context(
            "string-primitive",
            delimited(
                context("string-start", tag(STRING_KW)),
                context("string-content", take_till(|c| c == STRING_CHAR)),
                context("string-end", tag(STRING_KW)),
            ),
        )
        .map(|s: Span| Self::Str(s.to_string()))
        .parse(input)
    }
}

fn is_negative(input: Span) -> B2Result<bool> {
    opt(alt((tag("+"), tag("-"))))
        .map(|o| o.is_some_and(|s: Span| s.to_string() == "-"))
        .parse(input)
}

impl From<i32> for Primitive {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<&str> for Primitive {
    fn from(value: &str) -> Self {
        Self::Str(value.to_string())
    }
}

impl From<String> for Primitive {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<bool> for Primitive {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
