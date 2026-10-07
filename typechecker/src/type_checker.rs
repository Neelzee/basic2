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
    lexer::{lex_expr::LexExpr, lex_mod::LexModule, lex_stmt::LexStmt},
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
    stmt: &LexStmt<'a>,
    st: SymbolTable<'a>,
) -> B2Result<'a, (SymbolTable<'a>, B2Stmt<'a>)> {
    match stmt {
        LexStmt::EnumDeclaration { identifier, .. }
        | LexStmt::TraitDecl { identifier, .. }
        | LexStmt::TypeAlias { identifier, .. }
        | LexStmt::ImportModule { identifier }
        | LexStmt::StructDeclaration { identifier, .. }
        | LexStmt::FunctionDeclaration { identifier, .. }
        | LexStmt::VariableDeclarationAssignment { identifier, .. }
        | LexStmt::VariableDeclaration { identifier, .. }
            if st.contains_ident(identifier) =>
        {
            Err(B2Error::IdentifierExists {
                ident: identifier,
                st,
            })
        }
        LexStmt::TupleUnpacking { identifiers, .. }
        | LexStmt::StructUnpacking { identifiers, .. }
        | LexStmt::ListUnpacking { identifiers, .. }
            if identifiers.iter().any(|i| st.contains_ident(i)) =>
        {
            Err(B2Error::IdentifierExists {
                ident: identifiers.iter().find(|i| st.contains_ident(i)).unwrap(),
                st,
            })
        }
        LexStmt::VariableReassignment {
            ident: identifier, ..
        } if !st.contains_ident(identifier) => Err(B2Error::IdentifierDoesNotExist {
            ident: identifier,
            st,
        }),
        LexStmt::ListReassignment {
            indexee,
            index,
            reassignment,
            new_value,
        } => todo!(),
        LexStmt::If { condition, body } => todo!(),
        LexStmt::While { condition, body } => todo!(),
        LexStmt::FunctionImplementation {
            identifier,
            parameters,
            body,
        } => todo!(),
        LexStmt::Block { body } => todo!(),
        LexStmt::FunctionInvocation {
            identifier,
            arguments,
        } => todo!(),
        LexStmt::Break => todo!(),
        LexStmt::Continue => todo!(),
        LexStmt::Return { value } => todo!(),
        LexStmt::For {
            start_stmt,
            condition,
            incrementer,
            body,
        } => todo!(),
        LexStmt::StructFieldReassignment {
            identifier,
            field,
            reassignment,
            new_value,
        } => todo!(),
        LexStmt::WhenStatement {
            identifier,
            branches,
        } => todo!(),
        LexStmt::TraitImpl {
            trait_identifier,
            type_identifier,
            body,
        } => todo!(),
        LexStmt::VariableDeclaration {
            identifier,
            variable_type,
        } => todo!(),
        LexStmt::VariableDeclarationAssignment {
            identifier,
            variable_type,
            value,
        } => todo!(),
        LexStmt::FunctionDeclaration {
            identifier,
            parameters,
            generics,
            return_type,
        } => todo!(),
        LexStmt::StructDeclaration { identifier, fields } => todo!(),
        LexStmt::TypeAlias {
            identifier,
            b2_type,
        } => todo!(),
        LexStmt::ImportModule { identifier } => todo!(),
        LexStmt::EnumDeclaration {
            identifier,
            enumerations,
        } => todo!(),
        LexStmt::TraitDecl {
            identifier,
            restrictions,
            decls,
            impls,
        } => todo!(),
        LexStmt::TupleUnpacking { identifiers, value } => todo!(),
        LexStmt::StructUnpacking { identifiers, value } => todo!(),
        LexStmt::ListUnpacking {
            identifiers,
            remainder,
            value,
        } => todo!(),
        LexStmt::VariableReassignment {
            ident: identifier,
            reassignment,
            new_value,
        } => todo!(),
    }
}

pub fn infer_type<'a>(expr: &'a LexExpr<'a>, st: &'a SymbolTable<'a>) -> Option<B2Type<'a>> {
    match expr {
        LexExpr::Literal(p) => match p {
            Primitive::Int(_) => Some(B2Type::Int),
            Primitive::Float(_) => Some(B2Type::Float),
            Primitive::Str(_) => Some(B2Type::Str),
            Primitive::Bool(_) => Some(B2Type::Bool),
        },
        LexExpr::Tuple(fst, snd) => infer_type(fst, st)
            .and_then(|f| infer_type(snd, st).map(|s| B2Type::Tuple(Box::new(f), Box::new(s)))),
        LexExpr::List(xs) => xs.last().and_then(|x| infer_type(x, st)),
        LexExpr::Variable(i) => st.lookup_var(i).and_then(|v| {
            v.get_type()
                .cloned()
                .or_else(|| v.get_expr().map(|x| x.get_type(st)))
        }),
        LexExpr::Group(i) => infer_type(i, st),
        LexExpr::FunctionCall {
            ident: identifier, ..
        } => st.lookup_fn(identifier).map(|f| f.get_return()).cloned(),
        LexExpr::Op(op) => match op.inner() {
            B2OpInner::Prefix(_, e) => infer_type(e, st),
            B2OpInner::Postfix(e, o) => match o {
                Postfix::Incr | Postfix::Decr => infer_type(e, st),
                Postfix::Index(lex_expr) => todo!(),
            },
            B2OpInner::Binary(_, _, _) => todo!(),
        },
        LexExpr::Struct {
            ident: identifier,
            field_implementations,
        } => todo!(),
        LexExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => todo!(),
        LexExpr::Enum {
            ident: identifier,
            instance,
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
