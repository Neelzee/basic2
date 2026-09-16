use crate::interpreter::gst::GlobalSymbolTable;
use parser::lexer::{lex_expr::LexExpr, lex_type::LexType};

pub enum IE<'a> {
    UnpackingNonUnpackableType(LexType<'a>),
    MissingFunctionArgument(&'a str, usize, &'a str),
    UndefinedVariable(&'a str),
    MissingFunctionImplementation(&'a str),
    StructAccessingOnNonStruct(&'a str, &'a str, LexType<'a>),
    StructAccessingOnNonExistingField(&'a str, &'a str),
    StructAccessingOnNonExistingStruct(&'a str, &'a str),
    MoreArgumentsSuppliedToFunctionThanExpected(
        &'a str,
        Vec<LexExpr<'a>>,
        Vec<(&'a str, Option<LexExpr<'a>>)>,
    ),
}

pub enum IR<'a, T> {
    Ok {
        scope: usize,
        gst: GlobalSymbolTable<'a>,
        val: T,
    },
    Err(IE<'a>),
}

impl<'a, T> IR<'a, T> {
    pub fn map<F, K>(self, f: F) -> IR<'a, K>
    where
        F: Fn(T) -> K,
    {
        match self {
            Self::Ok { scope, gst, val } => IR::Ok {
                val: f(val),
                scope,
                gst,
            },
            Self::Err(err) => IR::Err(err),
        }
    }

    pub fn unwrap_or(self, fallback: T) -> T {
        match self {
            IR::Ok { val, .. } => val,
            IR::Err(_) => fallback,
        }
    }

    pub fn ok(scope: usize, gst: GlobalSymbolTable<'a>, val: T) -> Self {
        Self::Ok { scope, gst, val }
    }

    pub fn err(err: IE<'a>) -> Self {
        Self::Err(err)
    }
}
