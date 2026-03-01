use crate::lexer::{
    lex_stmt::LexStmt,
    utils::{
        B2Result, Span, VerboseError,
        consts::{BEGIN_MODULE_KW, END_MODULE_KW, MODULE_KW},
        helper_parsers::{parse_comment, parse_identifier},
    },
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, space1},
    multi::many0,
    sequence::preceded,
};

#[derive(Debug)]
pub struct LexProgram {
    identifier: String,
    statements: Vec<LexStmt>,
}

impl LexProgram {
    pub fn parse_program(input: Span) -> B2Result<Self> {
        let (i, identifier) = preceded(
            tag(BEGIN_MODULE_KW),
            preceded(
                space1,
                preceded(tag(MODULE_KW), preceded(space1, parse_identifier)),
            ),
        )
        .map(|s| s.to_string())
        .parse(input)?;
        let (i, _newline) = alt((tag("\n"), tag("\r\n"))).parse(i)?;
        let (i, statements) = many0(alt((
            parse_comment.map(|_| None),
            LexStmt::parse_statement.map(|s| Some(s)),
        )))
        .map(|xs| xs.into_iter().filter_map(|x| x).collect())
        .parse(i)?;
        let (rem, _end_module_kw) = preceded(
            multispace0,
            preceded(
                tag(MODULE_KW),
                preceded(
                    space1,
                    preceded(
                        tag(identifier.as_str()),
                        preceded(space1, tag(END_MODULE_KW)),
                    ),
                ),
            ),
        )
        .parse(i)?;

        if !rem.is_empty() {
            return Err(nom::Err::Failure(VerboseError::new(
                i,
                "Extranous text remainding after program parse",
            )));
        }

        Ok((
            Span::new(""),
            LexProgram {
                identifier,
                statements,
            },
        ))
    }
}
