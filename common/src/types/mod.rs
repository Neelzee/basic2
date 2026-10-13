mod impls;
mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
    Mono(MonoType<'a>),
    Poly(PolyType<Self>),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MonoType<'a> {
    #[default]
    Nil,
    Str,
    Int,
    Float,
    Bool,
    SelfType,
    /// Can be a Type alias, a generic, and a struct
    TypeVar(&'a str),
    TypeVarGen(&'a str, Vec<&'a str>),
    /// # Example
    ///
    /// ```b2
    /// ENUMS Num
    ///   One;
    ///   Two;
    /// END
    ///
    /// DECL ConstOne() : Num.One;
    /// IMPL ConstOne()
    ///   RETURN Num.One;
    /// ```
    EnumVariant(&'a str, &'a str),
    StructType(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolyType<T> {
    Tuple { fst: Box<T>, snd: Box<T> },
    List(Box<T>),
    FnType { input: Box<T>, output: Box<T> },
}
