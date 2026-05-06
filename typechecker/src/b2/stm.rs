use crate::b2::{
    exp::B2Expr,
    stm::{decls::Decl, impls::Impl},
    typ::B2Type,
};
use parser::common::binop::BinOp;

pub mod decls;
pub mod impls;

#[derive(Debug, Clone)]
pub enum B2Stmt<'a> {
    VarReAss {
        ident: &'a str,
        val: B2Expr<'a>,
    },
    ListReAss {
        ide: B2Expr<'a>,
        idx: B2Expr<'a>,
        re: Option<BinOp>,
        val: B2Expr<'a>,
    },
    If {
        cond: B2Expr<'a>,
        body: Vec<Self>,
    },
    While {
        cond: B2Expr<'a>,
        body: Vec<Self>,
    },
    Block(Vec<Self>),
    Break,
    Continue,
    Return(Option<B2Expr<'a>>),
    For {
        start_stmt: Box<Self>,
        condition: B2Expr<'a>,
        incrementer: B2Expr<'a>,
        body: Vec<Self>,
    },
    StructFieldReAss {
        ident: &'a str,
        field: &'a str,
        re: Option<BinOp>,
        val: B2Expr<'a>,
    },
    When {
        ident: &'a str,
        branches: Vec<B2Branch<'a>>,
    },
    Decl(Decl<'a>),
    Impl(Impl<'a>),
}

pub type FunImplComps<'a> = (&'a str, Vec<(&'a str, Option<B2Expr<'a>>)>, Vec<B2Stmt<'a>>);
pub type FunDeclComps<'a> = (
    &'a str,
    Vec<(&'a str, Vec<&'a str>)>,
    Vec<B2Type<'a>>,
    Option<B2Type<'a>>,
);

#[derive(Debug, Clone)]
pub enum B2Branch<'a> {
    /// [ ]
    EmptyList {
        cond: Option<B2Expr<'a>>,
        body: Vec<B2Stmt<'a>>,
    },
    /// [x]
    Singleton {
        ident: &'a str,
        cond: Option<B2Expr<'a>>,
        body: Vec<B2Stmt<'a>>,
    },
    /// [a, b, ...xs]
    VariadicList {
        idents: Vec<&'a str>,
        rem: Option<&'a str>,
        cond: Option<B2Expr<'a>>,
        body: Vec<B2Stmt<'a>>,
    },
    /// x
    CatchAll {
        ident: &'a str,
        cond: Option<B2Expr<'a>>,
        body: Vec<B2Stmt<'a>>,
    },
    /// IS INT
    /// Matches if the value is assignable to the specified type
    Type {
        b2_type: B2Type<'a>,
        cond: Option<B2Expr<'a>>,
        body: Vec<B2Stmt<'a>>,
    },
    // [::field_a, ::field_b] AND field_a < field_b FOLLOWS ...
    StructField {
        fields: Vec<&'a str>,
        cond: Option<B2Expr<'a>>,
        body: Vec<B2Stmt<'a>>,
    },
}
