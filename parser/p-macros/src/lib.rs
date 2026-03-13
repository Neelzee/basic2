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
        LexExpr::Variable(Span::new($ident))
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
            identifier: Span::new($ident),
            variable_type: Some($type),
            value: $value.into(),
        }
    };
    (
        $ident:expr,
        $value:expr
    ) => {
        LexStmt::VariableDeclarationAssignment {
            identifier: Span::new($ident),
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
            identifier: Span::new($ident),
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

#[macro_export]
macro_rules! lai {
    (
        $indexee:expr,
        $indexer:expr
    ) => {
        LexExpr::Op(Box::new(B2Op::Postfix($indexee.into(), Postfix::Index($indexer.into()))))
    };
}