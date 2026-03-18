use crate::lexer::{
    lex_mod::LexModule,
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

    let result = LexModule::parse_program(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    Ok(())
}

#[rstest]
fn test_can_parse_basic_examples(
    #[base_dir = "../assets/basic-examples/"]
    #[mode = str]
    #[files("*.b2")]
    content: &str,
) {
    let input = Span::new(content);

    let result = LexModule::parse_program(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
}

#[rstest]
fn test_can_parse_valid_examples(
    #[base_dir = "../assets/valid-examples/"]
    #[mode = str]
    #[files("*.b2")]
    content: &str,
) {
    let input = Span::new(content);
    let result = LexModule::parse_program(input);

    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
}

#[rstest]
fn test_can_parse_trait_and_generics(
    #[base_dir = "../assets/traits-and-generics/"]
    #[mode = str]
    #[files("*.b2")]
    content: &str,
) {
    let input = Span::new(content);
    let result = LexModule::parse_program(input);

    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
}
