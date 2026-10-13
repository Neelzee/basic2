use crate::b2::{
    Module,
    exp::B2Expr,
    stm::{
        B2Stmt,
        decls::{Decl, FnDecl, VarDecl},
        impls::Impl,
    },
    typ::B2Type,
};
use parser::{
    common::{B2OpInner, postfix::Postfix, primitive::Primitive},
    pexpr::PExpr,
    pmod::LexModule,
    pstmt::PStmt,
};
use std::collections::HashMap;

pub enum B2Error<'a> {
    MissMatchModuleIdentifierName { start: &'a str, end: &'a str },
    IdentifierExists { ident: &'a str, st: SymbolTable<'a> },
    IdentifierDoesNotExist { ident: &'a str, st: SymbolTable<'a> },
}
pub type B2Result<'a, T> = Result<T, B2Error<'a>>;

pub fn typecheck_module<'a>(module: LexModule<'a>) -> B2Result<'a, Module<'a>> {
    todo!()
}

pub fn typecheck_statement<'a>(
    stmt: &PStmt<'a>,
    st: SymbolTable<'a>,
) -> B2Result<'a, (SymbolTable<'a>, B2Stmt<'a>)> {
    match stmt {
        PStmt::Decl(decl) => todo!(),
        PStmt::Impl(_) => todo!(),
        PStmt::Import(import) => todo!(),
        PStmt::VariableReassignment {
            ident,
            reassignment,
            new_value,
        } => todo!(),
        PStmt::ListReassignment {
            indexee,
            index,
            reassignment,
            new_value,
        } => todo!(),
        PStmt::If { condition, body } => todo!(),
        PStmt::While { condition, body } => todo!(),
        PStmt::Block { body } => todo!(),
        PStmt::FunctionInvocation { ident, arguments } => todo!(),
        PStmt::Break => todo!(),
        PStmt::Continue => todo!(),
        PStmt::Return { value } => todo!(),
        PStmt::For {
            start_stmt,
            condition,
            incrementer,
            body,
        } => todo!(),
        PStmt::StructFieldReassignment {
            ident,
            field,
            reassignment,
            new_value,
        } => todo!(),
        PStmt::WhenStatement { ident, branches } => todo!(),
    }
}

pub fn infer_type<'a>(expr: &'a PExpr<'a>, st: &'a SymbolTable<'a>) -> Option<B2Type<'a>> {
    match expr {
        PExpr::Nil => todo!(),
        PExpr::Literal(p) => match p {
            Primitive::Int(_) => Some(B2Type::int()),
            Primitive::Float(_) => Some(B2Type::float()),
            Primitive::Str(_) => Some(B2Type::str()),
            Primitive::Bool(_) => Some(B2Type::bool()),
        },
        PExpr::Tuple(fst, snd) => {
            infer_type(fst, st).and_then(|f| infer_type(snd, st).map(|s| B2Type::tuple(f, s)))
        }
        PExpr::List(xs) => xs.last().and_then(|x| infer_type(x, st)),
        PExpr::Variable(i) => st.lookup_var(i).and_then(|v| {
            v.get_type()
                .cloned()
                .or_else(|| v.get_expr().map(|x| x.get_type(st)))
        }),
        PExpr::Group(i) => infer_type(i, st),
        PExpr::FunctionCall { ident: ident, .. } => {
            st.lookup_fn(ident).map(|f| f.get_return()).cloned()
        }
        PExpr::Op(op) => match op.inner() {
            B2OpInner::Prefix(_, e) => infer_type(e, st),
            B2OpInner::Postfix(e, o) => match o {
                Postfix::Incr | Postfix::Decr => infer_type(e, st),
                Postfix::Index(lex_expr) => todo!(),
            },
            B2OpInner::Binary(_, _, _) => todo!(),
        },
        PExpr::Struct {
            ident: ident,
            field_implementations,
        } => todo!(),
        PExpr::StructFieldAccessing {
            ident: ident,
            field,
        } => todo!(),
        PExpr::Enum {
            ident,
            instance,
            values,
        } => todo!(),
    }
}

#[derive(Clone, Default)]
pub struct SymbolTable<'a> {
    outer: Option<Box<Self>>,
    decls: HashMap<&'a str, Decl<'a>>,
    impls: HashMap<&'a str, Impl<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn add_var(
        self,
        ident: &'a str,
        b2_type: Option<B2Type<'a>>,
        val: Option<B2Expr<'a>>,
    ) -> Self {
        let mut decls = self.decls;
        decls.insert(ident, Decl::VarDecl(VarDecl::new(ident, b2_type, val)));
        Self { decls, ..self }
    }

    pub fn lookup_fn(&self, ident: &str) -> Option<&FnDecl> {
        self.decls
            .get(ident)
            .and_then(|d| d.as_fn())
            .or_else(|| self.outer.as_ref().and_then(|st| st.lookup_fn(ident)))
    }

    pub fn lookup_var(&self, ident: &str) -> Option<&VarDecl> {
        self.decls.get(ident).and_then(|d| d.as_var())
    }

    pub fn contains_ident(&self, ident: &str) -> bool {
        self.decls.contains_key(ident)
    }

    pub fn type_has_trait(&self, tr: &str, ty: &str) -> bool {
        self.impls
            .get(tr)
            .and_then(|i| i.as_trait())
            .is_some_and(|tr| tr.type_ident() == ty)
    }
}
