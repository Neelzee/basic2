use nom::{
    IResult, Parser, error::{ContextError, ErrorKind, ParseError}
};
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

pub type B2Result<'a, O> = IResult<Span<'a>, O, VerboseError<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct VerboseError<'a> {
    pub errors: Vec<(Span<'a>, VerboseErrorKind)>,
}

impl<'a> VerboseError<'a> {
    pub fn new(input: Span<'a>, ctx: &'static str) -> Self {
        Self {
            errors: vec![(input, VerboseErrorKind::Context(ctx))],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerboseErrorKind {
    Context(&'static str),
    Char(char),
    Nom(ErrorKind),
}

impl<'a> ParseError<Span<'a>> for VerboseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        Self {
            errors: vec![(input, VerboseErrorKind::Nom(kind))],
        }
    }

    fn append(input: Span<'a>, kind: ErrorKind, other: Self) -> Self {
        let mut errors = other.errors;
        errors.push((input, VerboseErrorKind::Nom(kind)));
        Self { errors }
    }
}

impl<'a> ContextError<Span<'a>> for VerboseError<'a> {
    fn add_context(input: Span<'a>, ctx: &'static str, other: Self) -> Self {
        let mut errors = other.errors;
        errors.push((input, VerboseErrorKind::Context(ctx)));
        Self { errors }
    }
}
