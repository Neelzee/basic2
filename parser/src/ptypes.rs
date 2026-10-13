use common::{consts::{pstms::{ENUM_INDEXING, fns::{FUNCTION_GENERICS_DELIMITER, FUNCTION_GENERICS_END, FUNCTION_GENERICS_START, FUNCTION_TYPE_ARROW_KW, FUNCTION_TYPE_END, FUNCTION_TYPE_START}}, types::{BOOL_TYPE_KW, FLOAT_TYPE_KW, INT_TYPE_KW, LIST_END, LIST_START, NIL_TYPE_KW, SELF_TYPE_KW, STR_TYPE_KW, TUPLE_DELIMITER, TUPLE_END, TUPLE_START}}, types::{MonoType, PolyType, Type}};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::{complete::space0, streaming::multispace0},
    error::{ErrorKind, ParseError, context},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};
use crate::utils::{B2PError, B2PResult, Span, helper_parsers::{parse_identifier, parse_poly_list_with}};

pub struct TypeParser;

impl<'a> TypeParser {
    pub fn parse_type(input: Span<'a>) -> B2PResult<'a, Type> {
        alt((TypeParser::parse_function_type, TypeParser::parse_type_excl_fn)).parse(input)
    }

    pub fn parse_type_excl_fn(input: Span<'a>) -> B2PResult<'a, Type> {
        alt((
            tag(NIL_TYPE_KW).map(|_| Type::Mono(MonoType::Nil)),
            tag(STR_TYPE_KW).map(|_| Type::Mono(MonoType::Str)),
            tag(INT_TYPE_KW).map(|_| Type::Mono(MonoType::Int)),
            tag(FLOAT_TYPE_KW).map(|_| Type::Mono(MonoType::Float)),
            tag(BOOL_TYPE_KW).map(|_| Type::Mono(MonoType::Bool)),
            TypeParser::parse_tuple_type,
            TypeParser::parse_list,
            TypeParser::parse_enum_variant,
            TypeParser::parse_self_type,
            TypeParser::parse_type_var_gen,
            TypeParser::parse_type_var,
        ))
        .parse(input)
    }

    pub fn parse_tuple_type(input: Span<'a>) -> B2PResult<'a, Type> {
        context(
            "tuple-type-parsing",
            separated_pair(
                context(
                    "tuple-type-fst",
                    preceded(
                        preceded(multispace0, tag(TUPLE_START)),
                        preceded(multispace0, TypeParser::parse_type),
                    ),
                ),
                preceded(multispace0, tag(TUPLE_DELIMITER)),
                context(
                    "tuple-type-snd",
                    terminated(
                        preceded(multispace0, TypeParser::parse_type),
                        preceded(multispace0, tag(TUPLE_END)),
                    ),
                ),
            )
            .map(|(fst, snd)| {
                Type::Poly(PolyType::Tuple {
                    fst: Box::new(fst),
                    snd: Box::new(snd),
                })
            }),
        )
        .parse(input)
    }

    pub fn parse_list(input: Span<'a>) -> B2PResult<'a, Type> {
        context(
            "list-type-parsing",
            delimited(
                pair(tag(LIST_START), space0),
                TypeParser::parse_type,
                pair(space0, tag(LIST_END)),
            )
            .map(|t| Type::Poly(PolyType::List(Box::new(t)))),
        )
        .parse(input)
    }

    pub fn parse_type_var(input: Span<'a>) -> B2PResult<'a, Type> {
        context("type-var", parse_identifier)
            .map(|s| Type::Mono(MonoType::TypeVar(s)))
            .parse(input)
    }

    pub fn parse_type_var_gen(input: Span<'a>) -> B2PResult<'a, Type> {
        context(
            "type-var-with-generics",
            (
                parse_identifier,
                context(
                    "generics",
                    delimited(
                        tag(FUNCTION_GENERICS_START),
                        separated_list1(
                            (multispace0, tag(FUNCTION_GENERICS_DELIMITER), multispace0),
                            parse_identifier,
                        ),
                        tag(FUNCTION_GENERICS_END),
                    ),
                ),
            ),
        )
        .map(|(ident, gens)| Type::Mono(MonoType::TypeVarGen(ident, gens)))
        .parse(input)
    }

    pub fn parse_self_type(input: Span<'a>) -> B2PResult<'a, Type> {
        context("self-type", tag(SELF_TYPE_KW))
            .map(|_| Type::Mono(MonoType::SelfType))
            .parse(input)
    }

    pub fn fold_funs(mut xs: Vec<Type>) -> Result<Type, ()> {
        let x = xs.pop();
        xs.reverse();
        match x {
            Some(x) if !xs.is_empty() => Ok(xs.into_iter().fold(x, |i, o| {
                Type::Poly(PolyType::FnType {
                    input: Box::new(o),
                    output: Box::new(i),
                })
            })),
            _ => Err(()),
        }
    }

    pub fn parse_function_type(input: Span<'a>) -> B2PResult<'a, Type> {
        let (rem, res) = context(
            "function-type",
            alt((
                delimited(
                    (tag(FUNCTION_TYPE_START), multispace0),
                    separated_pair(
                        context("input-type", TypeParser::parse_type_excl_fn.map(|t| Box::new(t))),
                        context(
                            "function-arrow",
                            (multispace0, tag(FUNCTION_TYPE_ARROW_KW), multispace0),
                        ),
                        context("output-type", TypeParser::parse_type.map(|t| Box::new(t))),
                    ),
                    (multispace0, tag(FUNCTION_TYPE_END)),
                )
                .map(|(input, output)| Ok(Type::Poly(PolyType::FnType { input, output }))),
                parse_poly_list_with(
                    FUNCTION_TYPE_START,
                    FUNCTION_TYPE_ARROW_KW,
                    FUNCTION_TYPE_END,
                    TypeParser::parse_type,
                )
                .map(TypeParser::fold_funs),
            )),
        )
        .parse(input)?;

        match res {
            Ok(res) => Ok((rem, res)),
            Err(_) => Err(nom::Err::Error(B2PError::from_error_kind(
                input,
                ErrorKind::Fail,
            ))),
        }
    }

    pub fn parse_enum_variant(input: Span<'a>) -> B2PResult<'a, Type> {
        context(
            "enum-variant",
            separated_pair(parse_identifier, tag(ENUM_INDEXING), parse_identifier),
        )
        .map(|(e, v)| Type::Mono(MonoType::EnumVariant(e, v)))
        .parse(input)
    }
}