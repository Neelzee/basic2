use crate::lexer::{
    lex_stmt::LexStmt,
    utils::{
        B2Result, Span,
        consts::{BEGIN_MODULE_KW, END_MODULE_KW, MODULE_KW, ONE_SPACE},
        helper_parsers::{parse_comment, parse_identifier},
    },
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, space1},
    combinator::cut,
    error::{ErrorKind, FromExternalError, ParseError, context},
    multi::many0,
    sequence::{delimited, pair, preceded},
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
            )
            .and(context(
                "module-statements",
                many0(preceded(
                    alt((multispace0, parse_comment)),
                    LexStmt::parse_statement,
                )),
            ))
            .and(context(
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
            )),
        )
        .map(|((start_identifier, statements), end_identifier)| Self {
            start_identifier,
            end_identifier,
            statements,
        })
        .parse(input)
    }
}
