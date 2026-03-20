use crate::lexer::{
    lex_type::{LexMonoType, LexPolyType, LexType},
    utils::{Span, convert_error},
};
use p_macros::ltype;
use rstest::rstest;

#[rstest]
#[case(
    r##"(INT, INT)"##,
    ltype!((INT, INT)),
)]
#[case(
    r##"(INT, (INT, INT))"##,
    ltype!((INT, ltype!((INT, INT)))),
)]
#[case(
    r##"((INT, INT), INT)"##,
    ltype!((ltype!((INT, INT)), INT))
)]
#[case(
    r##"(STR, (STR, (INT, (STR, (INT, (STR, INT))))))"##,
    ltype!((STR, ltype!((STR, ltype!((INT, ltype!((STR, ltype!((INT, ltype!((STR, INT))))))))))))
    ,
)]
fn test_parse_tuple_types(#[case] input: &str, #[case] expected: LexType) {
    let input = Span::new(input);
    let result = LexType::parse_tuple_type(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_parse_function_type() {
    let input = Span::new("{INT => INT}");
    let result = LexType::parse_function_type(input);
    let expected = ltype!(INT => INT);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    "{STR => INT}",
    ltype!(STR => INT)
)]
#[case(
    "{STR => {INT => INT}}",
    ltype!(STR => INT => INT)
)]
#[case(
    "{STR => INT => INT}",
    ltype!(STR => INT => INT)
)]
#[case(
    "{STR => INT => INT => BOOL}",
    ltype!(STR => INT => INT => BOOL)
)]
#[case(
    "{STR => INT => INT => BOOL => {INT => BOOL}}",
    ltype!(STR => INT => INT => BOOL => INT => BOOL)
)]
fn test_parse_multi_function_type(#[case] input: &str, #[case] expected: LexType) {
    let input = Span::new(input);
    let result = LexType::parse_function_type(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_parse_enum() {
    let input = Span::new("Days.Saturday");
    let result = LexType::parse_type(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, ltype!("Days"; "Saturday"));
}
