use crate::{
    common::{BinOp, Primitive},
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::LexStmt,
        lex_type::LexType,
        utils::{
            Span,
            consts::{BLOCK_STATEMENT_END_KW, BLOCK_STATEMENT_START_KW},
            helper_parsers::parse_comment,
        },
    },
};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::opt,
    multi::many0,
    sequence::{pair, preceded},
};
use rstest::rstest;

#[rstest]
fn test_variable_declaration() {
    let input = r##"LET FOO;"##;
    let result = LexStmt::parse_variable_declaration(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(
        result.unwrap().1,
        LexStmt::VariableDeclaration {
            identifier: "FOO".to_string(),
            variable_type: None,
        }
    );
}

#[rstest]
fn test_variable_declaration_fails_with_missing_end_stmt_kw() {
    let input = r##"LET FOO"##;
    let result = LexStmt::parse_variable_declaration(Span::new(input));
    assert!(result.is_err(), "{result:?}");
}

#[rstest]
fn test_variable_declaration_assignment() {
    let input = r##"LET FOO = "BAR";"##;
    let result = LexStmt::parse_variable_declaration_assignment(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(
        result.unwrap().1,
        LexStmt::VariableDeclarationAssignment {
            identifier: "FOO".to_string(),
            variable_type: None,
            value: LexExpr::Literal(Primitive::Str("BAR".to_string()))
        }
    );
}

#[rstest]
fn test_variable_declaration_assignment_fails_with_missing_end_stmt_kw() {
    let input = r##"LET FOO = "BAR""##;
    let result = LexStmt::parse_variable_declaration_assignment(Span::new(input));
    assert!(result.is_err(), "{result:?}");
}

#[rstest]
fn test_variable_declaration_assignment_consumes_end_stmt_kw() {
    let input = r##"LET FOO = "BAR";"##;
    let result = LexStmt::parse_variable_declaration_assignment(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_ne!(result.unwrap().0.to_string(), ";");
}

#[rstest]
fn test_variable_reassignment() {
    let input = r##"FOO = "BAR";"##;
    let result = LexStmt::parse_variable_reassignment(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(
        result.unwrap().1,
        LexStmt::VariableReassignment {
            identifier: "FOO".to_string(),
            new_value: LexExpr::Literal(Primitive::Str("BAR".to_string())),
            reassignment: None
        }
    );
}

#[rstest]
fn test_variable_reassignment_addition() {
    let input = r##"FOO += "BAR";"##;
    let result = LexStmt::parse_variable_reassignment(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(
        result.unwrap().1,
        LexStmt::VariableReassignment {
            identifier: "FOO".to_string(),
            new_value: LexExpr::Literal(Primitive::Str("BAR".to_string())),
            reassignment: Some(BinOp::Add)
        }
    );
}

#[rstest]
fn test_variable_reassignment_fails_with_missing_end_stmt_kw() {
    let input = r##"FOO = "BAR""##;
    let result = LexStmt::parse_variable_reassignment(Span::new(input));
    assert!(result.is_err(), "{result:?}");
}

#[rstest]
#[case(
    r##"
        IF TRUE THEN
        FI
    "##,
    LexStmt::If { condition: LexExpr::Literal(Primitive::Bool(true)), body: Vec::new() }
)]
#[case(
    r##"
        IF FALSE THEN
        FI
    "##,
    LexStmt::If { condition: LexExpr::Literal(Primitive::Bool(false)), body: Vec::new() }
)]
#[case(
    r##"
        IF TRUE THEN
            LET FOO = "BAR";
        FI
    "##,
    LexStmt::If {
        condition: LexExpr::Literal(Primitive::Bool(true)),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "FOO".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("BAR".to_string()))
            }
        ]
    }
)]
#[case::nested_if_statement(
    r##"
        IF TRUE THEN
            LET FOO = "BAR";
            IF TRUE THEN
                LET BAR = "FOO";
            FI
        FI
    "##,
    LexStmt::If {
        condition: LexExpr::Literal(Primitive::Bool(true)),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "FOO".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("BAR".to_string()))
            },

    LexStmt::If {
        condition: LexExpr::Literal(Primitive::Bool(true)),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("FOO".to_string()))
            }
        ]
    }
        ]
    }
)]
fn test_parse_if(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_if_statement(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"
        WHILE TRUE DO
        END
    "##,
    LexStmt::While { condition: LexExpr::Literal(Primitive::Bool(true)), body: Vec::new() }
)]
#[case(
    r##"
        WHILE FALSE DO
        END
    "##,
    LexStmt::While { condition: LexExpr::Literal(Primitive::Bool(false)), body: Vec::new() }
)]
#[case(
    r##"
        WHILE TRUE DO
            LET FOO = "BAR";
        END
    "##,
    LexStmt::While {
        condition: LexExpr::Literal(Primitive::Bool(true)),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "FOO".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("BAR".to_string()))
            }
        ]
    }
)]
fn test_parse_while(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_while_statement(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"
        DECL f();
    "##,
    LexStmt::FunctionDeclaration {
        identifier: "f".to_string(),
        parameters: Vec::new(),
        return_type: None
    }
)]
#[case(
    r##"
        DECL foo() : INT;
    "##,
    LexStmt::FunctionDeclaration {
        identifier: "foo".to_string(),
        parameters: Vec::new(),
        return_type: Some(LexType::Int)
    }
)]
#[case(
    r##"
        DECL bar(INT, INT, INT);
    "##,
    LexStmt::FunctionDeclaration {
        identifier: "bar".to_string(),
        parameters: vec![LexType::Int, LexType::Int, LexType::Int],
        return_type: None
    }
)]
#[case(
    r##"
        DECL foobar(INT, INT, INT, STR) : STR;
    "##,
    LexStmt::FunctionDeclaration {
        identifier: "foobar".to_string(),
        parameters: vec![LexType::Int, LexType::Int, LexType::Int, LexType::Str],
        return_type: Some(LexType::Str)
    }
)]
fn test_parse_function_declaration(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_function_declaration(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"
        IMPL f() DOES END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "f".to_string(),
        parameters: Vec::new(),
        body: Vec::new(),
    }
)]
#[case(
    r##"
        IMPL f() DOES
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "f".to_string(),
        parameters: Vec::new(),
        body: Vec::new(),
    }
)]
#[case(
    r##"
        IMPL foo() DOES
            LET BAR = 0;
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "foo".to_string(),
        parameters: Vec::new(),
        body: vec![
            LexStmt::VariableDeclarationAssignment { identifier: "BAR".to_string(), variable_type: None, value: LexExpr::Literal(Primitive::Int(0)) }
        ]
    }
)]
#[case(
    r##"
        IMPL bar(a, b, c) DOES
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "bar".to_string(),
        parameters: vec![("a".to_string(), None), ("b".to_string(), None), ("c".to_string(), None)],
        body: Vec::new(),
    }
)]
#[case(
    r##"
        IMPL foobar(a = "FOO") DOES
            LET BAR = 0;
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "foobar".to_string(),
        parameters: vec![("a".to_string(), Some(LexExpr::Literal(Primitive::Str("FOO".to_string()))))],
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            }
        ]
    }
)]
fn test_parse_function_implementation(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_function_implementation(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"
    IMPL FOOBAR() DOES
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
    END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "FOOBAR".to_string(),
        parameters: Vec::new(),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
        ]
    }
)]
#[case(
    r##"
    IF TRUE THEN
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
    FI
    "##,
    LexStmt::If {
        condition: LexExpr::Literal(Primitive::Bool(true)),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR".to_string(),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
        ]
    }
)]
#[case(
    r##"
    DO END
    "##,
    LexStmt::Block { body: Vec::new() }
)]
fn test_parse_multiline_statements(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_statement(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"
    DO END
    "##,
    LexStmt::Block { body: Vec::new() }
)]
#[case(
    r##"
    DO
    END
    "##,
    LexStmt::Block { body: Vec::new() }
)]
#[case(
    r##"
    DO
        # COmments
    END
    "##,
    LexStmt::Block { body: Vec::new() }
)]
#[case(
    r##"
    DO
        LET FOO;
    END
    "##,
    LexStmt::Block { body: vec![LexStmt::VariableDeclaration { identifier: "FOO".to_string(), variable_type: None }] }
)]
fn test_parse_block_statements(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_block_statement(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn parse_function_invocation() {
    let input = r##"
    INVOKE PRINT("HELLO");
    "##;
    let result = LexStmt::parse_function_invocation(Span::new(input));
    dbg!(result.as_ref());
    assert!(result.is_ok(), "{result:?}");
}
