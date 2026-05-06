use std::process::id;

use crate::{
    common::ToB2,
    lexer::{
        lex_type::{LexMonoType, LexPolyType, LexType},
        utils::consts::{
            BOOL_TYPE_KW, ENUM_INDEXING, FLOAT_TYPE_KW, FUNCTION_GENERICS_DELIMITER,
            FUNCTION_GENERICS_END, FUNCTION_GENERICS_START, FUNCTION_TYPE_ARROW_KW,
            FUNCTION_TYPE_END, FUNCTION_TYPE_START, INT_TYPE_KW, LIST_END, LIST_START, NIL_TYPE_KW,
            SELF_TYPE_KW, STR_TYPE_KW, TUPLE_DELIMITER, TUPLE_END, TUPLE_START,
        },
    },
};

impl<'a> LexType<'a> {
    pub fn nil() -> Self {
        Self::Mono(LexMonoType::Nil)
    }

    pub fn int() -> Self {
        Self::Mono(LexMonoType::Int)
    }

    pub fn float() -> Self {
        Self::Mono(LexMonoType::Float)
    }

    pub fn str() -> Self {
        Self::Mono(LexMonoType::Str)
    }

    pub fn bool() -> Self {
        Self::Mono(LexMonoType::Bool)
    }

    pub fn tuple(fst: Self, snd: Self) -> Self {
        Self::Poly(LexPolyType::Tuple {
            fst: Box::new(fst),
            snd: Box::new(snd),
        })
    }

    pub fn list(elem: Self) -> Self {
        Self::Poly(LexPolyType::List(Box::new(elem)))
    }

    pub fn fun(input: Self, output: Self) -> Self {
        Self::Poly(LexPolyType::FnType {
            input: Box::new(input),
            output: Box::new(output),
        })
    }

    pub fn r#enum(identifier: &'a str, instance: &'a str) -> Self {
        Self::Mono(LexMonoType::EnumVariant(identifier, instance))
    }

    pub fn var(identifier: &'a str) -> Self {
        Self::Mono(LexMonoType::TypeVar(identifier))
    }
}

impl<'a> ToB2 for LexType<'a> {
    fn to_b2(&self) -> String {
        match self {
            Self::Mono(m) => m.to_b2(),
            Self::Poly(p) => p.to_b2(),
        }
    }
}

impl<'a> Default for LexType<'a> {
    fn default() -> Self {
        Self::Mono(LexMonoType::default())
    }
}

impl<'a> From<&'a str> for LexType<'a> {
    fn from(value: &'a str) -> Self {
        Self::Mono(LexMonoType::TypeVar(value))
    }
}

impl<'a> ToB2 for LexMonoType<'a> {
    fn to_b2(&self) -> String {
        match self {
            LexMonoType::Nil => NIL_TYPE_KW.to_string(),
            LexMonoType::Str => STR_TYPE_KW.to_string(),
            LexMonoType::Int => INT_TYPE_KW.to_string(),
            LexMonoType::Float => FLOAT_TYPE_KW.to_string(),
            LexMonoType::Bool => BOOL_TYPE_KW.to_string(),
            LexMonoType::SelfType => SELF_TYPE_KW.to_string(),
            LexMonoType::TypeVar(v) => v.to_string(),
            LexMonoType::TypeVarGen(v, items) => format!(
                "{v}{FUNCTION_GENERICS_START}{}{FUNCTION_GENERICS_END}",
                items.join(FUNCTION_GENERICS_DELIMITER)
            ),
            LexMonoType::EnumVariant(id, var) => format!("{id}{ENUM_INDEXING}{var}"),
        }
    }
}

impl<'a, T: ToB2> ToB2 for LexPolyType<T> {
    fn to_b2(&self) -> String {
        match self {
            LexPolyType::Tuple { fst, snd } => format!(
                "{TUPLE_START}{}{TUPLE_DELIMITER}{}{TUPLE_END}",
                fst.to_b2(),
                snd.to_b2()
            ),
            LexPolyType::List(t) => format!("{LIST_START}{}{LIST_END}", t.to_b2()),
            LexPolyType::FnType { input, output } => format!(
                "{FUNCTION_TYPE_START}{}{FUNCTION_TYPE_ARROW_KW}{}{FUNCTION_TYPE_END}",
                input.to_b2(),
                output.to_b2()
            ),
        }
    }
}
