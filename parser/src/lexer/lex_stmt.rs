use crate::{
    common::binop::BinOp,
    lexer::{
        lex_expr::LexExpr,
        lex_type::LexType,
        utils::{
            B2Result, Span,
            consts::{
                ASSIGNMENT_KW, BLOCK_STATEMENT_END_KW, BLOCK_STATEMENT_START_KW, BREAK_STMT_KW,
                END_STMT_KW, FUNCTION_BODY_END_KW, FUNCTION_DECLARATION_KW,
                FUNCTION_IMPLEMENTATION_KW, FUNCTION_IMPLEMENTATION_START_KW,
                FUNCTION_INVOCATION_END, FUNCTION_INVOCATION_START_KW,
                FUNCTION_PARAMETERS_DELIMITER, FUNCTION_PARAMETERS_END, FUNCTION_PARAMETERS_START,
                IF_STATEMENT_BODY_START_KW, IF_STATEMENT_END_KW, IF_STATEMENT_START_KW,
                IMPORT_MODULE_KW, RETURN_STMT_KW, STRUCT_DECL_KW, STRUCT_END_KW,
                STRUCT_FIELD_DECL_KW, STRUCT_KW, TYPE_ALIAS_KW, VARIABLE_REASIGNMENT,
                WHILE_STATEMENT_BODY_START_KW, WHILE_STATEMENT_END_KW, WHILE_STATEMENT_START_KW,
            },
            helper_parsers::{
                parse_comment, parse_identifier, parse_parameters, parse_poly_list_with,
                parse_statements,
            },
        },
    },
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{multispace0, multispace1, space0, space1},
    combinator::opt,
    error::context,
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
};

const VARIABLE_DECLARATION: &str = "LET";

#[derive(Debug, PartialEq)]
pub enum LexStmt {
    VariableDeclaration {
        identifier: String,
        variable_type: Option<LexType>,
    },
    VariableDeclarationAssignment {
        identifier: String,
        variable_type: Option<LexType>,
        value: LexExpr,
    },
    VariableReassignment {
        identifier: String,
        reassignment: Option<BinOp>,
        new_value: LexExpr,
    },
    If {
        condition: LexExpr,
        body: Vec<Self>,
    },
    While {
        condition: LexExpr,
        body: Vec<Self>,
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<LexType>,
        return_type: Option<LexType>,
    },
    FunctionImplementation {
        identifier: String,
        parameters: Vec<(String, Option<LexExpr>)>,
        body: Vec<Self>,
    },
    StructDeclaration {
        identifier: String,
        fields: Vec<(String, LexType)>,
    },
    Block {
        body: Vec<Self>,
    },
    FunctionInvocation {
        identifier: String,
        arguments: Vec<LexExpr>,
    },
    Break,
    Return {
        value: Option<LexExpr>,
    },
    TypeAlias {
        identifier: String,
        b2_type: LexType,
    },
    ImportModule {
        identifier: String,
    },
}

impl LexStmt {
    pub fn parse_statement(input: Span) -> B2Result<Self> {
        context(
            "statements",
            alt((
                Self::parse_type_alias,
                Self::parse_variable_declaration,
                Self::parse_variable_declaration_assignment,
                Self::parse_variable_reassignment,
                Self::parse_if_statement,
                Self::parse_while_statement,
                Self::parse_function_declaration,
                Self::parse_function_implementation,
                Self::parse_block_statement,
                Self::parse_function_invocation,
                Self::parse_struct_declaration,
                Self::parse_break,
                Self::parse_return,
                Self::parse_import_module,
            )),
        )
        .parse(input)
    }

    pub fn parse_break(input: Span) -> B2Result<Self> {
        delimited(multispace0, tag(BREAK_STMT_KW), tag(END_STMT_KW))
            .map(|_| Self::Break)
            .parse(input)
    }

    pub fn parse_return(input: Span) -> B2Result<Self> {
        delimited(
            preceded(multispace0, tag(RETURN_STMT_KW)),
            opt(preceded(space0, LexExpr::parse_expr)),
            tag(END_STMT_KW),
        )
        .map(|value| Self::Return { value })
        .parse(input)
    }

