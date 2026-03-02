use crate::{
    common::{B2Op, binop::BinOp, postfix::Postfix, primitive::Primitive, uniop::UniOp},
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
use nom_language::{error::VerboseError, precedence::Operation};

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
    Op(Box<B2Op>),
    Struct {
        identifier: String,
        field_implementations: Vec<(String, Self)>,
    },
}

impl LexExpr {
    pub fn parse_expr(input: Span) -> B2Result<Self> {
        alt((
            Self::parse_postfix_operation,
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

    pub fn parse_expr_excl_postfix(input: Span) -> B2Result<Self> {
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

    pub fn parse_postfix_operation(input: Span) -> B2Result<Self> {
        context(
            "postfix-expresion",
            pair(Self::parse_expr_excl_postfix, Postfix::parse_postfix),
        )
        .map(|(val, op)| Self::Op(Box::new(Operation::Postfix(val, op))))
        .parse(input)
    }

    pub fn parse_unary_operation(input: Span) -> B2Result<Self> {
        context(
            "unary-operation",
            pair(UniOp::parse_unary_operation_symbol, Self::parse_expr),
        )
        .map(|(op, val)| Self::Op(Box::new(Operation::Prefix(op, val))))
        .parse(input)
    }

    pub fn parse_binary_operation(input: Span) -> B2Result<Self> {
        context(
            "binary-operation",
            (
                context("left-operand", Self::parse_expr_excl_binary),
                preceded(
                    multispace0,
                    context("binary-operation-symbol", BinOp::parse_symbol),
                ),
                preceded(multispace0, context("right-operand", Self::parse_expr)),
            ),
        )
        .map(|(l, op, r)| Self::Op(Box::new(Operation::Binary(l, op, r))))
        .parse(input)
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

impl std::fmt::Debug for LexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            Self::Tuple(arg0, arg1) => f.debug_tuple("Tuple").field(arg0).field(arg1).finish(),
            Self::List(arg0) => f.debug_tuple("List").field(arg0).finish(),
            Self::Variable(arg0) => f.debug_tuple("Variable").field(arg0).finish(),
            Self::Group(arg0) => f.debug_tuple("Group").field(arg0).finish(),
            Self::FunctionCall {
                identifier,
                arguments,
            } => f
                .debug_struct("FunctionCall")
                .field("identifier", identifier)
                .field("arguments", arguments)
                .finish(),
            Self::Op(arg0) => match &**arg0 {
                Operation::Prefix(op, val) => f
                    .debug_tuple("Operation::Prefix")
                    .field(op)
                    .field(val)
                    .finish(),
                Operation::Postfix(val, op) => f
                    .debug_tuple("Operation::Postfix")
                    .field(val)
                    .field(op)
                    .finish(),
                Operation::Binary(l, op, r) => f
                    .debug_tuple("Operation::Binary")
                    .field(l)
                    .field(op)
                    .field(r)
                    .finish(),
            },
            Self::Struct {
                identifier,
                field_implementations,
            } => f
                .debug_struct("Struct")
                .field("identifier", identifier)
                .field("field_implementations", field_implementations)
                .finish(),
        }
    }
}

impl PartialEq for LexExpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Tuple(l0, l1), Self::Tuple(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Variable(l0), Self::Variable(r0)) => l0 == r0,
            (Self::Group(l0), Self::Group(r0)) => l0 == r0,
            (
                Self::FunctionCall {
                    identifier: l_identifier,
                    arguments: l_arguments,
                },
                Self::FunctionCall {
                    identifier: r_identifier,
                    arguments: r_arguments,
                },
            ) => l_identifier == r_identifier && l_arguments == r_arguments,
            (Self::Op(l0), Self::Op(r0)) => match (&**l0, &**r0) {
                (Operation::Prefix(lop, lval), Operation::Prefix(rop, rval)) => {
                    lop == rop && lval == rval
                }
                (Operation::Postfix(lval, lop), Operation::Postfix(rval, rop)) => {
                    lop == rop && lval == rval
                }
                (Operation::Binary(ll, lop, lr), Operation::Binary(rl, rop, rr)) => {
                    lop == rop && ll == rl && lr == rr
                }
                _ => false,
            },
            (
                Self::Struct {
                    identifier: l_identifier,
                    field_implementations: l_field_implementations,
                },
                Self::Struct {
                    identifier: r_identifier,
                    field_implementations: r_field_implementations,
                },
            ) => l_identifier == r_identifier && l_field_implementations == r_field_implementations,
            _ => false,
        }
    }
}
