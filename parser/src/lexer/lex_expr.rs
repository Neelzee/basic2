use crate::{
    common::{B2Op, ToB2, binop::BinOp, postfix::Postfix, primitive::Primitive, uniop::UniOp},
    lexer::utils::{
        B2LexError, B2LexResult, Span,
        consts::{
            ADD_KW, AND_KW, DIV_KW, ENUM_INDEXING, EQ_KW, FUNCTION_CALL_DELIMITER,
            FUNCTION_CALL_END, FUNCTION_CALL_START, FUNCTION_PARAMETERS_DELIMITER,
            FUNCTION_PARAMETERS_END, FUNCTION_PARAMETERS_START, GEQ_KW, GROUP_END, GROUP_START,
            GT_KW, LEQ_KW, LIST_DELIMITER, LIST_END, LIST_START, LT_KW, MOD_KW, MUL_KW, NEQ_KW,
            NIL_VAL_KW, OR_KW, POW_KW, STRUCT_END_KW, STRUCT_FIELD_ACCESS_KW,
            STRUCT_FIELD_ASSIGNMENT, STRUCT_FIELD_END, STRUCT_FIELD_IMPL_KW, STRUCT_KW,
            STRUCT_START_KW, SUB_KW, TUPLE_DELIMITER, TUPLE_END, TUPLE_START,
            WHEN_STATEMENT_CONDITION_END_KW,
        },
        helper_parsers::{parse_identifier, parse_poly_list_with},
    },
};
use anyhow::Result;
use nom::{
    Parser,
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{multispace0, space0},
    combinator::opt,
    error::{ErrorKind, FromExternalError, context},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};