    pub fn parse_variable_declaration(input: Span) -> B2Result<Self> {
        delimited(
            preceded(multispace0, tag(VARIABLE_DECLARATION)),
            (
                preceded(multispace0, parse_identifier.map(|s| s.to_string())),
                preceded(multispace0, opt(LexType::parse_type)),
            ),
            tag(END_STMT_KW),
        )
        .map(|(identifier, variable_type)| Self::VariableDeclaration {
            identifier,
            variable_type,
        })
        .parse(input)
    }

    pub fn parse_variable_declaration_assignment(input: Span) -> B2Result<Self> {
        delimited(
            preceded(multispace0, tag(VARIABLE_DECLARATION)),
            (
                preceded(multispace0, parse_identifier.map(|s| s.to_string())),
                preceded(multispace0, opt(LexType::parse_type)),
                preceded(
                    (multispace0, tag(VARIABLE_REASIGNMENT)),
                    preceded(multispace0, LexExpr::parse_expr),
                ),
            ),
            tag(END_STMT_KW),
        )
        .map(
            |(identifier, variable_type, value)| Self::VariableDeclarationAssignment {
                identifier,
                variable_type,
                value,
            },
        )
        .parse(input)
    }

    pub fn parse_variable_reassignment(input: Span) -> B2Result<Self> {
        context(
            "parse-variable-reassignment",
            preceded(
                multispace0,
                terminated(
                    (
                        parse_identifier,
                        terminated(
                            preceded(space0, opt(BinOp::parse_symbol)),
                            tag(VARIABLE_REASIGNMENT),
                        ),
                        preceded(space0, LexExpr::parse_expr),
                    ),
                    tag(END_STMT_KW),
                ),
            ),
        )
        .map(
            |(ident, reassignment, new_value)| Self::VariableReassignment {
                identifier: ident.to_string(),
                reassignment,
                new_value,
            },
        )
        .parse(input)
    }

    pub fn parse_if_statement(input: Span) -> B2Result<Self> {
        let (i, _whitespace) = multispace0.parse(input)?;
        let (i, condition) = preceded(
            tag(IF_STATEMENT_START_KW),
            delimited(space0, LexExpr::parse_expr, space0),
        )
        .parse(i)?;
        let (i, _then_kw) = preceded(multispace0, tag(IF_STATEMENT_BODY_START_KW)).parse(i)?;
        let (i, body) = many0(preceded(multispace0, Self::parse_statement)).parse(i)?;
        let (rem, _end_kw) = preceded(multispace0, tag(IF_STATEMENT_END_KW)).parse(i)?;
        Ok((rem, Self::If { condition, body }))
    }

    pub fn parse_while_statement(input: Span) -> B2Result<Self> {
        let (i, _whitespace) = multispace0.parse(input)?;
        let (i, condition) = preceded(
            tag(WHILE_STATEMENT_START_KW),
            delimited(space0, LexExpr::parse_expr, space0),
        )
        .parse(i)?;
        let (i, _do_kw) = preceded(multispace0, tag(WHILE_STATEMENT_BODY_START_KW)).parse(i)?;
        let (i, body) = many0(preceded(multispace0, Self::parse_statement)).parse(i)?;
        let (rem, _end_kw) = preceded(multispace0, tag(WHILE_STATEMENT_END_KW)).parse(i)?;
        Ok((rem, Self::While { condition, body }))
    }

    pub fn parse_function_declaration(input: Span) -> B2Result<Self> {
        context(
            "function-declaration",
            delimited(
                tag(FUNCTION_DECLARATION_KW).and(multispace0),
                (
                    parse_identifier.map(|s| s.to_string()),
                    parse_poly_list_with(
                        FUNCTION_PARAMETERS_START,
                        FUNCTION_PARAMETERS_DELIMITER,
                        FUNCTION_PARAMETERS_END,
                        LexType::parse_type,
                    ),
                    opt(preceded(
                        (multispace0, tag(":"), multispace0),
                        LexType::parse_type,
                    )),
                ),
                tag(END_STMT_KW),
            ),
        )
        .map(
            |(identifier, parameters, return_type)| Self::FunctionDeclaration {
                identifier,
                parameters,
                return_type,
            },
        )
        .parse(input)
    }

