use crate::{
    common::{B2Op, binop::BinOp, postfix::Postfix, primitive::Primitive},
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::{LexStmt, WhenMatch},
        lex_type::LexType,
        utils::{Span, convert_error},
    },
};
use p_macros::{lbop, leel, lfin, lfne, lg, lprt, lv, lvda, rt};
use rstest::rstest;

#[rstest]
fn test_variable_declaration() {
    let input = r##"LET FOO: NIL;"##;
    let input = Span::new(input);
    let result = LexStmt::parse_variable_declaration(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(
        result.unwrap().1,
        LexStmt::VariableDeclaration {
            identifier: "FOO",
            variable_type: LexType::Nil,
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
        identifier: "FOO",
        variable_type: None,
        value: LexExpr::Literal(Primitive::Str("BAR"))
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
        identifier: "me",
        variable_type: None,
        value: LexExpr::Struct {
            identifier: "Person",
            field_implementations: vec![
                ("firstName", LexExpr::Literal(Primitive::Str("Nils"))),
                ("lastName", LexExpr::Literal(Primitive::Str("Fitjar"))),
                ("age", LexExpr::Literal(Primitive::Int(24))),
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
        identifier: "FOO",
        new_value: LexExpr::Literal(Primitive::Str("BAR")),
        reassignment: None
    }
)]
#[case(
    r##"
    hello += ", World!";
    "##,
    LexStmt::VariableReassignment {
        identifier: "hello",
        new_value: LexExpr::Literal(Primitive::Str(", World!")),
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
            identifier: "FOO",
            new_value: LexExpr::Literal(Primitive::Str("BAR")),
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
                identifier: "FOO",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("BAR"))
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
                identifier: "FOO",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("BAR"))
            },

    LexStmt::If {
        condition: LexExpr::Literal(Primitive::Bool(true)),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("FOO"))
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
                identifier: "FOO",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Str("BAR"))
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
                identifier: "PRINT",
                arguments: vec![LexExpr::Literal(Primitive::Str("Hello!"))]
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
        identifier: "f",
        parameters: Vec::new(),
        generics: Vec::new(),
        return_type: None
    }
)]
#[case(
    r##"DECL foo() : INT;
    "##,
    LexStmt::FunctionDeclaration {
        identifier: "foo",
        parameters: Vec::new(),
        generics: Vec::new(),
        return_type: Some(LexType::Int)
    }
)]
#[case(
    r##"DECL
        bar(INT, INT, INT);
    "##,
    LexStmt::FunctionDeclaration {
        identifier: "bar",
        parameters: vec![LexType::Int, LexType::Int, LexType::Int],
        generics: Vec::new(),
        return_type: None
    }
)]
#[case(
    r##"DECL foobar(INT, INT, INT, STR) : STR;
    "##,
    LexStmt::FunctionDeclaration {
        identifier: "foobar",
        parameters: vec![LexType::Int, LexType::Int, LexType::Int, LexType::Str],
        generics: Vec::new(),
        return_type: Some(LexType::Str)
    }
)]
#[case(
    "DECL map([INT], {INT => INT}): [INT];",
    LexStmt::FunctionDeclaration {
        identifier: "map",
        parameters: vec![LexType::List(Box::new(LexType::Int)), LexType::FnType { input: Box::new(LexType::Int), output: Box::new(LexType::Int) }],
        generics: Vec::new(),
        return_type: Some(LexType::List(Box::new(LexType::Int)))
    }
)]
fn test_parse_function_declaration(#[case] input: &str, #[case] expected: LexStmt) {
    let input = Span::new(input);
    let result = LexStmt::parse_function_declaration(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"IMPL f() DOES END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "f",
        parameters: Vec::new(),
        body: Vec::new(),
    }
)]
#[case(
    r##"IMPL f() DOES
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "f",
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
        identifier: "foo",
        parameters: Vec::new(),
        body: vec![
            LexStmt::VariableDeclarationAssignment { identifier: "BAR", variable_type: None, value: LexExpr::Literal(Primitive::Int(0)) }
        ]
    }
)]
#[case(
    r##"IMPL bar(a, b, c) DOES
        END
    "##,
    LexStmt::FunctionImplementation {
        identifier: "bar",
        parameters: vec![
            ("a", None),
            ("b", None),
            ("c", None)
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
        identifier: "foobar",
        parameters: vec![("a", Some(LexExpr::Literal(Primitive::Str("FOO"))))],
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
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
        identifier: "Person",
        parameters: vec![
            ("firstName", None),
            ("lastName", None),
            ("age", None),
            ("city", None),
            ("postcode", None),
            ("street", None),
        ],
        body: vec![LexStmt::Return {
            value: Some(LexExpr::Tuple(
                Box::new(LexExpr::Variable("firstName")),
                Box::new(LexExpr::Tuple(
                    Box::new(LexExpr::Variable("lastName")),
                    Box::new(LexExpr::Tuple(
                        Box::new(LexExpr::Variable("age")),
                        Box::new(LexExpr::Tuple(
                            Box::new(LexExpr::Variable("city")),
                            Box::new(LexExpr::Tuple(
                                Box::new(LexExpr::Variable("postcode")),
                                Box::new(LexExpr::Tuple(
                                    Box::new(LexExpr::Variable("street")),
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
#[case::map_for_int(
    r##"IMPL map(xs, f) DOES
            WHEN xs THEN
                [] FOLLOWS
                    RETURN [];
                END
                [y, ...ys] FOLLOWS
                    RETURN ADD(f(y), map(ys, f));
                END
            END
        END"##,
    LexStmt::FunctionImplementation {
        identifier: "map", 
        parameters: vec![("xs", None), ("f", None)],
        body: vec![
            LexStmt::WhenStatement {
                identifier: "xs",
                branches: vec![
                    (
                        WhenMatch::EmptyList { condition: None },
                        vec![rt!(leel!())]
                    ),
                    (
                        WhenMatch::VariadicList {
                            identifiers: vec!["y"],
                            remainder: Some("ys"),
                            condition: None
                        },
                        vec![rt!(lfne!("ADD", lfne!("f", lv!("y")), lfne!("map", lv!("ys"), lv!("f"))))]
                    )
                ]
            }
        ]
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
        identifier: "FOOBAR",
        parameters: Vec::new(),
        body: vec![
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
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
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
                variable_type: None,
                value: LexExpr::Literal(Primitive::Int(0))
            },
            LexStmt::VariableDeclarationAssignment {
                identifier: "BAR",
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
        LET FOO: NIL;
    END
    "##,
    LexStmt::Block { body: vec![LexStmt::VariableDeclaration { identifier: "FOO", variable_type: LexType::Nil }] }
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
        identifier: "PRINT",
        arguments: vec![LexExpr::Literal(Primitive::Str("HELLO"))],
    }
)]
#[case(
    r##"INVOKE PRINT("Before: " + global);"##,
    LexStmt::FunctionInvocation {
        identifier: "PRINT",
        arguments: vec![
            LexExpr::Op(Box::new(B2Op::binary(
                LexExpr::Literal(Primitive::Str("Before: ")),
                BinOp::Add,
                LexExpr::Variable("global")
            ).into()))
        ],
    }
)]
#[case(
    r##"INVOKE PRINT("foo is empty");"##,
    lprt!("foo is empty")
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
        identifier: "Foo",
        generics: Vec::new(),
        b2_type: LexType::Int
    }
)]
#[case(
    r##"ALIAS Foo = (INT, INT);"##,
    LexStmt::TypeAlias {
        identifier: "Foo",
        generics: Vec::new(),
        b2_type: LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Int) }
    }
)]
#[case(
    r##"ALIAS Foo = (INT, (INT, INT));"##,
    LexStmt::TypeAlias {
        identifier: "Foo",
        generics: Vec::new(),
        b2_type: LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Int) }) }
    }
)]
#[case(
    r##"ALIAS Foo[T] = (INT, (INT, T));"##,
    LexStmt::TypeAlias {
        identifier: "Foo",
        generics: vec![("T", Vec::new())],
        b2_type: LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::Tuple { fst: Box::new(LexType::Int), snd: Box::new(LexType::TypeVar("T")) }) }
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
        result.clone().unwrap().0,
        result.unwrap().1
    );
}

