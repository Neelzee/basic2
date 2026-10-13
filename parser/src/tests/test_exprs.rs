use crate::{
    common::{B2Op, binop::BinOp, postfix::Postfix, primitive::Primitive},
    pexpr::PExpr,
    utils::{Span, convert_error},
};
use common::ToB2;
use nom::Parser;
use p_macros::{hashmap, lai, lbop, lfne, lg, lv};
use rstest::rstest;
use std::collections::HashMap;

#[rstest]
#[case::parses_int("123", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_negative_int("-123", PExpr::Literal(Primitive::Int(-123)))]
#[case::parses_float_with_dot(".123", PExpr::Literal(Primitive::Float(0.123)))]
#[case::parses_float_with_0("0.123", PExpr::Literal(Primitive::Float(0.123)))]
#[case::parses_string(r##""string""##, PExpr::Literal(Primitive::Str("string")))]
#[case::parses_empty_string(r##""""##, PExpr::Literal(Primitive::Str("")))]
#[case::parses_group(
    r##"("string")"##,
    PExpr::Group(Box::new(PExpr::Literal(Primitive::Str("string"))))
)]
#[case::parses_tuple(
    r##"("string", 123)"##,
    PExpr::Tuple(
        Box::new(PExpr::Literal(Primitive::Str("string"))),
        Box::new(PExpr::Literal(Primitive::Int(123)))
    )
)]
#[case::parses_list(r##"["string", 123]"##, PExpr::List(vec![PExpr::Literal(Primitive::Str("string")), PExpr::Literal(Primitive::Int(123))]))]
#[case::parses_function_call(r##"foo()"##, PExpr::FunctionCall { ident: "foo", arguments: Vec::new(), })]
#[case::parses_struct_expr(
    r##"STRUCTURE Person WITH
        IMPL firstName = "Nils";
        IMPL lastName = "Fitjar";
        IMPL age = 24;
    END
    "##,
    PExpr::Struct {
        ident: "Person",
        field_implementations: hashmap![
            ("firstName", PExpr::Literal(Primitive::Str("Nils"))),
            ("lastName", PExpr::Literal(Primitive::Str("Fitjar"))),
            ("age", PExpr::Literal(Primitive::Int(24)))
        ]
    }
)]
#[case(
    r##""Before: " + global"##,
    PExpr::Op(Box::new(B2Op::binary(
        PExpr::Literal(Primitive::Str("Before: ")),
        BinOp::Add,
        PExpr::Variable("global")
    ).into()))
)]
#[case(
    "((p * p) <= n)",
    lg!(lbop!(lg!(lbop!(lv!("p"), BinOp::Mul, lv!("p"))), BinOp::Leq, lv!("n")))
)]
fn test_expression_parser(#[case] input: &str, #[case] expected: PExpr) {
    let result = PExpr::parse_expr(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case::parses_int("123", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_negative_int("-123", PExpr::Literal(Primitive::Int(-123)))]
#[case::parses_positive_int("+123", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_space_affix("123 ", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_tab_affix("123 ", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_newline_affix("123 ", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_windows_newline_affix("123\r\n", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_int_with_all_whitespace_affix("123 \t\r\n", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_negative_int_with_space_affix("-123 ", PExpr::Literal(Primitive::Int(-123)))]
#[case::parses_negative_int_with_tab_affix("-123 ", PExpr::Literal(Primitive::Int(-123)))]
#[case::parses_negative_int_with_newline_affix("-123 ", PExpr::Literal(Primitive::Int(-123)))]
#[case::parses_negative_int_with_windows_newline_affix("-123\r\n", PExpr::Literal(Primitive::Int(-123)))]
#[case::parses_negative_int_with_all_whitespace_affix("-123 \t\r\n", PExpr::Literal(Primitive::Int(-123)))]
#[case::parses_positive_int_with_space_affix("+123 ", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_positive_int_with_tab_affix("+123 ", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_positive_int_with_newline_affix("+123 ", PExpr::Literal(Primitive::Int(123)))]
#[case::parses_positive_int_with_windows_newline_affix(
    "+123\r\n",
    PExpr::Literal(Primitive::Int(123))
)]
#[case::parses_positive_int_with_all_whitespace_affix(
    "+123 \t\r\n",
    PExpr::Literal(Primitive::Int(123))
)]
fn test_primitive_int_parser(#[case] input: &str, #[case] expected: PExpr) {
    let result = PExpr::parse_expr(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    "STRUCTURE empty WITH END",
    PExpr::Struct {
        ident: "empty",
        field_implementations: HashMap::new(),
    }
)]
#[case(
    "STRUCTURE FOO WITH
        IMPL bar = 10;
    END",
    PExpr::Struct {
        ident: "FOO",
        field_implementations: hashmap![("bar", PExpr::Literal(Primitive::Int(10)))],
    }
)]
#[case(
    r##"STRUCTURE FOOBAR WITH
        IMPL bar = 10;
        IMPL foo = "";
    END"##,
    PExpr::Struct {
        ident: "FOOBAR",
        field_implementations: hashmap![
            ("bar", PExpr::Literal(Primitive::Int(10))),
            ("foo", PExpr::Literal(Primitive::Str("")))
        ],
    }
)]
fn test_struct_expr(#[case] input: &str, #[case] expected: PExpr) {
    let result = PExpr::parse_struct.parse(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    "IMPL bar = 10;\nIMPL foo = 10;",
    (
        "bar",
        PExpr::Literal(Primitive::Int(10))
    ),
    "\nIMPL foo = 10;"
)]
fn test_struct_field_impl_parser(
    #[case] input: &str,
    #[case] expected: (&str, PExpr),
    #[case] remainder: &str,
) {
    let result = PExpr::parse_struct_field.parse(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    let result = result.unwrap();
    let (f, s) = result.1;
    assert_eq!((f, s), expected);
    assert_eq!(result.0.to_string(), remainder);
}

#[rstest]
#[case(r##"foo()"##, PExpr::FunctionCall { ident: "foo", arguments: Vec::new(), })]
#[case(r##"PRINT("HELLO")"##, PExpr::FunctionCall { ident: "PRINT", arguments: vec![PExpr::Literal(Primitive::Str("HELLO"))], })]
#[case(
    r##"PRINT("Before: " + global)"##,
    PExpr::FunctionCall {
        ident: "PRINT",
        arguments: vec![
            PExpr::Op(Box::new(B2Op::binary(
                PExpr::Literal(Primitive::Str("Before: ")),
                BinOp::Add,
                PExpr::Variable("global")
            ).into()))
        ],
    }
)]
fn test_function_call_parser(#[case] input: &str, #[case] expected: PExpr) {
    let input = Span::new(input);
    let result = PExpr::parse_function_call.parse(input);
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
#[case(
    r##""foo contains atleast two elements, both of which are equal, and " + SHOW(LEN(xs)) + " more elements""##,
    lbop!(lbop!("foo contains atleast two elements, both of which are equal, and ", BinOp::Add, lfne!("SHOW", lfne!("LEN", lv!("xs")))), BinOp::Add, " more elements")
)]
fn test_precedence(#[case] input: &str, #[case] expected: PExpr) {
    let input = Span::new(input);
    let result = PExpr::parse_precedence(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    let result = result.unwrap().1;
    assert_eq!(
        result,
        expected,
        "{} != {}",
        result.to_b2(),
        expected.to_b2()
    );
}

#[test]
fn test_struct_field_access() {
    let input = Span::new("me::firstName");
    let result = PExpr::parse_expr(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(
        result.unwrap().1,
        PExpr::StructFieldAccessing {
            ident: "me",
            field: "firstName"
        }
    );
}

#[test]
fn test_parse_expr_does_not_parse_when_branch() {
    let input = Span::new("FOLLOWS");
    let result = PExpr::parse_expr.parse(input);
    assert!(result.is_err(), "{result:?}");
}

#[test]
fn test_binary_expression() {
    let input = Span::new("x == y");
    let result = PExpr::parse_expr(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, lbop!(lv!("x"), BinOp::Eq, lv!("y")));
}

#[test]
fn test_parse_enum_values() {
    let input = Span::new("Days.Sun(10)");
    let result = PExpr::parse_expr(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(
        result.unwrap().1,
        PExpr::Enum {
            ident: "Days",
            instance: "Sun",
            values: vec![10.into()]
        }
    );
}
