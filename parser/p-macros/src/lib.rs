#[macro_export]
macro_rules! b2 {
    (
        LET ($($v:ident),*) >< $var:ident;
    ) => {
        LexStmt::new_tuple_unpack(vec![$(stringify!($v)),*], LexExpr::Variable(stringify!($var)))
    };
    (
        LET $ident:ident: $type:ident;
    ) => {
        LexStmt::new_var_decl(stringify!($ident), ltype!($type))
    };
    (
        LET $ident:ident: $type:expr;
    ) => {
        LexStmt::new_var_decl(stringify!($ident), $type)
    };
    (
        ALIAS $ident:ident = $type:ident;
    ) => {
        LexStmt::new_type_alias(stringify!($ident), Vec::new(), ltype!($type))
    };
    (
        ALIAS $ident:ident = $type:expr;
    ) => {
        LexStmt::new_type_alias(stringify!($ident), Vec::new(), $type)
    };
    (
        LET $ident:ident = $expr:expr;
    ) => {
        LexStmt::new_var_decl_ass(stringify!($ident), None, $expr.into())
    };
    (
        LET $ident:ident: $type:expr, $expr:expr;
    ) => {
        LexStmt::new_var_decl_ass(stringify!($ident), Some($type), $expr.into())
    };
    (
        LET $ident:ident: $type:ident, $expr:expr;
    ) => {
        LexStmt::new_var_decl_ass(stringify!($ident), Some(ltype!($type)), $expr.into())
    };
    (
        STRUCTURE $ident:ident WITH
            $( IMPL $field:ident = $expr:expr; )*
        END
    ) => {
        LexExpr::Struct {
            identifier: stringify!($ident),
            field_implementations: vec![$( (stringify!($field), $expr.into()), )*]
        }
    };
    (
        DECL $ident:ident( $( $i:ident ),* )$(: $ret:ident)?;
    ) => {
        LexStmt::Decl(Decl::new_fn(stringify!($ident), vec![$( ltype!($i) ),*], Vec::new(), create_return_type!($($ret)?)))
    };
    (
        DECL $ident:ident( $($i:ident),* )$(: $ret:expr)?;
    ) => {
        LexStmt::Decl(Decl::new_fn(stringify!($ident), vec![$( ltype!($i) ),*], Vec::new(),create_return_type!($($ret)?)))
    };
    (
        DECL $ident:ident( $( $i:expr ),* )$(: $ret:ident)?;
    ) => {
        LexStmt::Decl(Decl::new_fn(stringify!($ident), vec![$( $i ),*], Vec::new(), create_return_type!($($ret)?)))
    };
    (
        DECL $ident:ident( $($i:expr),* )$(: $ret:expr)?;
    ) => {
        LexStmt::Decl(Decl::new_fn(stringify!($ident), vec![$( $i ),*], Vec::new(),create_return_type!($($ret)?)))
    };
}

#[macro_export]
macro_rules! create_return_type {
    () => { None };
    ($ret:ident) => { Some(ltype!($ret)) };
    ($ret:expr) => { Some($ret) };
}

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
    (
        $ident:ident
    ) => {
        LexExpr::Variable(stringify!($ident))
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
        LexStmt::new_var_decl_ass(
            $ident,
            Some($type),
            $value.into(),
        )
    };
    (
        $ident:expr,
        $value:expr
    ) => {
        LexStmt::new_var_decl_ass(
            $ident,
            None,
            $value.into(),
        )
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
        LexExpr::Op(Box::new(
            B2Op::postfix($indexee.into(), Postfix::Index($indexer.into())).into(),
        ))
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

#[macro_export]
macro_rules! rt {
    () => {
        LexStmt::Return { value: None }
    };
    (
        $expr:expr
    ) => {
        LexStmt::Return {
            value: Some($expr.into()),
        }
    };
}

#[macro_export]
macro_rules! leel {
    () => {
        LexExpr::List(Vec::new())
    };
}

#[macro_export]
macro_rules! lfnt {
    (
        $input:expr,
        $output:expr
    ) => {
        LexType::FnType {
            input: Box::new($input),
            output: Box::new($output)
        }
    };
    (
        $input:expr,
        $($rem:expr),*
    ) => {
        LexType::FnType {
            input: Box::new($input),
            output: Box::new(lfnt!($($rem),*))
        }
    };
}

#[macro_export]
macro_rules! ltype {
    (
        IT
    ) => {
        LexType::Mono(LexMonoType::SelfType)
    };
    (
        NIL
    ) => {
        LexType::Mono(LexMonoType::Nil)
    };
    (
        STR
    ) => {
        LexType::Mono(LexMonoType::Str)
    };
    (
        INT
    ) => {
        LexType::Mono(LexMonoType::Int)
    };
    (
        BOOL
    ) => {
        LexType::Mono(LexMonoType::Bool)
    };
    (
        $id:expr ; $va:expr
    ) => {
        LexType::Mono(LexMonoType::EnumVariant($id, $va))
    };
    (
        [ $i:ident ]
    ) => {
        LexType::Poly(LexPolyType::List(Box::new(ltype!($i))))
    };
    (
        ( $f:ident, $s:ident )
    ) => {
        LexType::Poly(LexPolyType::Tuple { fst: Box::new(ltype!($f)), snd: Box::new(ltype!($s)) })
    };
    (
        ( $f:ident, $s:expr )
    ) => {
        LexType::Poly(LexPolyType::Tuple { fst: Box::new(ltype!($f)), snd: Box::new($s) })
    };
    (
        ( $f:expr, $s:ident )
    ) => {
        LexType::Poly(LexPolyType::Tuple { fst: Box::new($f), snd: Box::new(ltype!($s)) })
    };
    (
        ( $f:expr, $s:expr )
    ) => {
        LexType::Poly(LexPolyType::Tuple { fst: Box::new($f), snd: Box::new($s) })
    };
    ($i:ident => $o:ident) => {
        LexType::Poly(LexPolyType::FnType {
            input: Box::new(ltype!($i)),
            output: Box::new(ltype!($o)),
        })
    };
    (
        $i:ident =>
        $($o:ident)=>*
    ) => {
        LexType::Poly(LexPolyType::FnType {
            input: Box::new(ltype!($i)),
            output: Box::new(ltype!($($o)=>*)),
        })
    };
}

#[macro_export]
macro_rules! lyv {
    ($t:expr) => {
        LexType::Mono(LexMonoType::TypeVar($t))
    };
}