#[test]
fn test_return_with_nested_index() {
    const INPUT: &str = r##"RETURN p[1][1][0];"##;
    let input = Span::new(INPUT);
    let expected = LexStmt::Return {
        value: Some(LexExpr::Op(Box::new(
            B2Op::postfix(
                LexExpr::Op(Box::new(
                    B2Op::postfix(
                        LexExpr::Op(Box::new(
                            B2Op::postfix(
                                LexExpr::Variable("p"),
                                Postfix::Index(LexExpr::Literal(Primitive::Int(1))),
                            )
                            .into(),
                        )),
                        Postfix::Index(LexExpr::Literal(Primitive::Int(1))),
                    )
                    .into(),
                )),
                Postfix::Index(LexExpr::Literal(Primitive::Int(0))),
            )
            .into(),
        ))),
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
        identifier: "getAge",
        parameters: vec![("p", None)],
        body: vec![LexStmt::Return {
            value: Some(LexExpr::Op(Box::new(
                B2Op::postfix(
                    LexExpr::Op(Box::new(
                        B2Op::postfix(
                            LexExpr::Op(Box::new(
                                B2Op::postfix(
                                    LexExpr::Variable("p"),
                                    Postfix::Index(LexExpr::Literal(Primitive::Int(1))),
                                )
                                .into(),
                            )),
                            Postfix::Index(LexExpr::Literal(Primitive::Int(1))),
                        )
                        .into(),
                    )),
                    Postfix::Index(LexExpr::Literal(Primitive::Int(0))),
                )
                .into(),
            ))),
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
        identifiers: vec!["firstName", "lastName", "age", "city", "postCode", "street"],
        value: LexExpr::Variable("p"),
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
        incrementer: LexExpr::Op(Box::new(B2Op::postfix(lv!("i"), Postfix::Incr).into())),
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
        identifier: "result",
        variable_type: Some(LexType::Str),
        value: LexExpr::Literal(Primitive::Str("")),
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
        identifiers: vec!["a", "b"],
        remainder: Some("cd"),
        value: lv!("list"),
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
        identifiers: vec!["a", "b"],
        value: lv!("struct"),
    };
    let result = LexStmt::parse_struct_unpacking(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_when_branch_empty_list() {
    let input = Span::new(
        r##"[] FOLLOWS
                        INVOKE PRINT("foo is empty");
                    END
                "##,
    );
    let expected = (
        WhenMatch::EmptyList { condition: None },
        vec![lprt!("foo is empty")],
    );
    let result = WhenMatch::parse_empty_list(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_when_stmt() {
    let input = Span::new(
        r##"WHEN foo THEN
                        [] FOLLOWS
                            INVOKE PRINT("foo is empty");
                        END
                        [x] FOLLOWS
                            INVOKE PRINT("foo is a singleton with " + SHOW(x));
                        END
                        _ AND x == y FOLLOWS
                            INVOKE PRINT("foo");
                        END
                        _ FOLLOWS
                            INVOKE PRINT("");
                        END
                        [x, y, ...xs] FOLLOWS
                            INVOKE PRINT("");
                        END
                        [x, y, ...xs] AND x == y FOLLOWS
                        END
                    END
                "##,
    );
    let expected = LexStmt::WhenStatement {
        identifier: "foo",
        branches: vec![
            (
                WhenMatch::EmptyList { condition: None },
                vec![lprt!("foo is empty")],
            ),
            (
                WhenMatch::Singleton {
                    identifier: "x",
                    condition: None,
                },
                vec![lprt!(lbop!(
                    "foo is a singleton with ",
                    BinOp::Add,
                    lfne!("SHOW", lv!("x"))
                ))],
            ),
            (
                WhenMatch::CatchAll {
                    identifier: "_",
                    condition: Some(lbop!(lv!("x"), BinOp::Eq, lv!("y"))),
                },
                vec![lprt!("foo")],
            ),
            (
                WhenMatch::CatchAll {
                    identifier: "_",
                    condition: None,
                },
                vec![lprt!("")],
            ),
            (
                WhenMatch::VariadicList {
                    identifiers: vec!["x", "y"],
                    remainder: Some("xs"),
                    condition: None,
                },
                vec![lprt!((""))],
            ),
            (
                WhenMatch::VariadicList {
                    identifiers: vec!["x", "y"],
                    remainder: Some("xs"),
                    condition: Some(lbop!(lv!("x"), BinOp::Eq, lv!("y"))),
                },
                vec![],
            ),
        ],
    };
    let result = LexStmt::parse_when_statement(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[test]
fn test_variadic_list_branch() {
    let input = Span::new(
        r##"[x, y, ...xs] AND x == y FOLLOWS
                        INVOKE PRINT("");
                    END
                "##,
    );
    let expected = (
        WhenMatch::VariadicList {
            identifiers: vec!["x", "y"],
            remainder: Some("xs"),
            condition: Some(lbop!(lv!("x"), BinOp::Eq, lv!("y"))),
        },
        vec![lprt!("")],
    );
    let result = WhenMatch::parse_variadic_list(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
#[case(
    r##"[] FOLLOWS
        INVOKE PRINT("foo is empty");
    END"##,
    (
        WhenMatch::EmptyList { condition: None },
        vec![lprt!("foo is empty")],
    )
)]
#[case(
    r##"[x] FOLLOWS
        INVOKE PRINT("foo is a singleton with " + SHOW(x));
    END"##,
    (
        WhenMatch::Singleton {
            identifier: "x",
            condition: None,
        },
        vec![lprt!(lbop!(
            "foo is a singleton with ",
            BinOp::Add,
            lfne!("SHOW", lv!("x"))
        ))],
    )
)]
#[case(
    r##"[x, y, ...xs] AND x == y FOLLOWS
    END"##,
    (
        WhenMatch::VariadicList {
            identifiers: vec!["x", "y"],
            remainder: Some("xs"),
            condition: Some(lbop!(lv!("x"), BinOp::Eq, lv!("y"))),
        },
        vec![],
    )
)]
#[case(
    r##"[x, y, ...xs] FOLLOWS
        INVOKE PRINT("");
    END"##,
    (
        WhenMatch::VariadicList {
            identifiers: vec!["x", "y"],
            remainder: Some("xs"),
            condition: None,
        },
        vec![lprt!((""))],
    ),
)]
#[case(
    r##"_ FOLLOWS
        INVOKE PRINT("");
    END"##,
    (
        WhenMatch::CatchAll {
            identifier: "_",
            condition: None,
        },
        vec![lprt!("")],
    ),
)]
#[case(
    r##"_ AND x == y FOLLOWS
        INVOKE PRINT("");
    END"##,
    (
        WhenMatch::CatchAll {
            identifier: "_",
            condition: Some(lbop!(lv!("x"), BinOp::Eq, lv!("y"))),
        },
        vec![lprt!("")],
    ),
)]
#[case(
    r##"_ AND x == y FOLLOWS
    END"##,
    (
        WhenMatch::CatchAll {
            identifier: "_",
            condition: Some(lbop!(lv!("x"), BinOp::Eq, lv!("y"))),
        },
        vec![],
    ),
)]
fn test_parse_list_branch(#[case] input: &str, #[case] expected: (WhenMatch, Vec<LexStmt>)) {
    let input = Span::new(input);
    let result = WhenMatch::parse(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(result.unwrap().1, expected);
}

#[rstest]
fn test_when_statement_example() {
    let input = Span::new(
        r##"WHEN foo THEN
            [] FOLLOWS
                INVOKE PRINT("foo is empty");
            END
            [x] FOLLOWS
                INVOKE PRINT("foo is a singleton with " + SHOW(x));
            END
            [x, y, ...xs] FOLLOWS
                INVOKE PRINT("foo contains atleast two elements, and " + SHOW(LEN(xs)) + " more elements");
            END
            [x, y, ...xs] AND x == y FOLLOWS
                INVOKE PRINT("foo contains atleast two elements, both of which are equal, and " + SHOW(LEN(xs)) + " more elements");
            END
            END
        "##,
    );
    let result = LexStmt::parse_statement(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
}

#[test]
fn test_parse_enum() {
    let input = Span::new(
        r##"ENUMS Days
                    Monday;
                    Tuesday;
                    Wednsday;
                    Thursday;
                    Friday;
                    Saturday;
                    Sunday;
                END
                "##,
    );
    let result = LexStmt::parse_enum_declaration(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
}

#[test]
fn test_parse_enum_with_types() {
    let input = Span::new(
        r##"ENUMS Days
                    Monday(INT);
                    Tuesday;
                    Wednsday;
                    Thursday;
                    Friday;
                    Saturday;
                    Sunday;
                END
                "##,
    );
    let result = LexStmt::parse_enum_declaration(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
}

#[test]
fn test_parse_enum_with_generics() {
    let input = Span::new(
        r##"ENUMS Days[T]
                    Monday(T);
                    Tuesday;
                    Wednsday;
                    Thursday;
                    Friday;
                    Saturday;
                    Sunday;
                END
                "##,
    );
    let result = LexStmt::parse_enum_declaration(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
}

#[test]
fn test_parse_struct_with_generics() {
    let input = Span::new(
        r##"STRUCTURE Days[T] WHERE
                    DECL foo: T;
                END
                "##,
    );
    let result = LexStmt::parse_struct_declaration(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
}

#[test]
fn test_recursive_enum() {
    let input = Span::new(
        r##"ENUMS Days
                    Tuesday(IT);
                END
                "##,
    );
    let result = LexStmt::parse_enum_declaration(input);
    assert!(
        result.is_ok(),
        "{}",
        convert_error(input, result.unwrap_err())
    );
    assert_eq!(
        result.unwrap().1,
        LexStmt::EnumDeclaration {
            identifier: "Days",
            generics: Vec::new(),
            enumerations: vec![("Tuesday", vec![LexType::SelfType])]
        }
    )
}