use nom_language::precedence::{Assoc, binary_op, precedence, unary_op};

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexExpr<'a> {
    #[default]
    Nil,
    Literal(Primitive<'a>),
    Tuple(Box<Self>, Box<Self>),
    List(Vec<Self>),
    Variable(&'a str),
    Group(Box<Self>),
    FunctionCall {
        identifier: &'a str,
        arguments: Vec<Self>,
    },
    Op(Box<B2Op<'a>>),
    Struct {
        identifier: &'a str,
        field_implementations: Vec<(&'a str, Self)>,
    },
    StructFieldAccessing {
        identifier: &'a str,
        field: &'a str,
    },
    Enum {
        identifier: &'a str,
        instance: &'a str,
        values: Vec<Self>,
    },
}

impl<'a> LexExpr<'a> {
    pub fn parse_expr(input: Span<'a>) -> B2LexResult<'a, Self> {
        alt((
            Self::parse_enum,
            Self::parse_struct_field_accessing,
            Self::parse_precedence,
            Self::parse_literal,
            Self::parse_function_call,
            Self::parse_group,
            Self::parse_tuple,
            Self::parse_list,
            Self::parse_struct,
            Self::parse_variable,
        ))
        .parse(input)
    }

    pub fn parse_precedence(i: Span<'a>) -> B2LexResult<'a, Self> {
        let expr_parser = alt((
            LexExpr::parse_literal,
            LexExpr::parse_function_call,
            LexExpr::parse_group,
            LexExpr::parse_tuple,
            LexExpr::parse_list,
            LexExpr::parse_struct,
            LexExpr::parse_variable,
        ));

        context(
            "precedence",
            precedence(
                preceded(
                    multispace0,
                    unary_op(1, UniOp::parse_unary_operation_symbol),
                ),
                preceded(multispace0, unary_op(1, Postfix::parse_postfix)),
                preceded(
                    multispace0,
                    alt((
                        binary_op(3, Assoc::Left, tag(ADD_KW).map(|_| BinOp::Add)),
                        binary_op(2, Assoc::Left, tag(MUL_KW).map(|_| BinOp::Mul)),
                        binary_op(3, Assoc::Left, tag(SUB_KW).map(|_| BinOp::Sub)),
                        binary_op(2, Assoc::Left, tag(DIV_KW).map(|_| BinOp::Div)),
                        binary_op(1, Assoc::Left, tag(POW_KW).map(|_| BinOp::Pow)),
                        binary_op(4, Assoc::Left, tag(EQ_KW).map(|_| BinOp::Eq)),
                        binary_op(4, Assoc::Left, tag(GEQ_KW).map(|_| BinOp::Geq)),
                        binary_op(4, Assoc::Left, tag(LEQ_KW).map(|_| BinOp::Leq)),
                        binary_op(4, Assoc::Left, tag(GT_KW).map(|_| BinOp::Gt)),
                        binary_op(4, Assoc::Left, tag(LT_KW).map(|_| BinOp::Lt)),
                        binary_op(4, Assoc::Left, tag(NEQ_KW).map(|_| BinOp::Neq)),
                        binary_op(3, Assoc::Left, tag(MOD_KW).map(|_| BinOp::Mod)),
                        binary_op(5, Assoc::Left, tag(AND_KW).map(|_| BinOp::And)),
                        binary_op(5, Assoc::Left, tag(OR_KW).map(|_| BinOp::Or)),
                    )),
                ),
                preceded(multispace0, expr_parser),
                |op| -> Result<Self> { Ok(Self::Op(Box::new(op.into()))) },
            ),
        )
        .parse(i)
    }

    pub fn parse_literal(input: Span<'a>) -> B2LexResult<'a, Self> {
        Primitive::parse_primitive
            .map(|p| Self::Literal(p))
            .parse(input)
    }

    pub fn parse_tuple(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_list(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_variable(input: Span<'a>) -> B2LexResult<'a, Self> {
        match parse_identifier.parse(input)? {
            // TODO: Figure out a better way to not allow keywords as identifiers
            (_, ident)
                if matches!(
                    ident.to_string().as_str(),
                    STRUCT_KW | WHEN_STATEMENT_CONDITION_END_KW
                ) =>
            {
                Err(nom::Err::Error(B2LexError::from_external_error(
                    input,
                    ErrorKind::Fail,
                    "not valid identifier",
                )))
            }
            (rem, ident) => Ok((rem, Self::Variable(ident))),
        }
    }

    pub fn parse_group(input: Span<'a>) -> B2LexResult<'a, Self> {
        let (rem, expr) =
            delimited(tag(GROUP_START), Self::parse_expr, tag(GROUP_END)).parse(input)?;
        Ok((rem, Self::Group(Box::new(expr))))
    }

    pub fn parse_function_call(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "parse-function-call",
            (
                parse_identifier,
                context(
                    "function-arguments",
                    alt((
                        tag(FUNCTION_CALL_START)
                            .and(multispace0.and(tag(FUNCTION_CALL_END)))
                            .map(|_| Vec::new()),
                        parse_poly_list_with(
                            FUNCTION_CALL_START,
                            FUNCTION_CALL_DELIMITER,
                            FUNCTION_CALL_END,
                            Self::parse_expr,
                        ),
                        delimited(
                            tag(FUNCTION_CALL_START).and(multispace0),
                            Self::parse_expr,
                            multispace0.and(tag(FUNCTION_CALL_END)),
                        )
                        .map(|x| vec![x]),
                    )),
                ),
            ),
        )
        .map(|(ident, arguments)| Self::FunctionCall {
            identifier: &ident,
            arguments,
        })
        .parse(input)
    }

    pub fn parse_struct(input: Span<'a>) -> B2LexResult<'a, Self> {
        let (i, ident) =
            preceded(tag(STRUCT_KW), preceded(space0, parse_identifier)).parse(input)?;
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
                identifier: &ident,
                field_implementations,
            },
        ))
    }

    pub fn parse_struct_field_accessing(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "struct-field-accessing",
            (
                parse_identifier,
                tag(STRUCT_FIELD_ACCESS_KW),
                parse_identifier,
            ),
        )
        .map(|(identifier, _, field)| Self::StructFieldAccessing { identifier, field })
        .parse(input)
    }

    pub fn parse_struct_field(input: Span<'a>) -> B2LexResult<'a, (&'a str, Self)> {
        pair(
            pair(
                preceded(
                    tag(STRUCT_FIELD_IMPL_KW),
                    preceded(space0, parse_identifier),
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

    pub fn parse_enum(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "enum",
            (
                parse_identifier,
                tag(ENUM_INDEXING),
                parse_identifier,
                opt(parse_poly_list_with(
                    FUNCTION_PARAMETERS_START,
                    FUNCTION_PARAMETERS_DELIMITER,
                    FUNCTION_PARAMETERS_END,
                    Self::parse_expr,
                ))
                .map(|x| x.unwrap_or_default()),
            ),
        )
        .map(|(identifier, _, instance, values)| Self::Enum {
            identifier,
            instance,
            values,
        })
        .parse(input)
    }
}

impl<'a> ToB2 for LexExpr<'a> {
    fn to_b2(&self) -> String {
        match self {
            LexExpr::Literal(primitive) => primitive.to_b2(),
            LexExpr::Tuple(fst, snd) => format!(
                "{}{}{}{}{}",
                TUPLE_START,
                fst.to_b2(),
                TUPLE_DELIMITER,
                snd.to_b2(),
                TUPLE_END
            ),
            LexExpr::List(xs) => format!(
                "{}{}{}",
                LIST_START,
                xs.iter()
                    .map(|x| x.to_b2())
                    .collect::<Vec<_>>()
                    .join(LIST_DELIMITER),
                LIST_END
            ),
            LexExpr::Variable(xs) => xs.to_string(),
            LexExpr::Group(x) => format!("{}{}{}", GROUP_START, x.to_b2(), GROUP_END),
            LexExpr::FunctionCall {
                identifier,
                arguments,
            } => format!(
                "{}{}{}{}",
                identifier,
                FUNCTION_CALL_START,
                arguments
                    .iter()
                    .map(|a| a.to_b2())
                    .collect::<Vec<_>>()
                    .join(FUNCTION_CALL_DELIMITER),
                FUNCTION_CALL_END
            ),
            LexExpr::Op(op) => op.to_b2(),
            LexExpr::Struct {
                identifier,
                field_implementations,
            } => format!(
                "{STRUCT_KW} {identifier} {STRUCT_START_KW} {} {STRUCT_END_KW}",
                field_implementations
                    .iter()
                    .map(|(f, v)| format!("{STRUCT_FIELD_IMPL_KW} {f} {}", v.to_b2()))
                    .collect::<Vec<_>>()
                    .join(STRUCT_FIELD_END)
            ),
            LexExpr::StructFieldAccessing { identifier, field } => {
                format!("{identifier}{STRUCT_FIELD_ACCESS_KW}{field}")
            }
            LexExpr::Enum {
                identifier,
                instance,
                values,
            } => {
                if values.is_empty() {
                    format!("{identifier}{ENUM_INDEXING}{instance}")
                } else {
                    format!(
                        "{identifier}{ENUM_INDEXING}{instance}{FUNCTION_PARAMETERS_START}{}{FUNCTION_PARAMETERS_END}",
                        values
                            .into_iter()
                            .map(|e| e.to_b2())
                            .collect::<Vec<_>>()
                            .join(FUNCTION_PARAMETERS_DELIMITER)
                    )
                }
            }
            LexExpr::Nil => NIL_VAL_KW.to_string(),
        }
    }
}

impl<'a> From<&'a str> for LexExpr<'a> {
    fn from(value: &'a str) -> Self {
        LexExpr::Literal(value.into())
    }
}

impl<'a> From<i32> for LexExpr<'a> {
    fn from(value: i32) -> Self {
        LexExpr::Literal(value.into())
    }
}

impl<'a> From<bool> for LexExpr<'a> {
    fn from(value: bool) -> Self {
        LexExpr::Literal(value.into())
    }
}

impl<'a, T: Into<LexExpr<'a>>> From<Vec<T>> for LexExpr<'a> {
    fn from(value: Vec<T>) -> Self {
        Self::List(value.into_iter().map(|i| i.into()).collect())
    }
}

impl<'a, F: Into<LexExpr<'a>>, S: Into<LexExpr<'a>>> From<(F, S)> for LexExpr<'a> {
    fn from((f, s): (F, S)) -> Self {
        Self::Tuple(Box::new(f.into()), Box::new(s.into()))
    }
}
