use crate::lexer::{
    lex_prog::LexProgram,
    utils::{Span, consts::test_const::EMPTY_PROGRAM_PATH, convert_error},
};
use anyhow::{Context, Result};
use rstest::rstest;
use std::{fs::File, io::Read, path::PathBuf};

#[rstest]
fn test_can_parse_empty_program() -> Result<()> {
    let path: PathBuf = EMPTY_PROGRAM_PATH.into();
    let path = path.canonicalize().with_context(|| format!("{path:?}"))?;
    let mut file = File::open(&path).with_context(|| format!("{path:?}"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let input = Span::new(&buf);

    let result = LexProgram::parse_program(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    Ok(())
}

#[rstest]
fn test_can_parse_basic_examples(
    #[files("../assets/basic-examples/*.b2")] path: PathBuf,
) -> Result<()> {
    let mut file = File::open(&path).with_context(|| format!("{path:?}"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let input = Span::new(&buf);

    let result = LexProgram::parse_program(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );

    Ok(())
}

#[rstest]
fn test_can_parse_valid_examples(
    #[files("../assets/valid-examples/*.b2")] path: PathBuf,
) -> Result<()> {
    let mut file = File::open(&path).with_context(|| format!("{path:?}"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let input = Span::new(&buf);
    let result = LexProgram::parse_program(input);

    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );

    Ok(())
}
