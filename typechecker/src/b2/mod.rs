use crate::b2::stm::{B2Stmt, decls::Decl};
use std::collections::HashMap;

pub mod exp;
pub mod stm;
pub mod typ;

pub struct Module<'a> {
    ident: &'a str,
    prg: Vec<B2Stmt<'a>>,
    deps: Vec<&'a str>,
    exps: HashMap<&'a str, Decl<'a>>,
}

impl<'a> Module<'a> {
    pub fn new(
        ident: &'a str,
        prg: Vec<B2Stmt<'a>>,
        deps: Vec<&'a str>,
        exps: HashMap<&'a str, Decl<'a>>,
    ) -> Self {
        Self {
            ident,
            prg,
            deps,
            exps,
        }
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn program(&self) -> &[B2Stmt] {
        &self.prg
    }

    pub fn dependencies(&self) -> &[&str] {
        &self.deps
    }

    pub fn exports(&self) -> &HashMap<&str, Decl> {
        &self.exps
    }
}
