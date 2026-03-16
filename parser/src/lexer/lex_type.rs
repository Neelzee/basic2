use crate::lexer::utils::{
    B2Error, B2Result, Span,
    consts::{
        BOOL_TYPE_KW, FUNCTION_TYPE_ARROW_KW, FUNCTION_TYPE_END, FUNCTION_TYPE_START, INT_TYPE_KW,
        LIST_END, LIST_START, STR_TYPE_KW, TUPLE_DELIMITER, TUPLE_END, TUPLE_START,
    },
    helper_parsers::{parse_identifier, parse_poly_list_with},
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::{complete::space0, streaming::multispace0},
    error::{ErrorKind, ParseError, context},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};

#[derive(Debug, Clone, PartialEq)]
pub enum LexType<'a> {
    Str,
    Int,
    Bool,
    Tuple {
        fst: Box<Self>,
        snd: Box<Self>,
    },
    List(Box<Self>),
    /// Can be a Type alias, a generic, an enum, and a struct
    TypeVar(&'a str),
    FnType {
        input: Box<Self>,
        output: Box<Self>,
    },
}

impl<'a> LexType<'a> {
    pub fn parse_type(input: Span<'a>) -> B2Result<'a, Self> {
        alt((Self::parse_function_type, Self::parse_type_excl_fn)).parse(input)
    }

    pub fn parse_type_excl_fn(input: Span<'a>) -> B2Result<'a, Self> {
        alt((
            tag(STR_TYPE_KW).map(|_| Self::Str),
            tag(INT_TYPE_KW).map(|_| Self::Int),
            tag(BOOL_TYPE_KW).map(|_| Self::Bool),
            Self::parse_tuple_type,
            Self::parse_list,
            Self::parse_type_var,
        ))
        .parse(input)
    }

    pub fn parse_tuple_type(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_list(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_type_var(input: Span<'a>) -> B2Result<'a, Self> {
        context("type-var", parse_identifier)
            .map(Self::TypeVar)
            .parse(input)
    }

    pub fn parse_function_type(input: Span<'a>) -> B2Result<'a, Self> {
        let (rem, res) = context(
            "function-type",
            alt((
                delimited(
                    (tag(FUNCTION_TYPE_START), multispace0),
                    separated_pair(
                        context("input-type", Self::parse_type_excl_fn.map(|t| Box::new(t))),
                        context(
                            "function-arrow",
                            (multispace0, tag(FUNCTION_TYPE_ARROW_KW), multispace0),
                        ),
                        context("output-type", Self::parse_type.map(|t| Box::new(t))),
                    ),
                    (multispace0, tag(FUNCTION_TYPE_END)),
                )
                .map(|(input, output)| Ok(Self::FnType { input, output })),
                parse_poly_list_with(
                    FUNCTION_TYPE_START,
                    FUNCTION_TYPE_ARROW_KW,
                    FUNCTION_TYPE_END,
                    Self::parse_type,
                )
                .map(|mut xs| {
                    let x = xs.pop();
                    xs.reverse();
                    match x {
                        Some(x) if !xs.is_empty() => {
                            Ok(xs.into_iter().fold(x, |i, o| Self::FnType {
                                input: Box::new(o),
                                output: Box::new(i),
                            }))
                        }
                        _ => Err(()),
                    }
                }),
            )),
        )
        .parse(input)?;

        match res {
            Ok(res) => Ok((rem, res)),
            Err(_) => Err(nom::Err::Error(B2Error::from_error_kind(
                input,
                ErrorKind::Fail,
            ))),
        }
    }
}
