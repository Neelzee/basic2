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
