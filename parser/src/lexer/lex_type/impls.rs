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

impl<'a> ToB2 for LexType<'a> {
    fn to_b2(&self) -> String {
        match self {
            LexType::Mono(LexMonoType::Nil) => NIL_TYPE_KW.to_string(),
            LexType::Mono(LexMonoType::Str) => STR_TYPE_KW.to_string(),
            LexType::Mono(LexMonoType::Int) => INT_TYPE_KW.to_string(),
            LexType::Mono(LexMonoType::Float) => FLOAT_TYPE_KW.to_string(),
            LexType::Mono(LexMonoType::Bool) => BOOL_TYPE_KW.to_string(),
            LexType::Mono(LexMonoType::SelfType) => SELF_TYPE_KW.to_string(),
            LexType::Poly(LexPolyType::Tuple { fst, snd }) => format!(
                "{TUPLE_START}{}{TUPLE_DELIMITER}{}{TUPLE_END}",
                fst.to_b2(),
                snd.to_b2()
            ),
            LexType::Poly(LexPolyType::List(lex_type)) => {
                format!("{LIST_START}{}{LIST_END}", lex_type.to_b2())
            }
            LexType::Mono(LexMonoType::TypeVar(v)) => v.to_string(),
            LexType::Mono(LexMonoType::TypeVarGen(v, items)) => format!(
                "{v}{FUNCTION_GENERICS_START}{}{FUNCTION_GENERICS_END}",
                items
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(FUNCTION_GENERICS_DELIMITER)
            ),
            LexType::Poly(LexPolyType::FnType { input, output }) => format!(
                "{FUNCTION_TYPE_START}{}{FUNCTION_TYPE_ARROW_KW}{}{FUNCTION_TYPE_END}",
                input.to_b2(),
                output.to_b2()
            ),
            LexType::Mono(LexMonoType::EnumVariant(e, i)) => format!("{e}{ENUM_INDEXING}{i}"),
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
