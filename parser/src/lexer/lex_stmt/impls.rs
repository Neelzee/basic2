use crate::lexer::{lex_expr::LexExpr, lex_stmt::LexStmt};

#[derive(Debug, PartialEq, Clone)]
pub enum Impl<'a> {
    FnImpl(FunctionImplementation<'a>),
    TraitImpl(TraitImpl<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionImplementation<'a> {
    identifier: &'a str,
    parameters: Vec<(&'a str, Option<LexExpr<'a>>)>,
    body: Vec<LexStmt<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitImpl<'a> {
    trait_identifier: &'a str,
    type_identifier: &'a str,
    body: Vec<LexStmt<'a>>,
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
        parameters: Vec<(&'a str, Option<LexExpr<'a>>)>,
        body: Vec<LexStmt<'a>>,
    ) -> Self {
        Self::FnImpl(FunctionImplementation {
            identifier,
            parameters,
            body,
        })
    }

    pub fn new_trait(
        trait_identifier: &'a str,
        type_identifier: &'a str,
        body: Vec<LexStmt<'a>>,
    ) -> Self {
        Self::TraitImpl(TraitImpl {
            trait_identifier,
            type_identifier,
            body,
        })
    }
}
