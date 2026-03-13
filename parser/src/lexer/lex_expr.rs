use crate::{
    common::{B2Op, binop::BinOp, postfix::Postfix, primitive::Primitive, uniop::UniOp},
    lexer::utils::{
        B2Error, B2Result, Span,
        consts::{
            ADD_KW, AND_KW, DIV_KW, ENUM_INDEXING, EQ_KW, FUNCTION_CALL_DELIMITER, FUNCTION_CALL_END, FUNCTION_CALL_START, GEQ_KW, GROUP_END, GROUP_START, GT_KW, LEQ_KW, LIST_DELIMITER, LIST_END, LIST_START, LT_KW, MOD_KW, MUL_KW, NEQ_KW, OR_KW, POW_KW, STRUCT_END_KW, STRUCT_FIELD_ACCESS_KW, STRUCT_FIELD_ASSIGNMENT, STRUCT_FIELD_END, STRUCT_FIELD_IMPL_KW, STRUCT_KW, STRUCT_START_KW, SUB_KW, TUPLE_DELIMITER, TUPLE_END, TUPLE_START
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
    error::{ErrorKind, FromExternalError, context},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};
use nom_language::precedence::{Assoc, binary_op, precedence, unary_op};

pub enum LexExpr<'a> {
    Literal(Primitive),
    Tuple(Box<Self>, Box<Self>),
    List(Vec<Self>),
    Variable(Span<'a>),
    Group(Box<Self>),
    FunctionCall {
        identifier: Span<'a>,
        arguments: Vec<Self>,
    },
    Op(Box<B2Op<'a>>),
    Struct {
        identifier: Span<'a>,
        field_implementations: Vec<(Span<'a>, Self)>,
    },
    StructFieldAccessing {
        identifier: Span<'a>,
        field: Span<'a>,
    },
    Enum {
        identifier: Span<'a>,
        instance: Span<'a>,
    }
}

impl<'a> LexExpr<'a> {
    pub fn parse_expr(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_precedence(i: Span<'a>) -> B2Result<'a, Self> {
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
                |op| -> Result<Self> { Ok(Self::Op(Box::new(op))) },
            ),
        )
        .parse(i)
    }

    pub fn parse_expr_excl_postfix(input: Span<'a>) -> B2Result<'a, Self> {
        alt((
            Self::parse_binary_operation,
            Self::parse_literal,
            Self::parse_function_call,
            Self::parse_group,
            Self::parse_tuple,
            Self::parse_list,
            Self::parse_struct,
            Self::parse_variable,
            Self::parse_unary_operation,
        ))
        .parse(input)
    }

    fn parse_expr_no_inf_rec(input: Span<'a>) -> B2Result<'a, Self> {
        alt((
            Self::parse_literal,
            Self::parse_function_call,
            Self::parse_group,
            Self::parse_tuple,
            Self::parse_list,
            Self::parse_struct,
            Self::parse_variable,
            Self::parse_unary_operation,
        ))
        .parse(input)
    }

    pub fn parse_literal(input: Span) -> B2Result<Self> {
        Primitive::parse_primitive(input).map(|(rem, p)| (rem, Self::Literal(p)))
    }

    pub fn parse_tuple(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_list(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_variable(input: Span<'a>) -> B2Result<'a, Self> {
        match parse_identifier.map(|s| Self::Variable(s)).parse(input)? {
            // TODO: Figure out a better way to not allow keywords as identifiers
            (_, LexExpr::Variable(ident)) if ident == Span::new(STRUCT_KW) => Err(nom::Err::Error(
                B2Error::from_external_error(input, ErrorKind::Fail, "not valid identifier"),
            )),
            res @ (_, _) => Ok(res),
        }
    }

    pub fn parse_group(input: Span<'a>) -> B2Result<'a, Self> {
        let (rem, expr) =
            delimited(tag(GROUP_START), Self::parse_expr, tag(GROUP_END)).parse(input)?;
        Ok((rem, Self::Group(Box::new(expr))))
    }

    pub fn parse_function_call(input: Span<'a>) -> B2Result<'a, Self> {
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
        .map(|(identifier, arguments)| Self::FunctionCall {
            identifier,
            arguments,
        })
        .parse(input)
    }

    pub fn parse_postfix_operation(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "postfix-expresion",
            pair(Self::parse_expr_no_inf_rec, many1(Postfix::parse_postfix)),
        )
        .map(|(val, ops)| {
            ops.into_iter()
                .fold(val, |acc, op| Self::Op(Box::new(B2Op::Postfix(acc, op))))
        })
        .parse(input)
    }

    pub fn parse_unary_operation(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "unary-operation",
            pair(many1(UniOp::parse_unary_operation_symbol), Self::parse_expr),
        )
        .map(|(ops, val)| Self::fold_unary(ops, val))
        .parse(input)
    }

    fn fold_unary(ops: Vec<UniOp>, expr: LexExpr<'a>) -> Self {
        if ops.is_empty() {
            expr
        } else {
            ops.into_iter()
                .fold(expr, |acc, op| Self::Op(Box::new(B2Op::Prefix(op, acc))))
        }
    }

    pub fn parse_binary_operation(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "binary-operation",
            (
                context("left-operand", Self::parse_expr_no_inf_rec),
                preceded(
                    multispace0,
                    context("binary-operation-symbol", BinOp::parse_symbol),
                ),
                preceded(multispace0, context("right-operand", Self::parse_expr)),
            ),
        )
        .map(|(l, op, r)| Self::Op(Box::new(B2Op::Binary(l, op, r))))
        .parse(input)
    }

    pub fn parse_struct(input: Span<'a>) -> B2Result<'a, Self> {
        let (i, identifier) =
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
                identifier,
                field_implementations,
            },
        ))
    }

    pub fn parse_struct_field_accessing(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_struct_field(input: Span<'a>) -> B2Result<'a, (Span<'a>, Self)> {
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

    pub fn parse_enum(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "enum",
            (parse_identifier, tag(ENUM_INDEXING), parse_identifier),
        )
        .map(|(identifier, _ , instance)| Self::Enum { identifier, instance })
        .parse(input)
    }
}

impl<'a> std::fmt::Debug for LexExpr<'a> {
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
                B2Op::Prefix(op, val) => f
                    .debug_tuple("Operation::Prefix")
                    .field(&op)
                    .field(&val)
                    .finish(),
                B2Op::Postfix(val, op) => f
                    .debug_tuple("Operation::Postfix")
                    .field(&val)
                    .field(&op)
                    .finish(),
                B2Op::Binary(l, op, r) => f
                    .debug_tuple("Operation::Binary")
                    .field(&l)
                    .field(&op)
                    .field(&r)
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
            Self::StructFieldAccessing { identifier, field } => f
                .debug_struct("StructFieldAccessing")
                .field("identifier", identifier)
                .field("field", field)
                .finish(),
            Self::Enum { identifier, instance } => f
                .debug_struct("Enum")
                .field("identifier", identifier)
                .field("instance", instance)
                .finish(),
        }
    }
}

impl<'a> PartialEq for LexExpr<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Tuple(l0, l1), Self::Tuple(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Variable(l0), Self::Variable(r0)) => l0.to_string() == r0.to_string(),
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
            ) => l_identifier.to_string() == r_identifier.to_string() && l_arguments == r_arguments,
            (Self::Op(l0), Self::Op(r0)) => match (&**l0, &**r0) {
                (B2Op::Prefix(lop, lval), B2Op::Prefix(rop, rval)) => lop == rop && lval == rval,
                (B2Op::Postfix(lval, lop), B2Op::Postfix(rval, rop)) => lop == rop && lval == rval,
                (B2Op::Binary(ll, lop, lr), B2Op::Binary(rl, rop, rr)) => {
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
            ) => {
                l_identifier.to_string() == r_identifier.to_string()
                    && l_field_implementations
                        .iter()
                        .map(|(s, e)| (s.to_string(), e))
                        .collect::<Vec<_>>()
                        == r_field_implementations
                            .iter()
                            .map(|(s, e)| (s.to_string(), e))
                            .collect::<Vec<_>>()
            }
            (
                Self::StructFieldAccessing {
                    identifier: li,
                    field: lf,
                },
                Self::StructFieldAccessing {
                    identifier: ri,
                    field: rf,
                },
            ) => li.to_string() == ri.to_string() && lf.to_string() == rf.to_string(),
            _ => false,
        }
    }
}

impl<'a> From<&str> for LexExpr<'a> {
    fn from(value: &str) -> Self {
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
