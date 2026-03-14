use nom::{AsBytes, AsChar, Compare, FindSubstring, IResult, Input, Offset, ParseTo};
use nom_language::error::{VerboseError, VerboseErrorKind};
use nom_locate::LocatedSpan;
use std::{fmt::Write, str::FromStr};

pub mod consts;
pub mod helper_parsers;

#[cfg(test)]
mod test_utils;

pub struct Span<'a>(LocatedSpan<&'a str, ()>);

impl<'a> Span<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(LocatedSpan::new(s))
    }
}

impl<'a> Clone for Span<'a> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a> Copy for Span<'a> {}

impl<'a> Offset for Span<'a> {
    fn offset(&self, second: &Self) -> usize {
        self.0.offset(&second.0)
    }
}

impl<'a> FindSubstring<&str> for Span<'a> {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.0.find_substring(substr)
    }
}

impl<'a> std::fmt::Display for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> Compare<&'a str> for Span<'a> {
    fn compare(&self, t: &'a str) -> nom::CompareResult {
        self.0.compare(t)
    }

    fn compare_no_case(&self, t: &'a str) -> nom::CompareResult {
        self.0.compare_no_case(t)
    }
}

impl<'a> Compare<&'a [u8]> for Span<'_> {
    fn compare(&self, t: &'a [u8]) -> nom::CompareResult {
        self.0.compare(t)
    }

    fn compare_no_case(&self, t: &'a [u8]) -> nom::CompareResult {
        self.0.compare_no_case(t)
    }
}

impl<'a, T: FromStr> ParseTo<T> for Span<'a> {
    fn parse_to(&self) -> Option<T> {
        self.0.parse::<T>().ok()
    }
}

impl<'a> AsBytes for Span<'a> {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<'a> std::fmt::Debug for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> PartialEq for Span<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl<'a> Input for Span<'a> {
    type Item = <&'a str as Input>::Item;
    type Iter = <&'a str as Input>::Iter;
    type IterIndices = <&'a str as Input>::IterIndices;

    fn input_len(&self) -> usize {
        self.0.input_len()
    }

    fn take(&self, index: usize) -> Self {
        Self(self.0.take(index))
    }

    fn take_from(&self, index: usize) -> Self {
        Self(self.0.take_from(index))
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let (f, s) = self.0.take_split(index);
        (Self(f), Self(s))
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.position(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.0.iter_elements()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.0.iter_indices()
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.0.slice_index(count)
    }
}

pub type B2Error<'a> = VerboseError<Span<'a>>;

pub type B2Result<'a, O> = IResult<Span<'a>, O, B2Error<'a>>;

pub fn convert_error(input: Span, e: nom::Err<B2Error>) -> String {
    match e {
        err @ nom::Err::Incomplete(_) => format!("non-verbose-error: {err:?}"),
        nom::Err::Error(e) | nom::Err::Failure(e) => _convert_error(input, e),
    }
}

/// Inlined convert_error
fn _convert_error(input: Span, e: B2Error) -> String {
    let input = input.0;
    let mut result = String::new();

    for (i, (substring, kind)) in e.errors.iter().enumerate() {
        let substring = &substring.0;
        let offset = input.offset(substring);

        if input.is_empty() {
            match kind {
                VerboseErrorKind::Char(c) => {
                    write!(&mut result, "{}: expected '{}', got empty input\n\n", i, c)
                }
                VerboseErrorKind::Context(s) => {
                    write!(&mut result, "{}: in {}, got empty input\n\n", i, s)
                }
                VerboseErrorKind::Nom(e) => {
                    write!(&mut result, "{}: in {:?}, got empty input\n\n", i, e)
                }
            }
        } else {
            let prefix = &input.as_bytes()[..offset];

            let line_number = prefix.iter().filter(|&&b| b == b'\n').count() + 1;

            let line_begin = prefix
                .iter()
                .rev()
                .position(|&b| b == b'\n')
                .map(|pos| offset - pos)
                .unwrap_or(0);

            let line = input[line_begin..]
                .lines()
                .next()
                .unwrap_or(&input[line_begin..])
                .trim_end();

            let column_number = line.offset(substring) + 1;

            match kind {
                VerboseErrorKind::Char(c) => {
                    if let Some(actual) = substring.chars().next() {
                        write!(
                            &mut result,
                            "{i}: at line {line_number}:\n\
               {line}\n\
               {caret:>column$}\n\
               expected '{expected}', found {actual}\n\n",
                            i = i,
                            line_number = line_number,
                            line = line,
                            caret = '^',
                            column = column_number,
                            expected = c,
                            actual = actual,
                        )
                    } else {
                        write!(
                            &mut result,
                            "{i}: at line {line_number}:\n\
               {line}\n\
               {caret:>column$}\n\
               expected '{expected}', got end of input\n\n",
                            i = i,
                            line_number = line_number,
                            line = line,
                            caret = '^',
                            column = column_number,
                            expected = c,
                        )
                    }
                }
                VerboseErrorKind::Context(s) => write!(
                    &mut result,
                    "{i}: at line {line_number}, in {context}:\n\
             {line}\n\
             {caret:>column$}\n\n",
                    i = i,
                    line_number = line_number,
                    context = s,
                    line = line,
                    caret = '^',
                    column = column_number,
                ),
                VerboseErrorKind::Nom(e) => write!(
                    &mut result,
                    "{i}: at line {line_number}, in {nom_err:?}:\n\
             {line}\n\
             {caret:>column$}\n\n",
                    i = i,
                    line_number = line_number,
                    nom_err = e,
                    line = line,
                    caret = '^',
                    column = column_number,
                ),
            }
        }
        .unwrap();
    }

    result
}
