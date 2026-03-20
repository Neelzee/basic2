mod impls;
mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum LexType<'a> {
    Mono(LexMonoType<'a>),
    Poly(LexPolyType<Self>),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LexMonoType<'a> {
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexPolyType<T> {
    Tuple { fst: Box<T>, snd: Box<T> },
    List(Box<T>),
    FnType { input: Box<T>, output: Box<T> },
}
