use crate::{pexpr::PExpr, pstmt::PStmt};

#[derive(Debug, PartialEq, Clone)]
pub enum Impl<'a> {
    FnImpl(FunctionImplementation<'a>),
    TraitImpl(TraitImpl<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionImplementation<'a> {
    pub identifier: &'a str,
    /// Function parameters, an identifier, and an optional default value
    pub parameters: Vec<(&'a str, Option<PExpr<'a>>)>,
    pub body: Vec<PStmt<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitImpl<'a> {
    /// Trait identifier?
    pub trait_identifier: &'a str,
    /// What is this?
    pub type_identifier: Vec<&'a str>,
    pub body: Vec<PStmt<'a>>,
}

impl<'a> Impl<'a> {
    pub fn as_fn_impl(self) -> Option<FunctionImplementation<'a>> {
        match self {
            Impl::FnImpl(fun) => Some(fun),
            _ => None,
        }
    }

    pub fn as_trait_impl(self) -> Option<TraitImpl<'a>> {
        match self {
            Impl::TraitImpl(trt) => Some(trt),
            _ => None,
        }
    }

    pub fn new_fn(
        identifier: &'a str,
        parameters: Vec<(&'a str, Option<PExpr<'a>>)>,
        body: Vec<PStmt<'a>>,
    ) -> Self {
        Self::FnImpl(FunctionImplementation {
            identifier,
            parameters,
            body,
        })
    }

    pub fn new_trait(
        trait_identifier: &'a str,
        type_identifier: Vec<&'a str>,
        body: Vec<PStmt<'a>>,
    ) -> Self {
        Self::TraitImpl(TraitImpl {
            trait_identifier,
            type_identifier,
            body,
        })
    }
}
