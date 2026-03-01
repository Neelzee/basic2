use crate::{
    common::BinOp,
    lexer::{
        lex_expr::LexExpr,
        lex_type::LexType,
        utils::{
            B2Result, Span,
            consts::{
                BLOCK_STATEMENT_END_KW, BLOCK_STATEMENT_START_KW, FUNCTION_BODY_END_KW,
                FUNCTION_DECLARATION_KW, FUNCTION_IMPLEMENTATION_KW,
                FUNCTION_IMPLEMENTATION_START_KW, FUNCTION_INVOCATION_END,
                FUNCTION_INVOCATION_START_KW, FUNCTION_PARAMETERS_DELIMITER,
                FUNCTION_PARAMETERS_DELIMITER_CHAR, FUNCTION_PARAMETERS_END,
                FUNCTION_PARAMETERS_END_CHAR, FUNCTION_PARAMETERS_START,
                IF_STATEMENT_BODY_START_KW, IF_STATEMENT_END_KW, IF_STATEMENT_START_KW,
                STRUCT_DECL_KW, STRUCT_END_KW, STRUCT_FIELD_DECL_KW, STRUCT_FIELD_IMPL_KW,
                STRUCT_KW, STRUCT_START_KW, VARIABLE_REASIGNMENT, WHILE_STATEMENT_BODY_START_KW,
                WHILE_STATEMENT_END_KW, WHILE_STATEMENT_START_KW,
            },
            helper_parsers::{
                parse_comment, parse_identifier, parse_parameters, parse_poly_list_with,
                till_end_of_stmt,
            },
        },
    },
};
use nom::{
    IResult, Parser,
    branch::{alt, permutation},
    bytes::complete::{tag, take, take_till, take_until},
    character::complete::{
        alpha1, alphanumeric0, char, digit1, multispace0, none_of, one_of, space0, space1,
    },
    combinator::{cut, opt},
    multi::{many0, separated_list0},
    number::complete::float,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
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
}

impl LexStmt {
    pub fn parse_statement(input: Span) -> B2Result<Self> {
        alt((
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
        ))
        .parse(input)
    }

    pub fn parse_variable_declaration(input: Span) -> B2Result<Self> {
        let (i, identifier) = preceded(
            tag(VARIABLE_DECLARATION),
            preceded(multispace0, parse_identifier),
        )
        .map(|s| s.to_string())
        .parse(input)?;
        let (rem, variable_type) = terminated(
            opt(preceded(
                tag(":"),
                preceded(multispace0, LexType::parse_type),
            )),
            tag(";"),
        )
        .parse(i)?;

        Ok((
            rem,
            Self::VariableDeclaration {
                identifier,
                variable_type,
            },
        ))
    }

    pub fn parse_variable_declaration_assignment(input: Span) -> B2Result<Self> {
        let (i, identifier) = preceded(
            tag(VARIABLE_DECLARATION),
            preceded(multispace0, parse_identifier),
        )
        .map(|s| s.to_string())
        .parse(input)?;
        let (i, variable_type) = opt(preceded(tag(":"), LexType::parse_type)).parse(i)?;
        let (i, _whitespace) = multispace0.parse(i)?;
        let (i, value) = preceded(
            tag("="),
            preceded(multispace0, till_end_of_stmt.and_then(LexExpr::parse_expr)),
        )
        .parse(i)?;
        let (rem, _end_kw) = tag(";").parse(i)?;

        Ok((
            rem,
            Self::VariableDeclarationAssignment {
                identifier,
                variable_type,
                value,
            },
        ))
    }

    pub fn parse_variable_reassignment(input: Span) -> B2Result<Self> {
        let (i, identifier) = preceded(multispace0, parse_identifier)
            .map(|s| s.to_string())
            .parse(input)?;
        let (i, _whitespace) = multispace0.parse(i)?;
        let (i, reassignment) = preceded(space0, opt(BinOp::parse_symbol)).parse(i)?;
        let (rem, new_value) = preceded(
            tag(VARIABLE_REASIGNMENT),
            till_end_of_stmt.and_then(LexExpr::parse_expr),
        )
        .parse(i)?;

        Ok((
            rem,
            Self::VariableReassignment {
                identifier,
                reassignment,
                new_value,
            },
        ))
    }

