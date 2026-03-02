use crate::lexer::{lex_type::LexType, utils::Span};
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
    use crate::lexer::utils::convert_error;

    let input = Span::new(input);
    let result = LexType::parse_tuple_type(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}
