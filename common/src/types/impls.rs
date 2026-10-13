use crate::{
    ToB2,
    consts::{
        pstms::{
            ENUM_INDEXING,
            fns::{
                FUNCTION_GENERICS_DELIMITER, FUNCTION_GENERICS_END, FUNCTION_GENERICS_START,
                FUNCTION_TYPE_ARROW_KW, FUNCTION_TYPE_END, FUNCTION_TYPE_START,
            },
        },
        types::{
            BOOL_TYPE_KW, FLOAT_TYPE_KW, INT_TYPE_KW, LIST_END, LIST_START, NIL_TYPE_KW,
            SELF_TYPE_KW, STR_TYPE_KW, TUPLE_DELIMITER, TUPLE_END, TUPLE_START,
        },
    },
    types::{MonoType, PolyType, Type},
};

impl<'a> Type<'a> {
    pub fn nil() -> Self {
        Self::Mono(MonoType::Nil)
    }

    pub fn int() -> Self {
        Self::Mono(MonoType::Int)
    }

    pub fn float() -> Self {
        Self::Mono(MonoType::Float)
    }

    pub fn str() -> Self {
        Self::Mono(MonoType::Str)
    }

    pub fn bool() -> Self {
        Self::Mono(MonoType::Bool)
    }

    pub fn tuple(fst: Self, snd: Self) -> Self {
        Self::Poly(PolyType::Tuple {
            fst: Box::new(fst),
            snd: Box::new(snd),
        })
    }

    pub fn list(elem: Self) -> Self {
        Self::Poly(PolyType::List(Box::new(elem)))
    }

    pub fn fun(input: Self, output: Self) -> Self {
        Self::Poly(PolyType::FnType {
            input: Box::new(input),
            output: Box::new(output),
        })
    }

    pub fn r#enum(identifier: &'a str, instance: &'a str) -> Self {
        Self::Mono(MonoType::EnumVariant(identifier, instance))
    }

    pub fn var(identifier: &'a str) -> Self {
        Self::Mono(MonoType::TypeVar(identifier))
    }

    pub fn r#struct(identifier: &'a str) -> Self {
        Self::Mono(MonoType::StructType(identifier))
    }
}

impl<'a> ToB2 for Type<'a> {
    fn to_b2(&self) -> String {
        match self {
            Self::Mono(m) => m.to_b2(),
            Self::Poly(p) => p.to_b2(),
        }
    }
}

impl<'a> Default for Type<'a> {
    fn default() -> Self {
        Self::Mono(MonoType::default())
    }
}

impl<'a> From<&'a str> for Type<'a> {
    fn from(value: &'a str) -> Self {
        Self::Mono(MonoType::TypeVar(value))
    }
}

impl<'a> ToB2 for MonoType<'a> {
    fn to_b2(&self) -> String {
        match self {
            MonoType::Nil => NIL_TYPE_KW.to_string(),
            MonoType::Str => STR_TYPE_KW.to_string(),
            MonoType::Int => INT_TYPE_KW.to_string(),
            MonoType::Float => FLOAT_TYPE_KW.to_string(),
            MonoType::Bool => BOOL_TYPE_KW.to_string(),
            MonoType::SelfType => SELF_TYPE_KW.to_string(),
            MonoType::TypeVar(v) => v.to_string(),
            MonoType::TypeVarGen(v, items) => format!(
                "{v}{FUNCTION_GENERICS_START}{}{FUNCTION_GENERICS_END}",
                items.join(FUNCTION_GENERICS_DELIMITER)
            ),
            MonoType::EnumVariant(id, var) => format!("{id}{ENUM_INDEXING}{var}"),
            MonoType::StructType(ident) => ident.to_string(),
        }
    }
}

impl<'a, T: ToB2> ToB2 for PolyType<T> {
    fn to_b2(&self) -> String {
        match self {
            PolyType::Tuple { fst, snd } => format!(
                "{TUPLE_START}{}{TUPLE_DELIMITER}{}{TUPLE_END}",
                fst.to_b2(),
                snd.to_b2()
            ),
            PolyType::List(t) => format!("{LIST_START}{}{LIST_END}", t.to_b2()),
            PolyType::FnType { input, output } => format!(
                "{FUNCTION_TYPE_START}{}{FUNCTION_TYPE_ARROW_KW}{}{FUNCTION_TYPE_END}",
                input.to_b2(),
                output.to_b2()
            ),
        }
    }
}
