#[macro_export]
macro_rules! lif {
    (
        $cond:expr,
        $( $body:expr );*
    ) => {
        LexStmt::If {
            condition: $cond.into(),
            body: vec![
                $( $body, )*
            ]
        }
    };
}

#[macro_export]
macro_rules! lv {
    (
        $ident:expr
    ) => {
        LexExpr::Variable($ident.to_string())
    };
}

#[macro_export]
macro_rules! lbop {
    (
        $l:expr,
        $op:expr,
        $r:expr
    ) => {
        LexExpr::Op(Box::new(B2Op::Binary($l.into(), $op, $r.into())))
    };
}

#[macro_export]
macro_rules! lvda {
    (
        $ident:expr,
        $type:ty,
        $value:expr
    ) => {
        LexStmt::VariableDeclarationAssignment {
            identifier: $ident.to_string(),
            variable_type: Some($type),
            value: $value.into(),
        }
    };
    (
        $ident:expr,
        $value:expr
    ) => {
        LexStmt::VariableDeclarationAssignment {
            identifier: $ident.to_string(),
            variable_type: None,
            value: $value.into(),
        }
    };
}

#[macro_export]
macro_rules! lvra {
    (
        $ident:expr,
        $value:expr
    ) => {
        LexStmt::VariableReassignment {
            identifier: $ident.to_string(),
            reassignment: None,
            new_value: $value.into(),
        }
    };
}

#[macro_export]
macro_rules! lg {
    (
        $expr:expr
    ) => {
        LexExpr::Group(Box::new($expr.into()))
    };
}
