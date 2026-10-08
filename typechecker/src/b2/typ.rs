use crate::b2::typ::B2Type::Poly;

#[derive(Debug, Clone, PartialEq)]
pub enum B2Type<'a> {
    Mono(MonoType<'a>),
    Poly(PolyType<Self>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MonoType<'a> {
    Nil,
    Int,
    Float,
    Bool,
    Str,
    TypeVar(&'a str),
    EnumVariant(&'a str, &'a str),
    Struct(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolyType<T> {
    List(Box<T>),
    Tuple(Box<T>, Box<T>),
    FnType(Box<T>, Box<T>),
}

impl<'a> B2Type<'a> {
    pub fn int() -> Self {
        Self::Mono(MonoType::Int)
    }

    pub fn float() -> Self {
        Self::Mono(MonoType::Float)
    }

    pub fn bool() -> Self {
        Self::Mono(MonoType::Bool)
    }

    pub fn str() -> Self {
        Self::Mono(MonoType::Str)
    }

    pub fn tuple(l: Self, r: Self) -> Self {
        Self::Poly(PolyType::Tuple(Box::new(l), Box::new(r)))
    }
}