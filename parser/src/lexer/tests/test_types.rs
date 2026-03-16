use crate::lexer::{
    lex_type::LexType,
    utils::{Span, convert_error},
};
use p_macros::{lfnt, ltype};
use rstest::rstest;

#[rstest]
#[case(
    r##"(INT, INT)"##,
    LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Int) },
)]
#[case(
    r##"(INT, (INT, INT))"##,
    LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Int) }) },
)]
#[case(
    r##"((INT, INT), INT)"##,
    LexType::Tuple { fst: Box::new(LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Int) }), snd: Box::new(LexType::Int) },
)]
#[case(
    r##"(STR, (STR, (INT, (STR, (INT, (STR, INT))))))"##,
    LexType::Tuple {
        fst: Box::new(LexType::Str),
        snd: Box::new(LexType::Tuple {
            fst: Box::new(LexType::Str),
            snd: Box::new(LexType::Tuple {
                fst: Box::new(LexType::Int),
                snd: Box::new(LexType::Tuple {
                        fst: Box::new(LexType::Str),
                        snd: Box::new(LexType::Tuple {
                            fst: Box::new(LexType::Int),
                            snd: Box::new(LexType::Tuple {
                                fst: Box::new(LexType::Str),
                                snd: Box::new(LexType::Int)
                            })
                    })
                })
            })
        })
    },
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
    let expected = LexType::FnType {
        input: Box::new(LexType::Int),
        output: Box::new(LexType::Int),
    };
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
