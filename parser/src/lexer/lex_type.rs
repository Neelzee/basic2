use crate::lexer::utils::{
    B2LexError, B2LexResult, Span,
    consts::{
        BOOL_TYPE_KW, ENUM_INDEXING, FLOAT_TYPE_KW, FUNCTION_TYPE_ARROW_KW, FUNCTION_TYPE_END,
        FUNCTION_TYPE_START, INT_TYPE_KW, LIST_END, LIST_START, NIL_TYPE_KW, STR_TYPE_KW,
        TUPLE_DELIMITER, TUPLE_END, TUPLE_START,
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
    Nil,
    Str,
    Int,
    Float,
    Bool,
    Tuple {
        fst: Box<Self>,
        snd: Box<Self>,
    },
    List(Box<Self>),
    /// Can be a Type alias, a generic, and a struct
    TypeVar(&'a str),
    FnType {
        input: Box<Self>,
        output: Box<Self>,
    },
    /// # Example
    ///
    /// ```b2
    /// ENUMS Num
    ///   One;
    ///   Two;
    /// END
    ///
    /// DECL ConstOne() : Num.One;
    /// IMPL ConstOne()
    ///   RETURN Num.One;
    /// ```
    EnumVariant(&'a str, &'a str),
}

impl<'a> LexType<'a> {
    pub fn parse_type(input: Span<'a>) -> B2LexResult<'a, Self> {
        alt((Self::parse_function_type, Self::parse_type_excl_fn)).parse(input)
    }

    pub fn parse_type_excl_fn(input: Span<'a>) -> B2LexResult<'a, Self> {
        alt((
            tag(NIL_TYPE_KW).map(|_| Self::Nil),
            tag(STR_TYPE_KW).map(|_| Self::Str),
            tag(INT_TYPE_KW).map(|_| Self::Int),
            tag(FLOAT_TYPE_KW).map(|_| Self::Float),
            tag(BOOL_TYPE_KW).map(|_| Self::Bool),
            Self::parse_tuple_type,
            Self::parse_list,
            Self::parse_enum_variant,
            Self::parse_type_var,
        ))
        .parse(input)
    }

    pub fn parse_tuple_type(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_list(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_type_var(input: Span<'a>) -> B2LexResult<'a, Self> {
        context("type-var", parse_identifier)
            .map(Self::TypeVar)
            .parse(input)
    }

    pub fn parse_function_type(input: Span<'a>) -> B2LexResult<'a, Self> {
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
            Err(_) => Err(nom::Err::Error(B2LexError::from_error_kind(
                input,
                ErrorKind::Fail,
            ))),
        }
    }

    pub fn parse_enum_variant(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "enum-variant",
            separated_pair(parse_identifier, tag(ENUM_INDEXING), parse_identifier),
        )
        .map(|(e, v)| Self::EnumVariant(e, v))
        .parse(input)
    }
}
