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
        LexExpr::Variable($ident)
    };
}

#[macro_export]
macro_rules! lbop {
    (
        $l:expr,
        $op:expr,
        $r:expr
    ) => {
        LexExpr::Op(Box::new(B2Op::binary($l.into(), $op, $r.into()).into()))
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
            identifier: $ident,
            variable_type: Some($type),
            value: $value.into(),
        }
    };
    (
        $ident:expr,
        $value:expr
    ) => {
        LexStmt::VariableDeclarationAssignment {
            identifier: $ident,
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
            identifier: $ident,
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
        LexExpr::Op(Box::new(B2Op::postfix($indexee.into(), Postfix::Index($indexer.into())).into()))
    };
}
#[macro_export]
macro_rules! lfne {
    (
        $ident:expr,
        $($expr:expr),* 
    ) => {
        LexExpr::FunctionCall {
            identifier: $ident,
            arguments: vec![$($expr.into()),*],
        }
    };
}

#[macro_export]
macro_rules! lfin {
    (
        $ident:expr,
        $($expr:expr),* 
    ) => {
        LexStmt::FunctionInvocation {
            identifier: $ident,
            arguments: vec![$($expr.into()),*],
        }
    };
}

#[macro_export]
macro_rules! lprt {
    (
        $($expr:expr),* 
    ) => {
        lfin!("PRINT", $($expr),*)
    };
}