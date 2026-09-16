use std::todo;

use crate::symbol_table::SymbolTable;
use parser::{
    common::{AsB2Type, ToB2}, lexer::{
        lex_mod::LexModule, lex_stmt::{
            decls::variants::{
                FunctionDeclaration, StructDeclaration, TraitDecl, TypeAlias, VariableDeclaration,
                VariableDeclarationAssignment,
            },
            impls::{FunctionImplementation, TraitImpl},
        }, lex_type::LexType,
    },
};

#[derive(Debug, Clone)]
pub struct GlobalSymbolTable<'a> {
    vars: SymbolTable<'a, VariableDeclarationAssignment<'a>>,
    fn_decls: SymbolTable<'a, FunctionDeclaration<'a>>,
    fn_impls: SymbolTable<'a, FunctionImplementation<'a>>,
    trait_decls: SymbolTable<'a, TraitDecl<'a>>,
    trait_impls: SymbolTable<'a, TraitImpl<'a>>,
    struct_decls: SymbolTable<'a, StructDeclaration<'a>>,
    type_decls: SymbolTable<'a, TypeAlias<'a>>,
}

impl<'a> GlobalSymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            vars: SymbolTable::new(),
            fn_decls: SymbolTable::new(),
            fn_impls: SymbolTable::new(),
            trait_decls: SymbolTable::new(),
            trait_impls: SymbolTable::new(),
            struct_decls: SymbolTable::new(),
            type_decls: SymbolTable::new(),
        }
    }

    pub fn from_module(module: &LexModule<'a>) -> Self {
        let stmts = module.statements().clone();
        Self {
            vars: todo!(),
            fn_decls: todo!(),
            fn_impls: todo!(),
            trait_decls: todo!(),
            trait_impls: todo!(),
            struct_decls: todo!(),
            type_decls: todo!(),
        }
    }

    pub fn lookup_var(
        &self,
        scope: usize,
        ident: &'a str,
    ) -> Option<&VariableDeclarationAssignment<'a>> {
        self.vars.lookup(&(scope, ident))
    }

    pub fn insert_var(
        mut self,
        scope: usize,
        ident: &'a str,
        var: VariableDeclaration<'a>,
    ) -> Self {
        self.vars = self.vars.insert((scope, ident), var.into());
        self
    }

    pub fn insert_var_ass(
        mut self,
        scope: usize,
        ident: &'a str,
        var: VariableDeclarationAssignment<'a>,
    ) -> Self {
        self.vars = self.vars.insert((scope, ident), var);
        self
    }

    pub fn lookup_fn_decl(
        &self,
        scope: usize,
        ident: &'a str,
    ) -> Option<&FunctionDeclaration<'a>> {
        self.fn_decls.lookup(&(scope, ident))
    }

    pub fn insert_fn_decl(mut self, scope: usize, ident: &'a str, d: FunctionDeclaration<'a>) -> Self {
        self.fn_decls = self.fn_decls.insert((scope, ident), d);
        self
    }

    pub fn lookup_fn_impl(
        &self,
        scope: usize,
        ident: &'a str,
    ) -> Option<&FunctionImplementation<'a>> {
        self.fn_impls.lookup(&(scope, ident))
    }

    pub fn insert_fn_impl(
        mut self,
        scope: usize,
        ident: &'a str,
        d: FunctionImplementation<'a>,
    ) -> Self {
        self.fn_impls = self.fn_impls.insert((scope, ident), d);
        self
    }

    pub fn lookup_trait_decl(&self, scope: usize, ident: &'a str) -> Option<&TraitDecl<'a>> {
        self.trait_decls.lookup(&(scope, ident))
    }

    pub fn insert_trait_decl(mut self, scope: usize, ident: &'a str, d: TraitDecl<'a>) -> Self {
        self.trait_decls = self.trait_decls.insert((scope, ident), d);
        self
    }

    pub fn lookup_trait_impl(&self, scope: usize, ident: &'a str) -> Option<&TraitImpl<'a>> {
        self.trait_impls.lookup(&(scope, ident))
    }

    pub fn insert_trait_impl(mut self, scope: usize, ident: &'a str, d: TraitImpl<'a>) -> Self {
        self.trait_impls = self.trait_impls.insert((scope, ident), d);
        self
    }

    pub fn lookup_struct_decl(
        &self,
        scope: usize,
        ident: &'a str,
    ) -> Option<&StructDeclaration<'a>> {
        self.struct_decls.lookup(&(scope, ident))
    }

    pub fn insert_struct_decl(
        mut self,
        scope: usize,
        ident: &'a str,
        d: StructDeclaration<'a>,
    ) -> Self {
        self.struct_decls = self.struct_decls.insert((scope, ident), d);
        self
    }

    pub fn lookup_type_decl(&self, scope: usize, ident: &'a str) -> Option<&TypeAlias<'a>> {
        self.type_decls.lookup(&(scope, ident))
    }

    pub fn insert_type_decl(mut self, scope: usize, ident: &'a str, d: TypeAlias<'a>) -> Self {
        self.type_decls = self.type_decls.insert((scope, ident), d);
        self
    }

    pub fn lookup_type(&self, scope: usize, ident: &'a str) -> Option<LexType<'a>> {
        self.lookup_type_decl(scope, ident)
            .map(AsB2Type::as_b2_type)
            .or_else(|| {
                self.lookup_struct_decl(scope, ident)
                    .map(AsB2Type::as_b2_type)
            })
            .or_else(|| self.lookup_fn_decl(scope, ident).map(AsB2Type::as_b2_type))
    }

    pub fn ident_is_available(&self, scope: usize, ident: &'a str) -> Result<(), String> {
        if let Some(var) = self.lookup_var(scope, ident) {
            return Err(format!(
                "Variable: {ident} already exists with value {}, and type {}",
                var.value().to_b2(),
                var.b2_type().cloned().unwrap_or_default().to_b2()
            ));
        }

        if let Some(_) = self.lookup_fn_decl(scope, ident) {
            return Err(format!("Function declaration: {ident} already exists",));
        }

        if let Some(_) = self.lookup_struct_decl(scope, ident) {
            return Err(format!("Structure declaration: {ident} already exists",));
        }

        if let Some(_) = self.lookup_trait_decl(scope, ident) {
            return Err(format!("Trait declaration: {ident} already exists",));
        }

        if let Some(typ) = self.lookup_type_decl(scope, ident) {
            return Err(format!(
                "Type declaration: {ident} already exists with type {}",
                typ.as_b2_type().to_b2()
            ));
        }

        Ok(())
    }
}