    pub fn parse_if_statement(input: Span) -> B2Result<Self> {
        let (i, _whitespace) = multispace0.parse(input)?;
        let (i, condition) = preceded(tag(IF_STATEMENT_START_KW), LexExpr::parse_expr).parse(i)?;
        let (i, _then_kw) = preceded(multispace0, tag(IF_STATEMENT_BODY_START_KW)).parse(i)?;
        let (i, body) = many0(preceded(multispace0, Self::parse_statement)).parse(i)?;
        let (rem, _end_kw) = preceded(multispace0, tag(IF_STATEMENT_END_KW)).parse(i)?;
        Ok((rem, Self::If { condition, body }))
    }

    pub fn parse_while_statement(input: Span) -> B2Result<Self> {
        let (i, _whitespace) = multispace0.parse(input)?;
        let (i, condition) =
            preceded(tag(WHILE_STATEMENT_START_KW), LexExpr::parse_expr).parse(i)?;
        let (i, _do_kw) = preceded(multispace0, tag(WHILE_STATEMENT_BODY_START_KW)).parse(i)?;
        let (i, body) = many0(preceded(multispace0, Self::parse_statement)).parse(i)?;
        let (rem, _end_kw) = preceded(multispace0, tag(WHILE_STATEMENT_END_KW)).parse(i)?;
        Ok((rem, Self::While { condition, body }))
    }

    pub fn parse_function_declaration(input: Span) -> B2Result<Self> {
        let (i, _whitespace) = multispace0.parse(input)?;
        let (i, identifier) = preceded(
            tag(FUNCTION_DECLARATION_KW),
            preceded(space1, parse_identifier),
        )
        .map(|s| s.to_string())
        .parse(i)?;
        let (i, parameters) = parse_poly_list_with(
            FUNCTION_PARAMETERS_START,
            FUNCTION_PARAMETERS_DELIMITER,
            FUNCTION_PARAMETERS_END,
            LexType::parse_type,
        )
        .parse(i)?;
        let (i, return_type) = opt(preceded(
            space0,
            preceded(tag(":"), preceded(space0, LexType::parse_type)),
        ))
        .parse(i)?;
        let (rem, _stmt_end) = tag(";").parse(i)?;

        Ok((
            rem,
            Self::FunctionDeclaration {
                identifier,
                parameters,
                return_type,
            },
        ))
    }

    pub fn parse_function_implementation(input: Span) -> B2Result<Self> {
        let (i, _whitespace) = multispace0.parse(input)?;
        let (i, identifier) = preceded(
            tag(FUNCTION_IMPLEMENTATION_KW),
            preceded(space1, parse_identifier),
        )
        .map(|s| s.to_string())
        .parse(i)?;
        let (i, parameters) = parse_poly_list_with(
            FUNCTION_PARAMETERS_START,
            FUNCTION_PARAMETERS_DELIMITER,
            FUNCTION_PARAMETERS_END,
            parse_parameters,
        )
        .parse(i)?;
        let (i, _do_kw) = preceded(multispace0, tag(FUNCTION_IMPLEMENTATION_START_KW)).parse(i)?;
        let (i, body) = many0(preceded(multispace0, Self::parse_statement)).parse(i)?;
        let (rem, _end_kw) = preceded(multispace0, tag(FUNCTION_BODY_END_KW)).parse(i)?;

        Ok((
            rem,
            Self::FunctionImplementation {
                identifier,
                parameters,
                body,
            },
        ))
    }

    pub fn parse_block_statement(input: Span) -> B2Result<Self> {
        let (i, _multispace) = multispace0.parse(input)?;
        let (i, _block_start_kw) = tag(BLOCK_STATEMENT_START_KW).parse(i)?;
        let (i, body) = many0(alt((
            parse_comment.map(|_| None),
            Self::parse_statement.map(|x| Some(x)),
        )))
        .map(|xs| xs.into_iter().filter_map(|x| x).collect())
        .parse(i)?;
        let (rem, _block_end_kw) = preceded(multispace0, tag(BLOCK_STATEMENT_END_KW)).parse(i)?;
        Ok((rem, Self::Block { body }))
    }

    pub fn parse_function_invocation(input: Span) -> B2Result<Self> {
        let (i, _multispace) =
            preceded(multispace0, tag(FUNCTION_INVOCATION_START_KW)).parse(input)?;
        let (i, function) = preceded(space0, LexExpr::parse_function_call).parse(i)?;
        let (rem, _end_stmt_kw) = preceded(
            take_until(FUNCTION_INVOCATION_END),
            take(FUNCTION_INVOCATION_END.chars().count()),
        )
        .parse(i)?;
        match function {
            LexExpr::FunctionCall {
                identifier,
                arguments,
            } => Ok((
                rem,
                Self::FunctionInvocation {
                    identifier,
                    arguments,
                },
            )),
            _ => unreachable!("parse_function_call should only return functioncall"),
        }
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
}
