use crate::{
    common::binop::BinOp,
    lexer::{
        lex_expr::LexExpr,
        lex_type::LexType,
        utils::{
            B2Result, Span,
            consts::{
                ASSIGNMENT_KW, BLOCK_STATEMENT_END_KW, BLOCK_STATEMENT_START_KW, BREAK_STMT_KW,
                END_STMT_KW, FOR_BODY_START_KW, FOR_CONDITION_END_KW, FOR_CONDITION_START_KW,
                FOR_END_KW, FOR_START_KW, FUNCTION_BODY_END_KW, FUNCTION_DECLARATION_KW,
                FUNCTION_IMPLEMENTATION_KW, FUNCTION_IMPLEMENTATION_START_KW,
                FUNCTION_INVOCATION_END, FUNCTION_INVOCATION_START_KW,
                FUNCTION_PARAMETERS_DELIMITER, FUNCTION_PARAMETERS_END, FUNCTION_PARAMETERS_START,
                IF_STATEMENT_BODY_START_KW, IF_STATEMENT_END_KW, IF_STATEMENT_START_KW,
                IMPORT_MODULE_KW, LIST_END, LIST_START, RETURN_STMT_KW, STRUCT_DECL_KW,
                STRUCT_END_KW, STRUCT_FIELD_DECL_KW, STRUCT_KW, TUPLE_DELIMITER, TUPLE_END,
                TUPLE_START, TYPE_ALIAS_KW, UNPACK_KW, VARIABLE_DECLARATION, VARIABLE_REASIGNMENT,
                VARIABLE_TYPE_START, WHILE_STATEMENT_BODY_START_KW, WHILE_STATEMENT_END_KW,
                WHILE_STATEMENT_START_KW,
            },
            helper_parsers::{
                parse_comments, parse_identifier, parse_parameters, parse_poly_list_with,
                parse_statements,
            },
        },
    },
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, space0},
    combinator::opt,
    error::context,
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
};

#[derive(Debug)]
pub enum LexStmt<'a> {
    VariableDeclaration {
        identifier: Span<'a>,
        variable_type: Option<LexType<'a>>,
    },
    VariableUnpacking {
        identifiers: Vec<Span<'a>>,
        value: LexExpr<'a>,
    },
    VariableDeclarationAssignment {
        identifier: Span<'a>,
        variable_type: Option<LexType<'a>>,
        value: LexExpr<'a>,
    },
    VariableReassignment {
        identifier: Span<'a>,
        reassignment: Option<BinOp>,
        new_value: LexExpr<'a>,
    },
    ListReassignment {
        indexee: LexExpr<'a>,
        index: LexExpr<'a>,
        reassignment: Option<BinOp>,
        new_value: LexExpr<'a>,
    },
    If {
        condition: LexExpr<'a>,
        body: Vec<Self>,
    },
    While {
        condition: LexExpr<'a>,
        body: Vec<Self>,
    },
    FunctionDeclaration {
        identifier: Span<'a>,
        parameters: Vec<LexType<'a>>,
        return_type: Option<LexType<'a>>,
    },
    FunctionImplementation {
        identifier: Span<'a>,
        parameters: Vec<(Span<'a>, Option<LexExpr<'a>>)>,
        body: Vec<Self>,
    },
    StructDeclaration {
        identifier: Span<'a>,
        fields: Vec<(Span<'a>, LexType<'a>)>,
    },
    Block {
        body: Vec<Self>,
    },
    FunctionInvocation {
        identifier: Span<'a>,
        arguments: Vec<LexExpr<'a>>,
    },
    Break,
    Return {
        value: Option<LexExpr<'a>>,
    },
    TypeAlias {
        identifier: Span<'a>,
        b2_type: LexType<'a>,
    },
    ImportModule {
        identifier: Span<'a>,
    },
    For {
        start_stmt: Box<Self>,
        condition: LexExpr<'a>,
        incrementer: LexExpr<'a>,
        body: Vec<Self>,
    },
}

