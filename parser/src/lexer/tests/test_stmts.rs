use crate::{
    common::{B2Op, binop::BinOp, postfix::Postfix, primitive::Primitive},
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::LexStmt,
        lex_type::LexType,
        utils::{Span, convert_error},
    },
};
use p_macros::{lbop, lg, lv, lvda};
use rstest::rstest;

#[rstest]
fn test_variable_declaration() {
    let input = r##"LET FOO;"##;
    let result = LexStmt::parse_variable_declaration(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(
        result.unwrap().1,
        LexStmt::VariableDeclaration {
            identifier: Span::new("FOO"),
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
#[case(
    r##"LET FOO = "BAR";"##,
    LexStmt::VariableDeclarationAssignment {
        identifier: Span::new("FOO"),
        variable_type: None,
        value: LexExpr::Literal(Primitive::Str("BAR".to_string()))
    }
)]
#[case(
    r##"
    LET me = STRUCTURE Person WITH
        IMPL firstName = "Nils";
        IMPL lastName = "Fitjar";
        IMPL age = 24;
    END;
    "##,
    LexStmt::VariableDeclarationAssignment {
        identifier: Span::new("me"),
        variable_type: None,
        value: LexExpr::Struct {
            identifier: Span::new("Person"),
            field_implementations: vec![
                (Span::new("firstName"), LexExpr::Literal(Primitive::Str("Nils".to_string()))),
                (Span::new("lastName"), LexExpr::Literal(Primitive::Str("Fitjar".to_string()))),
                (Span::new("age"), LexExpr::Literal(Primitive::Int(24))),
            ]
        }
    }
)]
fn test_variable_declaration_assignment(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_variable_declaration_assignment(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
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
#[case(
    r##"FOO = "BAR";"##,
    LexStmt::VariableReassignment {
        identifier: Span::new("FOO"),
        new_value: LexExpr::Literal(Primitive::Str("BAR".to_string())),
        reassignment: None
    }
)]
#[case(
    r##"
    hello += ", World!";
    "##,
    LexStmt::VariableReassignment {
        identifier: Span::new("hello"),
        new_value: LexExpr::Literal(Primitive::Str(", World!".to_string())),
        reassignment: Some(BinOp::Add),
    }
)]
fn test_variable_reassignment(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_variable_reassignment(Span::new(input));
    assert!(result.is_ok(), "{:?}", result.unwrap_err());
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
fn test_variable_reassignment_addition() {
    let input = r##"FOO += "BAR";"##;
    let result = LexStmt::parse_variable_reassignment(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(
        result.unwrap().1,
        LexStmt::VariableReassignment {
            identifier: Span::new("FOO"),
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
                identifier: Span::new("FOO"),
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
                identifier: Span::new("FOO"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("BAR".to_string()))
            },

    LexStmt::If {
        condition: LexExpr::Literal(Primitive::Bool(true)),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
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
                identifier: Span::new("FOO"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("BAR".to_string()))
            }
        ]
    }
)]
#[case(
    r##"
    WHILE (TRUE) DO
        INVOKE PRINT("Hello!");
        BREAK;
    END
    "##,
    LexStmt::While {
        condition: LexExpr::Group(Box::new(LexExpr::Literal(Primitive::Bool(true)))),
        body: vec![
            LexStmt::FunctionInvocation {
                identifier: Span::new("PRINT"),
                arguments: vec![LexExpr::Literal(Primitive::Str("Hello!".to_string()))]
            },
            LexStmt::Break,
        ]
    }
)]
#[case(
    r##"WHILE ((p * p) <= n) DO
        END
    "##,
    LexStmt::While {
        condition: lg!(lbop!(lg!(lbop!(lv!("p"), BinOp::Mul, lv!("p"))), BinOp::Leq, lv!("n"))),
        body: Vec::new(),
    }
)]
fn test_parse_while(#[case] input: &str, #[case] expected: LexStmt) {
    let result = LexStmt::parse_while_statement(Span::new(input));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"DECL f();
    "##,
    LexStmt::FunctionDeclaration {
        identifier: Span::new("f"),
        parameters: Vec::new(),
        return_type: None
    }
)]
#[case(
    r##"DECL foo() : INT;
    "##,
    LexStmt::FunctionDeclaration {
        identifier: Span::new("foo"),
        parameters: Vec::new(),
        return_type: Some(LexType::Int)
    }
)]
#[case(
    r##"DECL
        bar(INT, INT, INT);
    "##,
    LexStmt::FunctionDeclaration {
        identifier: Span::new("bar"),
        parameters: vec![LexType::Int, LexType::Int, LexType::Int],
        return_type: None
    }
)]
#[case(
    r##"DECL foobar(INT, INT, INT, STR) : STR;
    "##,
    LexStmt::FunctionDeclaration {
        identifier: Span::new("foobar"),
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
    r##"IMPL f() DOES END
    "##,
    LexStmt::FunctionImplementation {
        identifier: Span::new("f"),
        parameters: Vec::new(),
        body: Vec::new(),
    }
)]
#[case(
    r##"IMPL f() DOES
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: Span::new("f"),
        parameters: Vec::new(),
        body: Vec::new(),
    }
)]
#[case(
    r##"IMPL foo() DOES
            LET BAR = 0;
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: Span::new("foo"),
        parameters: Vec::new(),
        body: vec![
            LexStmt::VariableDeclarationAssignment { identifier: Span::new("BAR"), variable_type: None, value: LexExpr::Literal(Primitive::Int(0)) }
        ]
    }
)]
#[case(
    r##"IMPL bar(a, b, c) DOES
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: Span::new("bar"),
        parameters: vec![
            (Span::new("a"), None),
            (Span::new("b"), None),
            (Span::new("c"), None)
        ],
        body: Vec::new(),
    }
)]
#[case(
    r##"IMPL foobar(a = "FOO") DOES
            LET BAR = 0;
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: Span::new("foobar"),
        parameters: vec![(Span::new("a"), Some(LexExpr::Literal(Primitive::Str("FOO".to_string()))))],
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            }
        ]
    }
)]
#[case(
    r##"IMPL Person(firstName, lastName, age, city, postcode, street) DOES
        RETURN (firstName, (lastName, (age, (city, (postcode, (street, -1))))));
    END
    "##,
    LexStmt::FunctionImplementation {
        identifier: Span::new("Person"),
        parameters: vec![
            (Span::new("firstName"), None),
            (Span::new("lastName"), None),
            (Span::new("age"), None),
            (Span::new("city"), None),
            (Span::new("postcode"), None),
            (Span::new("street"), None),
        ],
        body: vec![LexStmt::Return {
            value: Some(LexExpr::Tuple(
                Box::new(LexExpr::Variable(Span::new("firstName"))),
                Box::new(LexExpr::Tuple(
                    Box::new(LexExpr::Variable(Span::new("lastName"))),
                    Box::new(LexExpr::Tuple(
                        Box::new(LexExpr::Variable(Span::new("age"))),
                        Box::new(LexExpr::Tuple(
                            Box::new(LexExpr::Variable(Span::new("city"))),
                            Box::new(LexExpr::Tuple(
                                Box::new(LexExpr::Variable(Span::new("postcode"))),
                                Box::new(LexExpr::Tuple(
                                    Box::new(LexExpr::Variable(Span::new("street"))),
                                    Box::new(LexExpr::Literal(Primitive::Int(-1)))
                                ))
                            ))
                        ))
                    ))
                ))
            ))
        }]
    }
)]
fn test_parse_function_implementation(#[case] input: &str, #[case] expected: LexStmt) {
    let input = Span::new(input);
    let result = LexStmt::parse_function_implementation(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"IMPL FOOBAR() DOES
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
        LET BAR = 0;
    END
    "##,
    LexStmt::FunctionImplementation {
        identifier: Span::new("FOOBAR"),
        parameters: Vec::new(),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
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
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: Span::new("BAR"),
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
    let input = Span::new(input);
    let result = LexStmt::parse_statement(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
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
    LexStmt::Block { body: vec![LexStmt::VariableDeclaration { identifier: Span::new("FOO"), variable_type: None }] }
)]
fn test_parse_block_statements(#[case] input: &str, #[case] expected: LexStmt) {
    let input = Span::new(input);
    let result = LexStmt::parse_block_statement(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"INVOKE PRINT("HELLO");"##,
    LexStmt::FunctionInvocation {
        identifier: Span::new("PRINT"),
        arguments: vec![LexExpr::Literal(Primitive::Str("HELLO".to_string()))],
    }
)]
#[case(
    r##"INVOKE PRINT("Before: " + global);"##,
    LexStmt::FunctionInvocation {
        identifier: Span::new("PRINT"),
        arguments: vec![
            LexExpr::Op(Box::new(B2Op::Binary(
                LexExpr::Literal(Primitive::Str("Before: ".to_string())),
                BinOp::Add,
                LexExpr::Variable(Span::new("global"))
            )))
        ],
    }
)]
fn test_parse_function_invocation(#[case] input: &str, #[case] expected: LexStmt) {
    let input = Span::new(input);
    let result = LexStmt::parse_function_invocation(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"ALIAS Foo = INT;"##,
    LexStmt::TypeAlias {
        identifier: Span::new("Foo"),
        b2_type: LexType::Int
    }
)]
#[case(
    r##"ALIAS Foo = (INT, INT);"##,
    LexStmt::TypeAlias {
        identifier: Span::new("Foo"),
        b2_type: LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Int) }
    }
)]
#[case(
    r##"ALIAS Foo = (INT, (INT, INT));"##,
    LexStmt::TypeAlias {
        identifier: Span::new("Foo"),
        b2_type: LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Int) }) }
    }
)]
fn test_parse_type_alias(#[case] input: &str, #[case] expected: LexStmt) {
    let input = Span::new(input);
    let result = LexStmt::parse_type_alias(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case::alias(r##"ALIAS Foo = (INT, (INT, INT))"##)]
#[case::declare_variable(r##"LET foo"##)]
#[case::variable_assignment(r##"LET foo = "bar""##)]
#[case::variable_reassignment(r##"foo = "bar""##)]
#[case::variable_reassignment_add(r##"foo += "bar""##)]
#[case::import(r##"USE Foobar"##)]
#[case::function_invocation(r##"INVOKE Foo()"##)]
fn test_parse_stmts_fail_with_missing_end_stmt_kw(#[case] input: &str) {
    let input = Span::new(input);
    let result = LexStmt::parse_statement(input);
    assert!(
        result.is_err(),
        "Remainding: {}, result: {:?}",
        result.as_ref().unwrap().0.to_string(),
        result.unwrap().1
    );
}

#[test]
fn test_return_with_nested_index() {
    const INPUT: &str = r##"RETURN p[1][1][0];"##;
    let input = Span::new(INPUT);
    let expected = LexStmt::Return {
        value: Some(LexExpr::Op(Box::new(B2Op::Postfix(
            LexExpr::Op(Box::new(B2Op::Postfix(
                LexExpr::Op(Box::new(B2Op::Postfix(
                    LexExpr::Variable(Span::new("p")),
                    Postfix::Index(LexExpr::Literal(Primitive::Int(1))),
                ))),
                Postfix::Index(LexExpr::Literal(Primitive::Int(1))),
            ))),
            Postfix::Index(LexExpr::Literal(Primitive::Int(0))),
        )))),
    };
    let result = LexStmt::parse_return(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_function_impl_get_age() {
    const INPUT: &str = r##"IMPL getAge(p) DOES
                RETURN p[1][1][0];
            END"##;
    let input = Span::new(INPUT);
    let expected = LexStmt::FunctionImplementation {
        identifier: Span::new("getAge"),
        parameters: vec![(Span::new("p"), None)],
        body: vec![LexStmt::Return {
            value: Some(LexExpr::Op(Box::new(B2Op::Postfix(
                LexExpr::Op(Box::new(B2Op::Postfix(
                    LexExpr::Op(Box::new(B2Op::Postfix(
                        LexExpr::Variable(Span::new("p")),
                        Postfix::Index(LexExpr::Literal(Primitive::Int(1))),
                    ))),
                    Postfix::Index(LexExpr::Literal(Primitive::Int(1))),
                ))),
                Postfix::Index(LexExpr::Literal(Primitive::Int(0))),
            )))),
        }],
    };
    let result = LexStmt::parse_function_implementation(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_function_unpacking() {
    const INPUT: &str = r##"LET (firstName, lastName, age, city, postCode, street) >< p;"##;
    let input = Span::new(INPUT);
    let expected = LexStmt::TupleUnpacking {
        identifiers: vec![
            Span::new("firstName"),
            Span::new("lastName"),
            Span::new("age"),
            Span::new("city"),
            Span::new("postCode"),
            Span::new("street"),
        ],
        value: LexExpr::Variable(Span::new("p")),
    };
    let result = LexStmt::parse_tuple_unpacking(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_for_loop() {
    const INPUT: &str = r##"FOR (LET i = 2; i <= (n + 1); i++;) THEN
            END
        "##;
    let input = Span::new(INPUT);
    let expected = LexStmt::For {
        start_stmt: Box::new(lvda!("i", 2)),
        condition: lbop!(lv!("i"), BinOp::Leq, lg!(lbop!(lv!("n"), BinOp::Add, 1))),
        incrementer: LexExpr::Op(Box::new(B2Op::Postfix(lv!("i"), Postfix::Incr))),
        body: Vec::new(),
    };
    let result = LexStmt::parse_for_statement(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_variable_declaration_assignment_with_type() {
    const INPUT: &str = r##"LET result: STR = "";"##;
    let input = Span::new(INPUT);
    let expected = LexStmt::VariableDeclarationAssignment {
        identifier: Span::new("result"),
        variable_type: Some(LexType::Str),
        value: LexExpr::Literal(Primitive::Str(String::new())),
    };
    let result = LexStmt::parse_variable_declaration_assignment(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_list_reasignment() {
    let input = Span::new(r##"prime[i] = FALSE;"##);
    let expected = LexStmt::ListReassignment {
        indexee: lv!("prime"),
        index: lv!("i"),
        reassignment: None,
        new_value: false.into(),
    };
    let result = LexStmt::parse_list_reassignment(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_list_unpacking() {
    let input = Span::new(r##"LET [a, b, ...cd] >< list;"##);
    let expected = LexStmt::ListUnpacking {
        identifiers: vec![Span::new("a"), Span::new("b")],
        remainder: Some(Span::new("cd")),
        value: lv!("list")
    };
    let result = LexStmt::parse_list_unpacking(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_struct_unpacking() {
    let input = Span::new(r##"LET [::a, ::b] >< struct;"##);
    let expected = LexStmt::StructUnpacking {
        identifiers: vec![Span::new("a"), Span::new("b")],
        value: lv!("struct")
    };
    let result = LexStmt::parse_struct_unpacking(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}