use crate::lexer::{
    lex_stmt::LexStmt,
    utils::{
        B2Result, Span,
        consts::{BEGIN_MODULE_KW, END_MODULE_KW, MODULE_KW, ONE_SPACE},
        helper_parsers::{parse_identifier, parse_statements},
    },
};
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::multispace0,
    error::context,
    sequence::{delimited, preceded},
};

#[derive(Debug)]
pub struct LexProgram {
    start_identifier: String,
    end_identifier: String,
    statements: Vec<LexStmt>,
}

impl LexProgram {
    pub fn parse_program(input: Span) -> B2Result<Self> {
        context(
            "module",
            (
                context(
                    "module-start-identifier",
                    preceded(
                        (
                            tag(BEGIN_MODULE_KW),
                            tag(ONE_SPACE),
                            tag(MODULE_KW),
                            tag(ONE_SPACE),
                        ),
                        parse_identifier,
                    )
                    .map(|s| s.to_string()),
                ),
                context("module-statements", parse_statements),
                context(
                    "module-end-identifier",
                    preceded(
                        multispace0,
                        delimited(
                            (tag(MODULE_KW), tag(ONE_SPACE)),
                            parse_identifier,
                            (tag(ONE_SPACE), tag(END_MODULE_KW)),
                        ),
                    )
                    .map(|s| s.to_string()),
                ),
            ),
        )
        .map(|(start_identifier, statements, end_identifier)| Self {
            start_identifier,
            end_identifier,
            statements,
        })
        .parse(input)
    }
}