impl<'a> LexStmt<'a> {
    pub fn parse_statement(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "statements",
            alt((
                Self::parse_type_alias,
                Self::parse_list_reassignment,
                Self::parse_variable_unpacking,
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
                Self::parse_for_statement,
            )),
        )
        .parse(input)
    }

    pub fn parse_break(input: Span) -> B2Result<Self> {
        delimited(multispace0, tag(BREAK_STMT_KW), tag(END_STMT_KW))
            .map(|_| Self::Break)
            .parse(input)
    }

    pub fn parse_return(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "return",
            delimited(
                preceded(multispace0, context("return-kw", tag(RETURN_STMT_KW))),
                context("return-value", opt(preceded(space0, LexExpr::parse_expr))),
                tag(END_STMT_KW),
            ),
        )
        .map(|value| Self::Return { value })
        .parse(input)
    }

    pub fn parse_variable_declaration(input: Span<'a>) -> B2Result<'a, Self> {
        delimited(
            preceded(multispace0, tag(VARIABLE_DECLARATION)),
            (
                preceded(multispace0, parse_identifier),
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

    pub fn parse_variable_declaration_assignment(input: Span<'a>) -> B2Result<'a, Self> {
        delimited(
            preceded(multispace0, tag(VARIABLE_DECLARATION)),
            (
                preceded(multispace0, parse_identifier),
                preceded(
                    multispace0,
                    opt(preceded(
                        (tag(VARIABLE_TYPE_START), multispace0),
                        LexType::parse_type,
                    )),
                ),
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

    pub fn parse_variable_reassignment(input: Span<'a>) -> B2Result<'a, Self> {
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
                identifier: ident,
                reassignment,
                new_value,
            },
        )
        .parse(input)
    }

    pub fn parse_if_statement(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_while_statement(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_function_declaration(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "function-declaration",
            delimited(
                tag(FUNCTION_DECLARATION_KW).and(multispace0),
                (
                    parse_identifier,
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

    pub fn parse_function_implementation(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "function-implementation",
            delimited(
                context("impl-kw", tag(FUNCTION_IMPLEMENTATION_KW)).and(multispace0),
                (
                    context("function-identifier", parse_identifier),
                    context(
                        "parameterers",
                        parse_poly_list_with(
                            FUNCTION_PARAMETERS_START,
                            FUNCTION_PARAMETERS_DELIMITER,
                            FUNCTION_PARAMETERS_END,
                            parse_parameters,
                        ),
                    ),
                    preceded(
                        (
                            multispace0,
                            context("start-kw", tag(FUNCTION_IMPLEMENTATION_START_KW)),
                            multispace0,
                        ),
                        context("function-body", parse_statements),
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

    pub fn parse_block_statement(input: Span<'a>) -> B2Result<'a, Self> {
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
                    many0(alt((multispace1, parse_comments))).and(tag(BLOCK_STATEMENT_END_KW)),
                ),
            )
            .map(|body| Self::Block { body }),
        )
        .parse(input)
    }

    pub fn parse_function_invocation(input: Span<'a>) -> B2Result<'a, Self> {
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

    pub fn parse_struct_declaration(input: Span<'a>) -> B2Result<'a, Self> {
        let (i, identifier) =
            preceded(tag(STRUCT_KW), preceded(space0, parse_identifier)).parse(input)?;
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

    pub fn parse_struct_field_statement(input: Span) -> B2Result<(Span, LexType)> {
        pair(
            pair(
                preceded(
                    tag(STRUCT_FIELD_DECL_KW),
                    preceded(space0, parse_identifier),
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

    pub fn parse_type_alias(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "type-alias-statement",
            terminated(
                pair(
                    delimited(
                        context(
                            "type-alias-kw-and-multispace",
                            preceded(tag(TYPE_ALIAS_KW), multispace0),
                        ),
                        context("type-alias-identifier", parse_identifier),
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

    pub fn parse_import_module(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "import-module-statement",
            terminated(
                preceded(
                    preceded(tag(IMPORT_MODULE_KW), multispace0),
                    parse_identifier,
                ),
                tag(END_STMT_KW),
            ),
        )
        .map(|identifier| Self::ImportModule { identifier })
        .parse(input)
    }

    pub fn parse_variable_unpacking(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "variable-unpacking",
            delimited(
                (tag(VARIABLE_DECLARATION), multispace0),
                (
                    parse_poly_list_with(TUPLE_START, TUPLE_DELIMITER, TUPLE_END, parse_identifier),
                    (multispace0, tag(UNPACK_KW), multispace0),
                    LexExpr::parse_expr,
                ),
                (multispace0, tag(END_STMT_KW)),
            ),
        )
        .map(|(identifiers, _, value)| Self::VariableUnpacking { identifiers, value })
        .parse(input)
    }

    pub fn parse_for_statement(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "for",
            terminated(
                (
                    delimited(
                        (tag(FOR_START_KW), multispace0),
                        delimited(
                            (tag(FOR_CONDITION_START_KW), multispace0),
                            (
                                Self::parse_variable_declaration_assignment.map(|b| Box::new(b)),
                                delimited(multispace0, LexExpr::parse_expr, tag(END_STMT_KW)),
                                delimited(multispace0, LexExpr::parse_expr, tag(END_STMT_KW)),
                            ),
                            (multispace0, tag(FOR_CONDITION_END_KW)),
                        ),
                        (multispace0, tag(FOR_BODY_START_KW)),
                    ),
                    parse_statements,
                ),
                (multispace0, tag(FOR_END_KW)),
            ),
        )
        .map(|((start_stmt, condition, incrementer), body)| Self::For {
            start_stmt,
            condition,
            incrementer,
            body,
        })
        .parse(input)
    }

    pub fn parse_list_reassignment(input: Span<'a>) -> B2Result<'a, Self> {
        context(
            "list-reassignment",
            (
                context(
                    "indexee",
                    alt((LexExpr::parse_list, LexExpr::parse_variable)),
                ),
                context(
                    "index",
                    delimited(
                        (tag(LIST_START), multispace0),
                        LexExpr::parse_expr,
                        (multispace0, tag(LIST_END)),
                    ),
                ),
                context(
                    "reassignment",
                    delimited(multispace0, opt(BinOp::parse_symbol), tag(ASSIGNMENT_KW)),
                ),
                context(
                    "new_value",
                    delimited(multispace0, LexExpr::parse_expr, tag(END_STMT_KW)),
                ),
            ),
        )
        .map(
            |(indexee, index, reassignment, new_value)| Self::ListReassignment {
                indexee,
                index,
                reassignment,
                new_value,
            },
        )
        .parse(input)
    }
}

impl<'a> PartialEq for LexStmt<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::VariableDeclaration {
                    identifier: l_identifier,
                    variable_type: l_variable_type,
                },
                Self::VariableDeclaration {
                    identifier: r_identifier,
                    variable_type: r_variable_type,
                },
            ) => {
                l_identifier.to_string() == r_identifier.to_string()
                    && l_variable_type == r_variable_type
            }
            (
                Self::VariableUnpacking {
                    identifiers: l_identifiers,
                    value: l_value,
                },
                Self::VariableUnpacking {
                    identifiers: r_identifiers,
                    value: r_value,
                },
            ) => {
                l_identifiers
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    == r_identifiers
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                    && l_value == r_value
            }
            (
                Self::VariableDeclarationAssignment {
                    identifier: l_identifier,
                    variable_type: l_variable_type,
                    value: l_value,
                },
                Self::VariableDeclarationAssignment {
                    identifier: r_identifier,
                    variable_type: r_variable_type,
                    value: r_value,
                },
            ) => {
                l_identifier.to_string() == r_identifier.to_string()
                    && l_variable_type == r_variable_type
                    && l_value == r_value
            }
            (
                Self::VariableReassignment {
                    identifier: l_identifier,
                    reassignment: l_reassignment,
                    new_value: l_new_value,
                },
                Self::VariableReassignment {
                    identifier: r_identifier,
                    reassignment: r_reassignment,
                    new_value: r_new_value,
                },
            ) => {
                l_identifier.to_string() == r_identifier.to_string()
                    && l_reassignment == r_reassignment
                    && l_new_value == r_new_value
            }
            (
                Self::ListReassignment {
                    indexee: l_indexee,
                    index: l_index,
                    reassignment: l_reassignment,
                    new_value: l_new_value,
                },
                Self::ListReassignment {
                    indexee: r_indexee,
                    index: r_index,
                    reassignment: r_reassignment,
                    new_value: r_new_value,
                },
            ) => {
                l_indexee == r_indexee
                    && l_index == r_index
                    && l_reassignment == r_reassignment
                    && l_new_value == r_new_value
            }
            (
                Self::If {
                    condition: l_condition,
                    body: l_body,
                },
                Self::If {
                    condition: r_condition,
                    body: r_body,
                },
            ) => l_condition == r_condition && l_body == r_body,
            (
                Self::While {
                    condition: l_condition,
                    body: l_body,
                },
                Self::While {
                    condition: r_condition,
                    body: r_body,
                },
            ) => l_condition == r_condition && l_body == r_body,
            (
                Self::FunctionDeclaration {
                    identifier: l_identifier,
                    parameters: l_parameters,
                    return_type: l_return_type,
                },
                Self::FunctionDeclaration {
                    identifier: r_identifier,
                    parameters: r_parameters,
                    return_type: r_return_type,
                },
            ) => {
                l_identifier.to_string() == r_identifier.to_string()
                    && l_parameters == r_parameters
                    && l_return_type == r_return_type
            }
            (
                Self::FunctionImplementation {
                    identifier: l_identifier,
                    parameters: l_parameters,
                    body: l_body,
                },
                Self::FunctionImplementation {
                    identifier: r_identifier,
                    parameters: r_parameters,
                    body: r_body,
                },
            ) => {
                l_identifier.to_string() == r_identifier.to_string()
                    && l_parameters
                        .iter()
                        .map(|(s, e)| (s.to_string(), e))
                        .collect::<Vec<_>>()
                        == r_parameters
                            .iter()
                            .map(|(s, e)| (s.to_string(), e))
                            .collect::<Vec<_>>()
                    && l_body == r_body
            }
            (
                Self::StructDeclaration {
                    identifier: l_identifier,
                    fields: l_fields,
                },
                Self::StructDeclaration {
                    identifier: r_identifier,
                    fields: r_fields,
                },
            ) => {
                l_identifier.to_string() == r_identifier.to_string()
                    && l_fields
                        .iter()
                        .map(|(s, e)| (s.to_string(), e))
                        .collect::<Vec<_>>()
                        == r_fields
                            .iter()
                            .map(|(s, e)| (s.to_string(), e))
                            .collect::<Vec<_>>()
            }
            (Self::Block { body: l_body }, Self::Block { body: r_body }) => l_body == r_body,
            (
                Self::FunctionInvocation {
                    identifier: l_identifier,
                    arguments: l_arguments,
                },
                Self::FunctionInvocation {
                    identifier: r_identifier,
                    arguments: r_arguments,
                },
            ) => l_identifier.to_string() == r_identifier.to_string() && l_arguments == r_arguments,
            (Self::Return { value: l_value }, Self::Return { value: r_value }) => {
                l_value == r_value
            }
            (
                Self::TypeAlias {
                    identifier: l_identifier,
                    b2_type: l_b2_type,
                },
                Self::TypeAlias {
                    identifier: r_identifier,
                    b2_type: r_b2_type,
                },
            ) => l_identifier.to_string() == r_identifier.to_string() && l_b2_type == r_b2_type,
            (
                Self::ImportModule {
                    identifier: l_identifier,
                },
                Self::ImportModule {
                    identifier: r_identifier,
                },
            ) => l_identifier.to_string() == r_identifier.to_string(),
            (
                Self::For {
                    start_stmt: l_start_stmt,
                    condition: l_condition,
                    incrementer: l_incrementer,
                    body: l_body,
                },
                Self::For {
                    start_stmt: r_start_stmt,
                    condition: r_condition,
                    incrementer: r_incrementer,
                    body: r_body,
                },
            ) => {
                l_start_stmt == r_start_stmt
                    && l_condition == r_condition
                    && l_incrementer == r_incrementer
                    && l_body == r_body
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
