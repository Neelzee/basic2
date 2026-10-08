use crate::symbol_table::SymbolTable;
use parser::{
    common::{AsB2Type, ToB2},
    lex_mod::LexModule,
    lex_stmt::{
        LexStmt,
        decls::{
            Decl,
            variants::{
                EnumDeclaration, FunctionDeclaration, StructDeclaration, TraitDecl, TupleUnpacking,
                TypeAlias, VariableDeclaration, VariableDeclarationAssignment,
            },
        },
        impls::{FunctionImplementation, Impl, TraitImpl},
    },
    lex_type::LexType,
};

#[derive(Debug, Clone)]
pub struct GlobalSymbolTable<'a> {
    current_scope: usize,
    vars: SymbolTable<'a, VariableDeclarationAssignment<'a>>,
    fn_decls: SymbolTable<'a, FunctionDeclaration<'a>>,
    fn_impls: SymbolTable<'a, FunctionImplementation<'a>>,
    trait_decls: SymbolTable<'a, TraitDecl<'a>>,
    trait_impls: SymbolTable<'a, TraitImpl<'a>>,
    struct_decls: SymbolTable<'a, StructDeclaration<'a>>,
    type_decls: SymbolTable<'a, TypeAlias<'a>>,
    enum_decls: SymbolTable<'a, EnumDeclaration<'a>>,
}

