use crate::lexer::{
    lex_expr::LexExpr,
    lex_stmt::LexStmt,
    lex_type::LexType,
    utils::{
        B2LexResult, Span,
        consts::{
            LIST_DELIMITER, LIST_END, LIST_START, LIST_UNPACKING_KW, STRUCT_FIELD_ACCESS_KW,
            WHEN_STATEMENT_BRANCH_END, WHEN_STATEMENT_CONDITION_END_KW,
            WHEN_STATEMENT_CONDITION_START_KW, WHEN_STATEMENT_TYPE_START_KW,
        },
        helper_parsers::{parse_identifier, parse_poly_list_with, parse_statements},
    },
};
use nom::{
    Parser,
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::opt,
    error::context,
    multi::separated_list0,
    sequence::{delimited, preceded, terminated},
};

#[derive(Debug, PartialEq, Clone)]
pub enum WhenMatch<'a> {
    /// [ ]
    EmptyList { condition: Option<LexExpr<'a>> },
    /// [x]
    Singleton {
        identifier: &'a str,
        condition: Option<LexExpr<'a>>,
    },
    /// [a, b, ...xs]
    VariadicList {
        identifiers: Vec<&'a str>,
        remainder: Option<&'a str>,
        condition: Option<LexExpr<'a>>,
    },
    /// x
    CatchAll {
        identifier: &'a str,
        condition: Option<LexExpr<'a>>,
    },
    /// IS INT
    /// Matches if the value is assignable to the specified type
    Type {
        b2_type: LexType<'a>,
        condition: Option<LexExpr<'a>>,
    },
    // [::field_a, ::field_b] AND field_a < field_b FOLLOWS ...
    StructField {
        fields: Vec<&'a str>,
        condition: Option<LexExpr<'a>>,
    },
}

type WMResult<'a> = B2LexResult<'a, (WhenMatch<'a>, Vec<LexStmt<'a>>)>;

impl<'a> WhenMatch<'a> {
    pub fn parse(input: Span<'a>) -> WMResult<'a> {
        context(
            "when-match",
            alt((
                Self::parse_empty_list,
                Self::parse_singleton,
                Self::parse_variadic_list,
                Self::parse_catch_all,
                Self::parse_type,
                Self::parse_struct,
            )),
        )
        .parse(input)
    }

    fn parse_condition_and_stmt(
        input: Span<'a>,
    ) -> B2LexResult<'a, (Option<LexExpr<'a>>, Vec<LexStmt<'a>>)> {
        context(
            "condition-and-statements",
            (
                delimited(
                    opt((
                        multispace0,
                        tag(WHEN_STATEMENT_CONDITION_START_KW),
                        multispace0,
                    )),
                    context(
                        "optional-condition",
                        opt(preceded(multispace0, LexExpr::parse_expr)),
                    ),
                    context(
                        "multispace-follows-multispace",
                        (multispace0, tag(WHEN_STATEMENT_CONDITION_END_KW)),
                    ),
                ),
                parse_statements,
            ),
        )
        .parse(input)
    }

    pub fn parse_empty_list(input: Span<'a>) -> WMResult<'a> {
        context(
            "empty-list-branch",
            delimited(
                (tag(LIST_START), multispace0, tag(LIST_END)),
                Self::parse_condition_and_stmt,
                (multispace0, tag(WHEN_STATEMENT_BRANCH_END)),
            ),
        )
        .map(|(condition, stmts)| (Self::EmptyList { condition }, stmts))
        .parse(input)
    }

    pub fn parse_singleton(input: Span<'a>) -> WMResult<'a> {
        context(
            "singleton-branch",
            terminated(
                (
                    delimited(
                        (tag(LIST_START), multispace0),
                        parse_identifier,
                        (multispace0, tag(LIST_END)),
                    ),
                    Self::parse_condition_and_stmt,
                ),
                (multispace0, tag(WHEN_STATEMENT_BRANCH_END)),
            ),
        )
        .map(|(identifier, (condition, stmts))| {
            (
                Self::Singleton {
                    identifier,
                    condition,
                },
                stmts,
            )
        })
        .parse(input)
    }

    pub fn parse_variadic_list(input: Span<'a>) -> WMResult<'a> {
        context(
            "variadic-list-branch",
            terminated(
                (
                    (
                        context(
                            "single-variables",
                            preceded(
                                tag(LIST_START),
                                separated_list0(
                                    permutation((multispace0, tag(LIST_DELIMITER), multispace0)),
                                    context("variadic-variables", parse_identifier),
                                ),
                            ),
                        ),
                        context(
                            "optional-variadic-variable",
                            terminated(
                                opt(preceded(
                                    (
                                        multispace0,
                                        tag(LIST_DELIMITER),
                                        multispace0,
                                        tag(LIST_UNPACKING_KW),
                                    ),
                                    parse_identifier,
                                )),
                                (multispace0, tag(LIST_END)),
                            ),
                        ),
                    ),
                    Self::parse_condition_and_stmt,
                ),
                (multispace0, tag(WHEN_STATEMENT_BRANCH_END)),
            ),
        )
        .map(|((identifiers, remainder), (condition, stmts))| {
            (
                Self::VariadicList {
                    identifiers,
                    remainder,
                    condition,
                },
                stmts,
            )
        })
        .parse(input)
    }

    pub fn parse_catch_all(input: Span<'a>) -> WMResult<'a> {
        context(
            "catch-all-branch",
            terminated(
                (
                    preceded(multispace0, parse_identifier),
                    Self::parse_condition_and_stmt,
                ),
                (multispace0, tag(WHEN_STATEMENT_BRANCH_END)),
            ),
        )
        .map(|(identifier, (condition, stmts))| {
            (
                Self::CatchAll {
                    identifier,
                    condition,
                },
                stmts,
            )
        })
        .parse(input)
    }

    pub fn parse_type(input: Span<'a>) -> WMResult<'a> {
        context(
            "type-branch",
            terminated(
                (
                    preceded(
                        (multispace0, tag(WHEN_STATEMENT_TYPE_START_KW), multispace0),
                        LexType::parse_type,
                    ),
                    Self::parse_condition_and_stmt,
                ),
                (multispace0, tag(WHEN_STATEMENT_BRANCH_END)),
            ),
        )
        .map(|(b2_type, (condition, stmts))| (Self::Type { b2_type, condition }, stmts))
        .parse(input)
    }

    pub fn parse_struct(input: Span<'a>) -> WMResult<'a> {
        context(
            "struct-branch",
            terminated(
                (
                    delimited(
                        multispace0,
                        parse_poly_list_with(
                            LIST_START,
                            LIST_DELIMITER,
                            LIST_END,
                            preceded(tag(STRUCT_FIELD_ACCESS_KW), parse_identifier),
                        ),
                        (tag(WHEN_STATEMENT_CONDITION_END_KW), multispace0),
                    ),
                    Self::parse_condition_and_stmt,
                ),
                (multispace0, tag(WHEN_STATEMENT_BRANCH_END)),
            ),
        )
        .map(|(fields, (condition, stmts))| (Self::StructField { fields, condition }, stmts))
        .parse(input)
    }
}
