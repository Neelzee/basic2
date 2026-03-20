use crate::{
    common::binop::BinOp,
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::{FunDeclComps, FunImplComps, Import, LexStmt, when_match::WhenMatch},
        lex_type::LexType,
        utils::{
            B2LexError, B2LexResult, Span,
            consts::{
                ASSIGNMENT_KW, BLOCK_STATEMENT_END_KW, BLOCK_STATEMENT_START_KW, BREAK_STMT_KW,
                CONTINUE_STMT_KW, END_STMT_KW, ENUM_END_KW, ENUM_START_KW, FOR_BODY_START_KW,
                FOR_CONDITION_END_KW, FOR_CONDITION_START_KW, FOR_END_KW, FOR_START_KW,
                FUNCTION_BODY_END_KW, FUNCTION_DECLARATION_KW, FUNCTION_GENERIC_TRAIT_KW,
                FUNCTION_GENERIC_TRAIT_SEP, FUNCTION_GENERICS_DELIMITER, FUNCTION_GENERICS_END,
                FUNCTION_GENERICS_START, FUNCTION_IMPLEMENTATION_KW,
                FUNCTION_IMPLEMENTATION_START_KW, FUNCTION_INVOCATION_END,
                FUNCTION_INVOCATION_START_KW, FUNCTION_PARAMETERS_DELIMITER,
                FUNCTION_PARAMETERS_END, FUNCTION_PARAMETERS_START, IF_STATEMENT_BODY_START_KW,
                IF_STATEMENT_END_KW, IF_STATEMENT_START_KW, IMPORT_MODULE_KW, LIST_DELIMITER,
                LIST_END, LIST_START, LIST_UNPACKING_KW, RETURN_STMT_KW, STRUCT_DECL_KW,
                STRUCT_END_KW, STRUCT_FIELD_ACCESS_KW, STRUCT_FIELD_DECL_KW, STRUCT_KW,
                TRAIT_DECL_BODY_END_KW, TRAIT_DECL_BODY_START_KW, TRAIT_DECL_KW,
                TRAIT_IMPL_BODY_END_KW, TRAIT_IMPL_BODY_START_KW, TRAIT_IMPL_KW,
                TRAIT_RESTRICTION_KW, TRAIT_RESTRICTION_SEP_KW, TUPLE_DELIMITER, TUPLE_END,
                TUPLE_START, TYPE_ALIAS_KW, UNPACK_KW, VARIABLE_DECLARATION, VARIABLE_REASIGNMENT,
                VARIABLE_TYPE_START, WHEN_STATEMENT_BODY_END_KW, WHEN_STATEMENT_BODY_START_KW,
                WHEN_STATEMENT_START_KW, WHILE_STATEMENT_BODY_START_KW, WHILE_STATEMENT_END_KW,
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
    error::{ErrorKind, ParseError, context},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated},
};

impl<'a> LexStmt<'a> {
    pub fn parse_statement(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "statements",
            alt([
                Self::parse_type_alias,
                Self::parse_list_reassignment,
                Self::parse_tuple_unpacking,
                Self::parse_list_unpacking,
                Self::parse_struct_unpacking,
                Self::parse_variable_declaration,
                Self::parse_variable_declaration_assignment,
                Self::parse_struct_field_reassignment,
                Self::parse_variable_reassignment,
                Self::parse_if_statement,
                Self::parse_while_statement,
                Self::parse_when_statement,
                Self::parse_function_declaration,
                Self::parse_function_implementation,
                Self::parse_block_statement,
                Self::parse_function_invocation,
                Self::parse_struct_declaration,
                Self::parse_break,
                Self::parse_return,
                Self::parse_import_module,
                Self::parse_for_statement,
                Self::parse_enum_declaration,
                Self::parse_trait_decl,
                Self::parse_trait_impl,
                Self::parse_continue,
            ]),
        )
        .parse(input)
    }

    pub fn parse_break(input: Span<'a>) -> B2LexResult<'a, Self> {
        delimited(multispace0, tag(BREAK_STMT_KW), tag(END_STMT_KW))
            .map(|_| Self::Break)
            .parse(input)
    }

    pub fn parse_continue(input: Span<'a>) -> B2LexResult<'a, Self> {
        delimited(multispace0, tag(CONTINUE_STMT_KW), tag(END_STMT_KW))
            .map(|_| Self::Continue)
            .parse(input)
    }

    pub fn parse_return(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_variable_declaration(input: Span<'a>) -> B2LexResult<'a, Self> {
        delimited(
            preceded(multispace0, tag(VARIABLE_DECLARATION)),
            (
                preceded(multispace0, parse_identifier),
                preceded(
                    (multispace0, tag(VARIABLE_TYPE_START), multispace0),
                    LexType::parse_type,
                ),
            ),
            tag(END_STMT_KW),
        )
        .map(|(identifier, variable_type)| Self::new_var_decl(identifier, variable_type))
        .parse(input)
    }

    pub fn parse_variable_declaration_assignment(input: Span<'a>) -> B2LexResult<'a, Self> {
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
        .map(|(identifier, variable_type, value)| {
            Self::new_var_decl_ass(identifier, variable_type, value)
        })
        .parse(input)
    }

    pub fn parse_struct_field_reassignment(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "struct-field-accessing",
            (
                parse_identifier,
                tag(STRUCT_FIELD_ACCESS_KW),
                parse_identifier,
                Self::parse_reasignment,
                delimited(multispace0, LexExpr::parse_expr, tag(END_STMT_KW)),
            ),
        )
        .map(
            |(identifier, _, field, reassignment, new_value)| Self::StructFieldReassignment {
                identifier,
                field,
                reassignment,
                new_value,
            },
        )
        .parse(input)
    }

    fn parse_reasignment(input: Span<'a>) -> B2LexResult<'a, Option<BinOp>> {
        context(
            "reassignment",
            terminated(
                preceded(space0, opt(BinOp::parse_symbol)),
                tag(VARIABLE_REASIGNMENT),
            ),
        )
        .parse(input)
    }

    pub fn parse_variable_reassignment(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "parse-variable-reassignment",
            preceded(
                multispace0,
                terminated(
                    (
                        parse_identifier,
                        Self::parse_reasignment,
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

    pub fn parse_if_statement(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_while_statement(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_function_declaration(input: Span<'a>) -> B2LexResult<'a, Self> {
        context("function-declaration", Self::parse_function_decl_comps)
            .map(|(identifier, generics, parameters, return_type)| {
                Self::new_fn_decl(identifier, parameters, generics, return_type)
            })
            .parse(input)
    }

    fn parse_generics(input: Span<'a>) -> B2LexResult<'a, Vec<(&'a str, Vec<&'a str>)>> {
        context(
            "parse-generics",
            parse_poly_list_with(
                FUNCTION_GENERICS_START,
                FUNCTION_GENERICS_DELIMITER,
                FUNCTION_GENERICS_END,
                (
                    parse_identifier,
                    opt(preceded(
                        (multispace0, tag(FUNCTION_GENERIC_TRAIT_KW), multispace0),
                        separated_list0(
                            (multispace0, tag(FUNCTION_GENERIC_TRAIT_SEP), multispace0),
                            parse_identifier,
                        ),
                    ))
                    .map(|x| x.unwrap_or_default()),
                ),
            ),
        )
        .parse(input)
    }

    fn parse_function_decl_comps(input: Span<'a>) -> B2LexResult<'a, FunDeclComps<'a>> {
        delimited(
            tag(FUNCTION_DECLARATION_KW).and(multispace0),
            (
                context("function-ident", parse_identifier),
                opt(Self::parse_generics).map(|x| x.unwrap_or_default()),
                context(
                    "function-parameters",
                    parse_poly_list_with(
                        FUNCTION_PARAMETERS_START,
                        FUNCTION_PARAMETERS_DELIMITER,
                        FUNCTION_PARAMETERS_END,
                        LexType::parse_type,
                    ),
                ),
                opt(preceded(
                    (multispace0, tag(":"), multispace0),
                    LexType::parse_type,
                )),
            ),
            tag(END_STMT_KW),
        )
        .parse(input)
    }

    pub fn parse_function_implementation(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "function-implementation",
            Self::parse_function_impl_components,
        )
        .map(|(identifier, parameters, body)| Self::new_fn_impl(identifier, parameters, body))
        .parse(input)
    }

    fn parse_function_impl_components(input: Span<'a>) -> B2LexResult<'a, FunImplComps<'a>> {
        alt((
            context(
                "function-impl-body",
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
                                context("does-kw", tag(FUNCTION_IMPLEMENTATION_START_KW)),
                                multispace0,
                            ),
                            context("function-body", parse_statements),
                        ),
                    ),
                    (multispace0, tag(FUNCTION_BODY_END_KW)),
                ),
            ),
            context(
                "function-impl-single-stmt",
                preceded(
                    context("impl-kw", (tag(FUNCTION_IMPLEMENTATION_KW), multispace0)),
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
                        context(
                            "single-statement",
                            preceded(multispace0, Self::parse_statement),
                        )
                        .map(|x| vec![x]),
                    ),
                ),
            ),
        ))
        .parse(input)
    }

    pub fn parse_block_statement(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_function_invocation(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_struct_declaration(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "structure-declaration",
            (
                context(
                    "structure-identifier",
                    preceded(
                        tag(STRUCT_KW),
                        preceded(
                            multispace0,
                            (
                                parse_identifier,
                                opt(Self::parse_generics).map(|x| x.unwrap_or_default()),
                            ),
                        ),
                    ),
                ),
                terminated(
                    context(
                        "structure-body",
                        preceded(
                            (multispace0, tag(STRUCT_DECL_KW)),
                            many0(preceded(multispace0, Self::parse_struct_field_statement)),
                        ),
                    ),
                    context(
                        "structure-end-kw",
                        preceded(multispace0, tag(STRUCT_END_KW)),
                    ),
                ),
            ),
        )
        .map(|((identifier, generics), fields)| Self::new_struct_decl(identifier, generics, fields))
        .parse(input)
    }

    pub fn parse_struct_field_statement(
        input: Span<'a>,
    ) -> B2LexResult<'a, (&'a str, LexType<'a>)> {
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

    pub fn parse_type_alias(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "type-alias-statement",
            terminated(
                pair(
                    delimited(
                        context(
                            "type-alias-kw-and-multispace",
                            preceded(tag(TYPE_ALIAS_KW), multispace0),
                        ),
                        (
                            context("type-alias-identifier", parse_identifier),
                            context(
                                "generics",
                                opt(Self::parse_generics).map(|x| x.unwrap_or_default()),
                            ),
                        ),
                        context("type-alias-type", preceded(multispace0, tag(ASSIGNMENT_KW))),
                    ),
                    preceded(multispace0, LexType::parse_type),
                ),
                tag(END_STMT_KW),
            ),
        )
        .map(|((identifier, generics), b2_type)| {
            Self::new_type_alias(identifier, generics, b2_type)
        })
        .parse(input)
    }

    pub fn parse_import_module(input: Span<'a>) -> B2LexResult<'a, Self> {
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
        .map(|identifier| Self::Import(Import { identifier }))
        .parse(input)
    }

    pub fn parse_tuple_unpacking(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "tuple-unpacking",
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
        .map(|(identifiers, _, value)| Self::new_tuple_unpack(identifiers, value))
        .parse(input)
    }

    pub fn parse_list_unpacking(input: Span<'a>) -> B2LexResult<'a, Self> {
        let (rem, (idents, _, value)) = context(
            "list-unpacking",
            delimited(
                (tag(VARIABLE_DECLARATION), multispace0),
                (
                    parse_poly_list_with(
                        LIST_START,
                        LIST_DELIMITER,
                        LIST_END,
                        alt((
                            parse_identifier.map(|i| Ok(i)),
                            preceded(tag(LIST_UNPACKING_KW), parse_identifier).map(|i| Err(i)),
                        )),
                    ),
                    (multispace0, tag(UNPACK_KW), multispace0),
                    LexExpr::parse_expr,
                ),
                (multispace0, tag(END_STMT_KW)),
            ),
        )
        .parse(input)?;

        let mut remainders = idents.iter().filter_map(|i| i.err()).collect::<Vec<_>>();
        if remainders.len() >= 2 {
            return Err(nom::Err::Error(B2LexError::from_error_kind(
                input,
                ErrorKind::Fail,
            )));
        }
        let remainder = remainders.pop();
        Ok((
            rem,
            Self::new_list_unpack(
                idents.into_iter().filter_map(|i| i.ok()).collect(),
                remainder,
                value,
            ),
        ))
    }

    pub fn parse_struct_unpacking(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "struct-unpacking",
            delimited(
                (tag(VARIABLE_DECLARATION), multispace0),
                (
                    parse_poly_list_with(
                        LIST_START,
                        LIST_DELIMITER,
                        LIST_END,
                        preceded(tag(STRUCT_FIELD_ACCESS_KW), parse_identifier),
                    ),
                    (multispace0, tag(UNPACK_KW), multispace0),
                    LexExpr::parse_expr,
                ),
                (multispace0, tag(END_STMT_KW)),
            ),
        )
        .map(|(identifiers, _, value)| Self::new_struct_unpack(identifiers, value))
        .parse(input)
    }

    pub fn parse_for_statement(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_list_reassignment(input: Span<'a>) -> B2LexResult<'a, Self> {
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

    pub fn parse_enum_declaration(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "enum-declaration",
            delimited(
                (tag(ENUM_START_KW), multispace0),
                (
                    parse_identifier,
                    opt(Self::parse_generics).map(|x| x.unwrap_or_default()),
                    preceded(
                        multispace0,
                        many0(preceded(
                            multispace0,
                            terminated(
                                (
                                    parse_identifier,
                                    opt(parse_poly_list_with(
                                        FUNCTION_PARAMETERS_START,
                                        FUNCTION_PARAMETERS_DELIMITER,
                                        FUNCTION_PARAMETERS_END,
                                        LexType::parse_type,
                                    ))
                                    .map(|x| x.unwrap_or_default()),
                                ),
                                tag(END_STMT_KW),
                            ),
                        )),
                    ),
                ),
                (multispace0, tag(ENUM_END_KW)),
            ),
        )
        .map(|(identifier, generics, enumerations)| {
            Self::new_enum_decl(identifier, generics, enumerations)
        })
        .parse(input)
    }

    pub fn parse_when_statement(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "when-statement",
            delimited(
                (tag(WHEN_STATEMENT_START_KW), multispace0),
                (
                    terminated(
                        parse_identifier,
                        (multispace0, tag(WHEN_STATEMENT_BODY_START_KW), multispace0),
                    ),
                    many0(delimited(multispace0, WhenMatch::parse, multispace0)),
                ),
                (multispace0, tag(WHEN_STATEMENT_BODY_END_KW)),
            ),
        )
        .map(|(identifier, branches)| Self::WhenStatement {
            identifier,
            branches,
        })
        .parse(input)
    }

    pub fn parse_trait_impl(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "trait-impl",
            delimited(
                context("trait-impl-kw", (tag(TRAIT_IMPL_KW), multispace0)),
                (
                    context(
                        "trait-ident",
                        terminated(
                            parse_identifier,
                            (multispace0, tag(TRAIT_IMPL_BODY_START_KW)),
                        ),
                    ),
                    context("trait-type", preceded(multispace0, parse_identifier)),
                    context("trait-body", parse_statements),
                ),
                context(
                    "trait-impl-end-kw",
                    (multispace0, tag(TRAIT_IMPL_BODY_END_KW)),
                ),
            ),
        )
        .map(|(trait_identifier, type_identifier, body)| {
            Self::new_trait_impl(trait_identifier, type_identifier, body)
        })
        .parse(input)
    }

    pub fn parse_trait_decl(input: Span<'a>) -> B2LexResult<'a, Self> {
        context(
            "trait-decl",
            delimited(
                (tag(TRAIT_DECL_KW), multispace0),
                (
                    terminated(
                        (
                            parse_identifier,
                            opt(preceded(
                                (multispace0, tag(TRAIT_RESTRICTION_KW)),
                                separated_list0(
                                    (multispace0, tag(TRAIT_RESTRICTION_SEP_KW), multispace0),
                                    delimited(multispace0, parse_identifier, multispace0),
                                ),
                            ))
                            .map(|x| x.unwrap_or_default()),
                        ),
                        (multispace0, tag(TRAIT_DECL_BODY_START_KW), multispace0),
                    ),
                    many0(alt((
                        Self::parse_function_impl_components.map(|o| Ok(o)),
                        Self::parse_function_decl_comps.map(|o| Err(o)),
                    ))),
                ),
                (multispace0, tag(TRAIT_DECL_BODY_END_KW)),
            ),
        )
        .map(|((identifier, restrictions), impls_decls)| {
            let mut decls = Vec::new();
            let mut impls = Vec::new();
            for id in impls_decls {
                match id {
                    Ok(i) => impls.push(i),
                    Err(d) => decls.push(d),
                }
            }
            Self::new_trait_decl(identifier, restrictions, decls, impls)
        })
        .parse(input)
    }
}