impl<'a> GlobalSymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            current_scope: 0,
            vars: SymbolTable::new(),
            fn_decls: SymbolTable::new(),
            fn_impls: SymbolTable::new(),
            trait_decls: SymbolTable::new(),
            trait_impls: SymbolTable::new(),
            struct_decls: SymbolTable::new(),
            type_decls: SymbolTable::new(),
            enum_decls: SymbolTable::new(),
        }
    }

    pub fn scope(self) -> Self {
        Self {
            current_scope: self.current_scope + 1,
            ..self
        }
    }

    pub fn drop_scope(self) -> Self {
        if self.current_scope == 0 {
            panic!("Cant drop global scope");
        }

        Self {
            current_scope: self.current_scope - 1,
            vars: self.vars.drop_scope(self.current_scope),
            fn_decls: self.fn_decls.drop_scope(self.current_scope),
            fn_impls: self.fn_impls.drop_scope(self.current_scope),
            trait_decls: self.trait_decls.drop_scope(self.current_scope),
            trait_impls: self.trait_impls.drop_scope(self.current_scope),
            struct_decls: self.struct_decls.drop_scope(self.current_scope),
            type_decls: self.type_decls.drop_scope(self.current_scope),
            enum_decls: self.enum_decls.drop_scope(self.current_scope),
        }
    }

    pub fn from_module(module: LexModule<'a>) -> Self {
        let mut gst = GlobalSymbolTable::new();
        for stmt in module.statements() {
            gst = match stmt {
                LexStmt::Decl(decl) => match decl {
                    Decl::VarDecl(var @ VariableDeclaration { identifier, .. }) => {
                        gst.insert_var(identifier, var)
                    }
                    Decl::VarDeclAss(var @ VariableDeclarationAssignment { identifier, .. }) => {
                        gst.insert_var_ass(identifier, var)
                    }
                    Decl::FnDecl(d @ FunctionDeclaration { identifier, .. }) => {
                        gst.insert_fn_decl(identifier, d)
                    }
                    Decl::StructDecl(d @ StructDeclaration { identifier, .. }) => {
                        gst.insert_struct_decl(identifier, d)
                    }
                    Decl::TypeDecl(d @ TypeAlias { identifier, .. }) => {
                        gst.insert_type_decl(identifier, d)
                    }
                    Decl::TraitDecl(d @ TraitDecl { identifier, .. }) => {
                        gst.insert_trait_decl(identifier, d)
                    }
                    Decl::EnumDecl(d @ EnumDeclaration { identifier, .. }) => {
                        gst.insert_enum_decl(identifier, d)
                    }
                    _ => gst,
                },
                LexStmt::Impl(impls) => match impls {
                    Impl::FnImpl(d @ FunctionImplementation { identifier, .. }) => {
                        gst.insert_fn_impl(identifier, d)
                    }
                    Impl::TraitImpl(
                        d @ TraitImpl {
                            trait_identifier, ..
                        },
                    ) => gst.insert_trait_impl(trait_identifier, d),
                },
                LexStmt::VariableReassignment { .. }
                | LexStmt::ListReassignment { .. }
                | LexStmt::If { .. }
                | LexStmt::While { .. }
                | LexStmt::Import(_)
                | LexStmt::Block { .. }
                | LexStmt::FunctionInvocation { .. }
                | LexStmt::Break
                | LexStmt::Continue
                | LexStmt::Return { .. }
                | LexStmt::For { .. }
                | LexStmt::StructFieldReassignment { .. }
                | LexStmt::WhenStatement { .. } => gst,
            };
        }

        gst
    }

    pub fn lookup_var(&self, ident: &'a str) -> Option<&VariableDeclarationAssignment<'a>> {
        self.vars.lookup((self.current_scope, ident))
    }

    pub fn insert_var(mut self, ident: &'a str, var: VariableDeclaration<'a>) -> Self {
        self.vars = self.vars.insert((self.current_scope, ident), var.into());
        self
    }

    pub fn insert_var_ass(
        mut self,
        ident: &'a str,
        var: VariableDeclarationAssignment<'a>,
    ) -> Self {
        self.vars = self.vars.insert((self.current_scope, ident), var);
        self
    }

    pub fn lookup_fn_decl(&self, ident: &'a str) -> Option<&FunctionDeclaration<'a>> {
        self.fn_decls.lookup((self.current_scope, ident))
    }

    pub fn insert_fn_decl(mut self, ident: &'a str, d: FunctionDeclaration<'a>) -> Self {
        self.fn_decls = self.fn_decls.insert((self.current_scope, ident), d);
        self
    }

    pub fn lookup_fn_impl(&self, ident: &'a str) -> Option<&FunctionImplementation<'a>> {
        self.fn_impls.lookup((self.current_scope, ident))
    }

    pub fn insert_fn_impl(mut self, ident: &'a str, d: FunctionImplementation<'a>) -> Self {
        self.fn_impls = self.fn_impls.insert((self.current_scope, ident), d);
        self
    }

    pub fn lookup_trait_decl(&self, ident: &'a str) -> Option<&TraitDecl<'a>> {
        self.trait_decls.lookup((self.current_scope, ident))
    }

    pub fn insert_trait_decl(mut self, ident: &'a str, d: TraitDecl<'a>) -> Self {
        self.trait_decls = self.trait_decls.insert((self.current_scope, ident), d);
        self
    }

    pub fn lookup_trait_impl(&self, ident: &'a str) -> Option<&TraitImpl<'a>> {
        self.trait_impls.lookup((self.current_scope, ident))
    }

    pub fn insert_trait_impl(mut self, ident: &'a str, d: TraitImpl<'a>) -> Self {
        self.trait_impls = self.trait_impls.insert((self.current_scope, ident), d);
        self
    }

    pub fn lookup_struct_decl(&self, ident: &'a str) -> Option<&StructDeclaration<'a>> {
        self.struct_decls.lookup((self.current_scope, ident))
    }

    pub fn insert_struct_decl(mut self, ident: &'a str, d: StructDeclaration<'a>) -> Self {
        self.struct_decls = self.struct_decls.insert((self.current_scope, ident), d);
        self
    }

    pub fn lookup_type_decl(&self, ident: &'a str) -> Option<&TypeAlias<'a>> {
        self.type_decls.lookup((self.current_scope, ident))
    }

    pub fn insert_type_decl(mut self, ident: &'a str, d: TypeAlias<'a>) -> Self {
        self.type_decls = self.type_decls.insert((self.current_scope, ident), d);
        self
    }

    pub fn lookup_enum_decl(&self, ident: &'a str) -> Option<&EnumDeclaration<'a>> {
        self.enum_decls.lookup((self.current_scope, ident))
    }

    pub fn insert_enum_decl(mut self, ident: &'a str, d: EnumDeclaration<'a>) -> Self {
        self.enum_decls = self.enum_decls.insert((self.current_scope, ident), d);
        self
    }

    pub fn lookup_type(&self, ident: &'a str) -> Option<LexType<'a>> {
        self.lookup_type_decl(ident)
            .map(AsB2Type::as_b2_type)
            .or_else(|| self.lookup_struct_decl(ident).map(AsB2Type::as_b2_type))
            .or_else(|| self.lookup_fn_decl(ident).map(AsB2Type::as_b2_type))
    }

    pub fn ident_is_available(&self, ident: &'a str) -> Result<(), String> {
        if let Some(var) = self.lookup_var(ident) {
            return Err(format!(
                "Variable: {ident} already exists with value {}, and type {}",
                var.value().to_b2(),
                var.b2_type().cloned().unwrap_or_default().to_b2()
            ));
        }

        if let Some(_) = self.lookup_fn_decl(ident) {
            return Err(format!("Function declaration: {ident} already exists",));
        }

        if let Some(_) = self.lookup_struct_decl(ident) {
            return Err(format!("Structure declaration: {ident} already exists",));
        }

        if let Some(_) = self.lookup_trait_decl(ident) {
            return Err(format!("Trait declaration: {ident} already exists",));
        }

        if let Some(typ) = self.lookup_type_decl(ident) {
            return Err(format!(
                "Type declaration: {ident} already exists with type {}",
                typ.as_b2_type().to_b2()
            ));
        }

        if let Some(_) = self.lookup_enum_decl(ident) {
            return Err(format!("Type declaration: {ident} already exists.",));
        }

        Ok(())
    }
}
