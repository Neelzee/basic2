use crate::b2::{exp::B2Expr, stm::B2Stmt};

#[derive(Debug, Clone)]
pub struct FnImpl<'a> {
    ident: &'a str,
    pars: Vec<(&'a str, Option<B2Expr<'a>>)>,
    body: Vec<Self>,
}

#[derive(Debug, Clone)]
pub struct TraitImpl<'a> {
    trait_ident: &'a str,
    type_ident: &'a str,
    rests: Vec<&'a str>,
    body: Vec<B2Stmt<'a>>,
}

impl<'a> TraitImpl<'a> {
    pub fn type_ident(&self) -> &str {
        &self.type_ident
    }
}

#[derive(Debug, Clone)]
pub enum Impl<'a> {
    FnImpl(FnImpl<'a>),
    TraitImpl(TraitImpl<'a>),
}

impl<'a> Impl<'a> {
    pub fn as_trait(&self) -> Option<&TraitImpl<'a>> {
        match self {
            Impl::TraitImpl(trait_impl) => Some(trait_impl),
            _ => None,
        }
    }
}
