#[macro_export]
macro_rules! b2 {
    (
        LET ($($v:ident),*) >< $var:ident;
    ) => {
        PStmt::new_tuple_unpack(vec![$(stringify!($v)),*], PExpr::Variable(stringify!($var)))
    };
    (
        LET $ident:ident: $type:ident;
    ) => {
        PStmt::new_var_decl(stringify!($ident), ltype!($type))
    };
    (
        LET $ident:ident: $type:expr;
    ) => {
        PStmt::new_var_decl(stringify!($ident), $type)
    };
    (
        ALIAS $ident:ident = $type:ident;
    ) => {
        PStmt::new_type_alias(stringify!($ident), Vec::new(), ltype!($type))
    };
    (
        ALIAS $ident:ident = $type:expr;
    ) => {
        PStmt::new_type_alias(stringify!($ident), Vec::new(), $type)
    };
    (
        LET $ident:ident = $expr:expr;
    ) => {
        PStmt::new_var_decl_ass(stringify!($ident), None, $expr.into())
    };
    (
        LET $ident:ident: $type:expr, $expr:expr;
    ) => {
        PStmt::new_var_decl_ass(stringify!($ident), Some($type), $expr.into())
    };
    (
        LET $ident:ident: $type:ident, $expr:expr;
    ) => {
        PStmt::new_var_decl_ass(stringify!($ident), Some(ltype!($type)), $expr.into())
    };
    (
        STRUCTURE $ident:ident WITH
            $( IMPL $field:ident = $expr:expr; )*
        END
    ) => {
        PExpr::Struct {
            ident: stringify!($ident),
            field_implementations: hashmap![$( (stringify!($field), $expr.into()), )*]
        }
    };
    (
        DECL $ident:ident( $( $i:ident ),* )$(: $ret:ident)?;
    ) => {
        PStmt::Decl(Decl::new_fn(stringify!($ident), vec![$( ltype!($i) ),*], Vec::new(), create_return_type!($($ret)?)))
    };
    (
        DECL $ident:ident( $($i:ident),* )$(: $ret:expr)?;
    ) => {
        PStmt::Decl(Decl::new_fn(stringify!($ident), vec![$( ltype!($i) ),*], Vec::new(),create_return_type!($($ret)?)))
    };
    (
        DECL $ident:ident( $( $i:expr ),* )$(: $ret:ident)?;
    ) => {
        PStmt::Decl(Decl::new_fn(stringify!($ident), vec![$( $i ),*], Vec::new(), create_return_type!($($ret)?)))
    };
    (
        DECL $ident:ident( $($i:expr),* )$(: $ret:expr)?;
    ) => {
        PStmt::Decl(Decl::new_fn(stringify!($ident), vec![$( $i ),*], Vec::new(),create_return_type!($($ret)?)))
    };
}

#[macro_export]
macro_rules! create_return_type {
    () => {
        None
    };
    ($ret:ident) => {
        Some(ltype!($ret))
    };
    ($ret:expr) => {
        Some($ret)
    };
}

#[macro_export]
macro_rules! lif {
    (
        $cond:expr,
        $( $body:expr );*
    ) => {
        PStmt::If {
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
        PExpr::Variable($ident)
    };
    (
        $ident:ident
    ) => {
        PExpr::Variable(stringify!($ident))
    };
}

#[macro_export]
macro_rules! lbop {
    (
        $l:expr,
        $op:expr,
        $r:expr
    ) => {
        PExpr::Op(Box::new(B2Op::binary($l.into(), $op, $r.into()).into()))
    };
}

#[macro_export]
macro_rules! lvdar {
    (
        $ident:expr,
        $type:ty,
        $value:expr
    ) => {
        VariableDeclarationAssignment::new($ident, Some($type), $value.into())
    };
    (
        $ident:expr,
        $value:expr
    ) => {
        VariableDeclarationAssignment::new($ident, None, $value.into())
    };
}

#[macro_export]
macro_rules! lvda {
    (
        $ident:expr,
        $type:ty,
        $value:expr
    ) => {
        PStmt::new_var_decl_ass($ident, Some($type), $value.into())
    };
    (
        $ident:expr,
        $value:expr
    ) => {
        PStmt::new_var_decl_ass($ident, None, $value.into())
    };
}

#[macro_export]
macro_rules! lvra {
    (
        $ident:expr,
        $value:expr
    ) => {
        PStmt::VariableReassignment {
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
        PExpr::Group(Box::new($expr.into()))
    };
}

#[macro_export]
macro_rules! lai {
    (
        $indexee:expr,
        $indexer:expr
    ) => {
        PExpr::Op(Box::new(
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
        PExpr::FunctionCall {
            ident: $ident,
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
        PStmt::FunctionInvocation {
            ident: $ident,
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
        PStmt::Return { value: None }
    };
    (
        $expr:expr
    ) => {
        PStmt::Return {
            value: Some($expr.into()),
        }
    };
}

#[macro_export]
macro_rules! leel {
    () => {
        PExpr::List(Vec::new())
    };
}

#[macro_export]
macro_rules! lfnt {
    (
        $input:expr,
        $output:expr
    ) => {
        Type::FnType {
            input: Box::new($input),
            output: Box::new($output)
        }
    };
    (
        $input:expr,
        $($rem:expr),*
    ) => {
        Type::FnType {
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
        Type::Mono(MonoType::SelfType)
    };
    (
        NIL
    ) => {
        Type::Mono(MonoType::Nil)
    };
    (
        STR
    ) => {
        Type::Mono(MonoType::Str)
    };
    (
        INT
    ) => {
        Type::Mono(MonoType::Int)
    };
    (
        BOOL
    ) => {
        Type::Mono(MonoType::Bool)
    };
    (
        $id:expr ; $va:expr
    ) => {
        Type::Mono(MonoType::EnumVariant($id, $va))
    };
    (
        [ $i:ident ]
    ) => {
        Type::Poly(PolyType::List(Box::new(ltype!($i))))
    };
    (
        ( $f:ident, $s:ident )
    ) => {
        Type::Poly(PolyType::Tuple { fst: Box::new(ltype!($f)), snd: Box::new(ltype!($s)) })
    };
    (
        ( $f:ident, $s:expr )
    ) => {
        Type::Poly(PolyType::Tuple { fst: Box::new(ltype!($f)), snd: Box::new($s) })
    };
    (
        ( $f:expr, $s:ident )
    ) => {
        Type::Poly(PolyType::Tuple { fst: Box::new($f), snd: Box::new(ltype!($s)) })
    };
    (
        ( $f:expr, $s:expr )
    ) => {
        Type::Poly(PolyType::Tuple { fst: Box::new($f), snd: Box::new($s) })
    };
    ($i:ident => $o:ident) => {
        Type::Poly(PolyType::FnType {
            input: Box::new(ltype!($i)),
            output: Box::new(ltype!($o)),
        })
    };
    (
        $i:ident =>
        $($o:ident)=>*
    ) => {
        Type::Poly(PolyType::FnType {
            input: Box::new(ltype!($i)),
            output: Box::new(ltype!($($o)=>*)),
        })
    };
}

#[macro_export]
macro_rules! lyv {
    ($t:expr) => {
        Type::Mono(MonoType::TypeVar($t))
    };
}

#[macro_export]
macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    (
        $(($key:expr, $val:expr)),+ $(,)?
    ) => {
        (|| {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)*
            map
        })()
    };
}
