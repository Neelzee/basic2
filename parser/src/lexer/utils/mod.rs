use nom::{
    IResult,
    error::{ContextError, ErrorKind, ParseError},
};
use nom_language::error::VerboseError;
use nom_locate::LocatedSpan;

pub mod consts;
pub mod helper_parsers;

#[cfg(test)]
mod test_utils;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone)]
pub struct Token<'a, O> {
    pub position: Span<'a>,
    pub inner: O,
}

impl<O: PartialEq> PartialEq for Token<'_, O> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

pub type B2Error<'a> = VerboseError<Span<'a>>;

pub type B2Result<'a, O> = IResult<Span<'a>, O, B2Error<'a>>;