    pub fn parse_function_implementation(input: Span) -> B2Result<Self> {
        context(
            "function-implementation",
            delimited(
                tag(FUNCTION_IMPLEMENTATION_KW).and(multispace0),
                (
                    parse_identifier.map(|s| s.to_string()),
                    parse_poly_list_with(
                        FUNCTION_PARAMETERS_START,
                        FUNCTION_PARAMETERS_DELIMITER,
                        FUNCTION_PARAMETERS_END,
                        parse_parameters,
                    ),
                    preceded(
                        (
                            multispace0,
                            tag(FUNCTION_IMPLEMENTATION_START_KW),
                            multispace0,
                        ),
                        parse_statements,
                    ),
                ),
                (multispace0, tag(FUNCTION_BODY_END_KW)),
            ),
        )
        .map(
            |(identifier, parameters, body)| Self::FunctionImplementation {
                identifier,
                parameters,
                body,
            },
        )
        .parse(input)
    }

    pub fn parse_block_statement(input: Span) -> B2Result<Self> {
        context(
            "block-statement",
            delimited(
                context(
                    "block-statement-start",
                    multispace0.and(tag(BLOCK_STATEMENT_START_KW).and(multispace0)),
                ),
                context("block-inner-statements", parse_statements),
                context(
                    "block-statement-end",
                    many0(alt((multispace1, parse_comment))).and(tag(BLOCK_STATEMENT_END_KW)),
                ),
            )
            .map(|body| Self::Block { body }),
        )
        .parse(input)
    }

    pub fn parse_function_invocation(input: Span) -> B2Result<Self> {
        context(
            "function-invocation",
            delimited(
                tag(FUNCTION_INVOCATION_START_KW).and(multispace0),
                LexExpr::parse_function_call,
                tag(FUNCTION_INVOCATION_END),
            ),
        )
        .map(|function| match function {
            LexExpr::FunctionCall {
                identifier,
                arguments,
            } => Self::FunctionInvocation {
                identifier,
                arguments,
            },
            _ => unreachable!("parse_function_call should only return functioncall"),
        })
        .parse(input)
    }

    pub fn parse_struct_declaration(input: Span) -> B2Result<Self> {
        let (i, identifier) = preceded(
            tag(STRUCT_KW),
            preceded(space0, parse_identifier.map(|s| s.to_string())),
        )
        .parse(input)?;
        let (rem, fields) = terminated(
            preceded(
                multispace0,
                preceded(
                    tag(STRUCT_DECL_KW),
                    many0(preceded(multispace0, Self::parse_struct_field_statement)),
                ),
            ),
            preceded(multispace0, tag(STRUCT_END_KW)),
        )
        .parse(i)?;
        Ok((rem, Self::StructDeclaration { identifier, fields }))
    }

    pub fn parse_struct_field_statement(input: Span) -> B2Result<(String, LexType)> {
        pair(
            pair(
                preceded(
                    tag(STRUCT_FIELD_DECL_KW),
                    preceded(space0, parse_identifier.map(|s| s.to_string())),
                ),
                preceded(
                    space0,
                    preceded(tag(":"), preceded(space0, LexType::parse_type)),
                ),
            ),
            tag(";"),
        )
        .map(|(f, _)| f)
        .parse(input)
    }

    pub fn parse_type_alias(input: Span) -> B2Result<Self> {
        context(
            "type-alias-statement",
            terminated(
                pair(
                    delimited(
                        context(
                            "type-alias-kw-and-multispace",
                            preceded(tag(TYPE_ALIAS_KW), multispace0),
                        ),
                        context(
                            "type-alias-identifier",
                            parse_identifier.map(|s| s.to_string()),
                        ),
                        context("type-alias-type", preceded(multispace0, tag(ASSIGNMENT_KW))),
                    ),
                    preceded(multispace0, LexType::parse_type),
                ),
                tag(END_STMT_KW),
            ),
        )
        .map(|(identifier, b2_type)| Self::TypeAlias {
            identifier,
            b2_type,
        })
        .parse(input)
    }

    pub fn parse_import_module(input: Span) -> B2Result<Self> {
        context(
            "import-module-statement",
            terminated(
                preceded(
                    preceded(tag(IMPORT_MODULE_KW), multispace0),
                    parse_identifier.map(|s| s.to_string()),
                ),
                tag(END_STMT_KW),
            ),
        )
        .map(|identifier| Self::ImportModule { identifier })
        .parse(input)
    }
}
