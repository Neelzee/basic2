use crate::{
    common::{BinOp, Primitive, UniOp},
    lexer::utils::{
        B2Result, Span,
        consts::{
            FUNCTION_CALL_DELIMITER, FUNCTION_CALL_END, FUNCTION_CALL_START, GROUP_END,
            GROUP_START, LIST_DELIMITER, LIST_END, LIST_START, STRUCT_END_KW,
            STRUCT_FIELD_ASSIGNMENT, STRUCT_FIELD_END, STRUCT_FIELD_IMPL_KW, STRUCT_KW,
            STRUCT_START_KW, TUPLE_DELIMITER, TUPLE_END, TUPLE_START,
        },
        helper_parsers::{parse_identifier, parse_poly_list_with},
    },
};
use nom::{
    Parser,
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{multispace0, space0},
    error::{ErrorKind, FromExternalError, context},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};
use nom_language::error::VerboseError;

#[derive(Debug, PartialEq)]
pub enum LexExpr {
    Literal(Primitive),
    Tuple(Box<Self>, Box<Self>),
    List(Vec<Self>),
    Variable(String),
    Group(Box<Self>),
    FunctionCall {
        identifier: String,
        arguments: Vec<Self>,
    },
    UniOp {
        op: UniOp,
        operand: Box<Self>,
    },
    BinOp {
        op: BinOp,
        left_operand: Box<Self>,
        right_operand: Box<Self>,
    },
    Struct {
        identifier: String,
        field_implementations: Vec<(String, Self)>,
    },
}

impl LexExpr {
    pub fn parse_expr(input: Span) -> B2Result<Self> {
        alt((
            Self::parse_unary_operation,
            Self::parse_binary_operation,
            Self::parse_literal,
            Self::parse_group,
            Self::parse_tuple,
            Self::parse_list,
            Self::parse_function_call,
            Self::parse_struct,
            Self::parse_variable,
        ))
        .parse(input)
    }

    pub fn parse_expr_excl_binary(input: Span) -> B2Result<Self> {
        alt((
            Self::parse_unary_operation,
            Self::parse_literal,
            Self::parse_group,
            Self::parse_tuple,
            Self::parse_list,
            Self::parse_function_call,
            Self::parse_struct,
            Self::parse_variable,
        ))
        .parse(input)
    }

    pub fn parse_literal(input: Span) -> B2Result<Self> {
        Primitive::parse_primitive(input).map(|(rem, p)| (rem, Self::Literal(p)))
    }

    pub fn parse_tuple(input: Span) -> B2Result<Self> {
        context(
            "tuple-expr",
            separated_pair(
                preceded(
                    permutation((tag(TUPLE_START), multispace0)),
                    context("tuple-fst-expr", Self::parse_expr),
                ),
                permutation((tag(TUPLE_DELIMITER), multispace0)),
                terminated(
                    context("tuple-snd-expr", Self::parse_expr),
                    permutation((tag(TUPLE_END), multispace0)),
                ),
            ),
        )
        .map(|(fst, snd)| Self::Tuple(Box::new(fst), Box::new(snd)))
        .parse(input)
    }

    pub fn parse_list(input: Span) -> B2Result<Self> {
        terminated(
            preceded(
                context("list-start", tag(LIST_START)),
                separated_list0(
                    context(
                        "list-delimiter",
                        permutation((tag(LIST_DELIMITER), multispace0)),
                    ),
                    context("list-element", Self::parse_expr),
                ),
            ),
            context("list-end", permutation((tag(LIST_END), multispace0))),
        )
        .map(|xs| Self::List(xs))
        .parse(input)
    }

    pub fn parse_variable(input: Span) -> B2Result<Self> {
        match parse_identifier
            .map(|s: Span| Self::Variable(s.to_string()))
            .parse(input)?
        {
            // TODO: Figure out a better way to not allow keywords as identifiers
            (_, LexExpr::Variable(ident)) if matches!(ident.as_str(), STRUCT_KW) => {
                Err(nom::Err::Error(VerboseError::from_external_error(
                    input,
                    ErrorKind::Fail,
                    "not valid identifier",
                )))
            }
            res @ (_, _) => Ok(res),
        }
    }

    pub fn parse_group(input: Span) -> B2Result<Self> {
        let (rem, expr) =
            delimited(tag(GROUP_START), Self::parse_expr, tag(GROUP_END)).parse(input)?;
        Ok((rem, Self::Group(Box::new(expr))))
    }

    pub fn parse_function_call(input: Span) -> B2Result<Self> {
        let (i, identifier) = parse_identifier.map(|s| s.to_string()).parse(input)?;
        let (rem, arguments) = parse_poly_list_with(
            FUNCTION_CALL_START,
            FUNCTION_CALL_DELIMITER,
            FUNCTION_CALL_END,
            Self::parse_expr,
        )
        .parse(i)?;

        Ok((
            rem,
            Self::FunctionCall {
                identifier,
                arguments,
            },
        ))
    }

    pub fn parse_unary_operation(input: Span) -> B2Result<Self> {
        let (i, op) = UniOp::parse_unary_operation_symbol.parse(input)?;
        Self::parse_expr
            .map(|expr| Self::UniOp {
                op,
                operand: Box::new(expr),
            })
            .parse(i)
    }

    pub fn parse_binary_operation(input: Span) -> B2Result<Self> {
        let (i, (left_expr, op)) = pair(
            context(
                "binary-operation-left-operand",
                Self::parse_expr_excl_binary,
            ),
            preceded(space0, BinOp::parse_symbol),
        )
        .parse(input)?;
        let (rem, right_expr) = context(
            "binary-operation-right-operand",
            preceded(space0, Self::parse_expr),
        )
        .parse(i)?;
        Ok((
            rem,
            Self::BinOp {
                op,
                left_operand: Box::new(left_expr),
                right_operand: Box::new(right_expr),
            },
        ))
    }

    pub fn parse_struct(input: Span) -> B2Result<Self> {
        let (i, identifier) = preceded(
            tag(STRUCT_KW),
            preceded(space0, parse_identifier.map(|s| s.to_string())),
        )
        .parse(input)?;
        let (rem, field_implementations) = terminated(
            preceded(
                multispace0,
                preceded(
                    tag(STRUCT_START_KW),
                    many0(preceded(multispace0, Self::parse_struct_field)),
                ),
            ),
            preceded(multispace0, tag(STRUCT_END_KW)),
        )
        .parse(i)?;
        Ok((
            rem,
            Self::Struct {
                identifier,
                field_implementations,
            },
        ))
    }

    pub fn parse_struct_field(input: Span) -> B2Result<(String, Self)> {
        pair(
            pair(
                preceded(
                    tag(STRUCT_FIELD_IMPL_KW),
                    preceded(space0, parse_identifier.map(|s| s.to_string())),
                ),
                preceded(
                    space0,
                    preceded(
                        tag(STRUCT_FIELD_ASSIGNMENT),
                        preceded(space0, Self::parse_expr),
                    ),
                ),
            ),
            tag(STRUCT_FIELD_END),
        )
        .map(|(f, _)| f)
        .parse(input)
    }
}
