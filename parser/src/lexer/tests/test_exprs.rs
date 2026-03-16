use crate::{
    common::{B2Op, binop::BinOp, postfix::Postfix, primitive::Primitive},
    lexer::{
        lex_expr::LexExpr,
        utils::{Span, convert_error},
    },
};
use nom::Parser;
use p_macros::{lai, lbop, lg, lv};
use rstest::rstest;

#[rstest]
#[case::parses_int("123", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_negative_int("-123", LexExpr::Literal(Primitive::Int(-123)))]
#[case::parses_float_with_dot(".123", LexExpr::Literal(Primitive::Float(0.123)))]
#[case::parses_float_with_0("0.123", LexExpr::Literal(Primitive::Float(0.123)))]
#[case::parses_string(r##""string""##, LexExpr::Literal(Primitive::Str("string".to_string())))]
#[case::parses_empty_string(r##""""##, LexExpr::Literal(Primitive::Str("".to_string())))]
#[case::parses_group(r##"("string")"##, LexExpr::Group(Box::new(LexExpr::Literal(Primitive::Str("string".to_string())))))]
#[case::parses_tuple(r##"("string", 123)"##, LexExpr::Tuple(Box::new(LexExpr::Literal(Primitive::Str("string".to_string()))), Box::new(LexExpr::Literal(Primitive::Int(123)))))]
#[case::parses_list(r##"["string", 123]"##, LexExpr::List(vec![LexExpr::Literal(Primitive::Str("string".to_string())), LexExpr::Literal(Primitive::Int(123))]))]
#[case::parses_function_call(r##"foo()"##, LexExpr::FunctionCall { identifier: Span::new("foo"), arguments: Vec::new(), })]
#[case::parses_struct_expr(
    r##"STRUCTURE Person WITH
        IMPL firstName = "Nils";
        IMPL lastName = "Fitjar";
        IMPL age = 24;
    END
    "##,
    LexExpr::Struct {
        identifier: Span::new("Person"),
        field_implementations: vec![
            (Span::new("firstName"), LexExpr::Literal(Primitive::Str("Nils".to_string()))),
            (Span::new("lastName"), LexExpr::Literal(Primitive::Str("Fitjar".to_string()))),
            (Span::new("age"), LexExpr::Literal(Primitive::Int(24))),
        ]
    }
)]
#[case(
    r##""Before: " + global"##,
    LexExpr::Op(Box::new(B2Op::binary(
        LexExpr::Literal(Primitive::Str("Before: ".to_string())),
        BinOp::Add,
        LexExpr::Variable(Span::new("global"))
    ).into()))
)]
#[case(
    "((p * p) <= n)",
    lg!(lbop!(lg!(lbop!(lv!("p"), BinOp::Mul, lv!("p"))), BinOp::Leq, lv!("n")))
)]
fn test_expression_parser(#[case] input: &str, #[case] expected: LexExpr) {
    let result = LexExpr::parse_expr(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case::parses_int("123", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_negative_int("-123", LexExpr::Literal(Primitive::Int(-123)))]
#[case::parses_positive_int("+123", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_space_affix("123 ", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_tab_affix("123 ", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_newline_affix("123 ", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_windows_newline_affix("123\r\n", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_all_whitespace_affix("123 \t\r\n", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_negative_int_with_space_affix("-123 ", LexExpr::Literal(Primitive::Int(-123)))]
#[case::parses_negative_int_with_tab_affix("-123 ", LexExpr::Literal(Primitive::Int(-123)))]
#[case::parses_negative_int_with_newline_affix("-123 ", LexExpr::Literal(Primitive::Int(-123)))]
#[case::parses_negative_int_with_windows_newline_affix("-123\r\n", LexExpr::Literal(Primitive::Int(-123)))]
#[case::parses_negative_int_with_all_whitespace_affix("-123 \t\r\n", LexExpr::Literal(Primitive::Int(-123)))]
#[case::parses_positive_int_with_space_affix("+123 ", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_positive_int_with_tab_affix("+123 ", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_positive_int_with_newline_affix("+123 ", LexExpr::Literal(Primitive::Int(123)))]
#[case::parses_positive_int_with_windows_newline_affix(
    "+123\r\n",
    LexExpr::Literal(Primitive::Int(123))
)]
#[case::parses_positive_int_with_all_whitespace_affix(
    "+123 \t\r\n",
    LexExpr::Literal(Primitive::Int(123))
)]
fn test_primitive_int_parser(#[case] input: &str, #[case] expected: LexExpr) {
    let result = LexExpr::parse_expr(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    "STRUCTURE empty WITH END",
    LexExpr::Struct {
        identifier: Span::new("empty"),
        field_implementations: Vec::new(),
    }
)]
#[case(
    "STRUCTURE FOO WITH
        IMPL bar = 10;
    END",
    LexExpr::Struct {
        identifier: Span::new("FOO"),
        field_implementations: vec![(Span::new("bar"), LexExpr::Literal(Primitive::Int(10)))],
    }
)]
#[case(
    r##"STRUCTURE FOOBAR WITH
        IMPL bar = 10;
        IMPL foo = "";
    END"##,
    LexExpr::Struct {
        identifier: Span::new("FOOBAR"),
        field_implementations: vec![
            (Span::new("bar"), LexExpr::Literal(Primitive::Int(10))),
            (Span::new("foo"), LexExpr::Literal(Primitive::Str(String::new())))
        ],
    }
)]
fn test_struct_expr(#[case] input: &str, #[case] expected: LexExpr) {
    let result = LexExpr::parse_struct.parse(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    "IMPL bar = 10;\nIMPL foo = 10;",
    (
        "bar".to_string(),
        LexExpr::Literal(Primitive::Int(10))
    ),
    "\nIMPL foo = 10;"
)]
fn test_struct_field_impl_parser(
    #[case] input: &str,
    #[case] expected: (String, LexExpr),
    #[case] remainder: &str,
) {
    let result = LexExpr::parse_struct_field.parse(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    let result = result.unwrap();
    let (f, s) = result.1;
    assert_eq!((f.to_string(), s), expected);
    assert_eq!(result.0.to_string(), remainder.to_string());
}

#[rstest]
#[case(r##"foo()"##, LexExpr::FunctionCall { identifier: Span::new("foo"), arguments: Vec::new(), })]
#[case(r##"PRINT("HELLO")"##, LexExpr::FunctionCall { identifier: Span::new("PRINT"), arguments: vec![LexExpr::Literal(Primitive::Str("HELLO".to_string()))], })]
#[case(
    r##"PRINT("Before: " + global)"##,
    LexExpr::FunctionCall {
        identifier: Span::new("PRINT"),
        arguments: vec![
            LexExpr::Op(Box::new(B2Op::binary(
                LexExpr::Literal(Primitive::Str("Before: ".to_string())),
                BinOp::Add,
                LexExpr::Variable(Span::new("global"))
            ).into()))
        ],
    }
)]
fn test_function_call_parser(#[case] input: &str, #[case] expected: LexExpr) {
    let input = Span::new(input);
    let result = LexExpr::parse_function_call.parse(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    "8-2*4",
    lbop!(8, BinOp::Sub, lbop!(2, BinOp::Mul, 4))
)]
#[case(
    "p[0] - 2*4",
    lbop!(lai!(lv!("p"), 0), BinOp::Sub, lbop!(2, BinOp::Mul, 4))
)]
#[case(
    "p[0]",
    lai!(lv!("p"), 0)
)]
fn test_precedence(#[case] input: &str, #[case] expected: LexExpr) {
    let input = Span::new(input);
    let result = LexExpr::parse_precedence(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_struct_field_access() {
    let input = Span::new("me::firstName");
    let result = LexExpr::parse_expr(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(
        result.unwrap().1,
        LexExpr::StructFieldAccessing {
            identifier: Span::new("me"),
            field: Span::new("firstName")
        }
    );
}

#[test]
fn test_parse_expr_does_not_parse_when_branch() {
    let input = Span::new("FOLLOWS");
    let result = LexExpr::parse_expr.parse(input);
    assert!(result.is_err(), "{result:?}");
}
