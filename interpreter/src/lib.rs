use crate::symbol_table::SymbolTable;
use parser::{
    common::{AsB2Type, B2OpInner, ToB2, binop::BinOp, postfix::Postfix, primitive::Primitive},
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::{
            LexStmt,
            decls::variants::{
                FunctionDeclaration, StructDeclaration, TraitDecl, TypeAlias, VariableDeclaration,
                VariableDeclarationAssignment,
            },
            impls::{FunctionImplementation, TraitImpl},
        },
        lex_type::LexType,
    },
};

mod interpreter;
mod symbol_table;

pub struct Interpreter<'a> {
    scope: usize,
    vars: SymbolTable<'a, VariableDeclarationAssignment<'a>>,
    fn_decls: SymbolTable<'a, FunctionDeclaration<'a>>,
    fn_impls: SymbolTable<'a, FunctionImplementation<'a>>,
    trait_decls: SymbolTable<'a, TraitDecl<'a>>,
    trait_impls: SymbolTable<'a, TraitImpl<'a>>,
    struct_decls: SymbolTable<'a, StructDeclaration<'a>>,
    type_decls: SymbolTable<'a, TypeAlias<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn enter_scope<F>(&'a mut self, f: F) -> LexExpr<'a>
    where
        F: FnOnce(&mut Self) -> LexExpr<'a>,
    {
        self.scope += 1;
        let result = f(self);
        self.scope -= 1;
        result
    }

    pub fn lookup_var(&'a self, ident: &'a str) -> Option<&'a VariableDeclarationAssignment<'a>> {
        self.vars.lookup_scope(&(self.scope, ident))
    }

    pub fn insert_var(&'a mut self, ident: &'a str, var: VariableDeclaration<'a>) {
        self.vars.insert((self.scope, ident), var.into());
    }

    pub fn insert_var_ass(&'a mut self, ident: &'a str, var: VariableDeclarationAssignment<'a>) {
        self.vars.insert((self.scope, ident), var);
    }

    pub fn lookup_fn_decl(&'a self, ident: &'a str) -> Option<&'a FunctionDeclaration<'a>> {
        self.fn_decls.lookup_scope(&(self.scope, ident))
    }

    pub fn insert_fn_decl(&'a mut self, ident: &'a str, d: FunctionDeclaration<'a>) {
        self.fn_decls.insert((self.scope, ident), d);
    }

    pub fn lookup_fn_impl(&'a self, ident: &'a str) -> Option<&'a FunctionImplementation<'a>> {
        self.fn_impls.lookup_scope(&(self.scope, ident))
    }

    pub fn insert_fn_impl(&'a mut self, ident: &'a str, d: FunctionImplementation<'a>) {
        self.fn_impls.insert((self.scope, ident), d);
    }

    pub fn lookup_trait_decl(&'a self, ident: &'a str) -> Option<&'a TraitDecl<'a>> {
        self.trait_decls.lookup_scope(&(self.scope, ident))
    }

    pub fn insert_trait_decl(&'a mut self, ident: &'a str, d: TraitDecl<'a>) {
        self.trait_decls.insert((self.scope, ident), d);
    }

    pub fn lookup_trait_impl(&'a self, ident: &'a str) -> Option<&'a TraitImpl<'a>> {
        self.trait_impls.lookup_scope(&(self.scope, ident))
    }

    pub fn insert_trait_impl(&'a mut self, ident: &'a str, d: TraitImpl<'a>) {
        self.trait_impls.insert((self.scope, ident), d);
    }

    pub fn lookup_struct_decl(&'a self, ident: &'a str) -> Option<&'a StructDeclaration<'a>> {
        self.struct_decls.lookup_scope(&(self.scope, ident))
    }

    pub fn insert_struct_decl(&'a mut self, ident: &'a str, d: StructDeclaration<'a>) {
        self.struct_decls.insert((self.scope, ident), d);
    }

    pub fn lookup_type_decl(&'a self, ident: &'a str) -> Option<&'a TypeAlias<'a>> {
        self.type_decls.lookup_scope(&(self.scope, ident))
    }

    pub fn insert_type_decl(&'a mut self, ident: &'a str, d: TypeAlias<'a>) {
        self.type_decls.insert((self.scope, ident), d);
    }

    pub fn lookup_type(&'a self, ident: &'a str) -> Option<LexType<'a>> {
        self.lookup_type_decl(ident)
            .map(AsB2Type::as_b2_type)
            .or_else(|| self.lookup_struct_decl(ident).map(AsB2Type::as_b2_type))
            .or_else(|| self.lookup_fn_decl(ident).map(AsB2Type::as_b2_type))
    }

    pub fn ident_is_available(&'a self, ident: &'a str) -> Result<(), String> {
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

        Ok(())
    }

    pub fn interpret(&'a mut self, stmt: LexStmt<'a>) -> Result<(), String> {
        match stmt {
            LexStmt::Decl(decl) => todo!(),
            LexStmt::Impl(_) => todo!(),
            LexStmt::Import(import) => todo!(),
            LexStmt::VariableReassignment {
                identifier,
                reassignment,
                new_value,
            } => todo!(),
            LexStmt::ListReassignment {
                indexee,
                index,
                reassignment,
                new_value,
            } => todo!(),
            LexStmt::If { condition, body } => todo!(),
            LexStmt::While { condition, body } => todo!(),
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
        }
    }

    fn unpack_expr(&'a self, expr: LexExpr<'a>) -> Result<Vec<LexExpr<'a>>, String> {
        match expr {
            LexExpr::Nil => Err("Cannot unpack NIL value".to_string()),
            e @ LexExpr::Literal(_) => Ok(vec![e]),
            LexExpr::Tuple(fst, snd) => {
                let mut fst = self.unpack_expr(*fst)?;
                let mut snd = self.unpack_expr(*snd)?;
                fst.append(&mut snd);
                Ok(fst)
            }
            LexExpr::List(lex_exprs) => Ok(lex_exprs),
            LexExpr::Variable(ident) => match self.lookup_var(ident) {
                Some(var) => self.unpack_expr(var.value().clone()),
                None => Err(format!("Variable {ident} is not defined")),
            },
            LexExpr::Group(lex_expr) => self.unpack_expr(*lex_expr),
            LexExpr::FunctionCall {
                identifier,
                arguments,
            } => todo!(),
            LexExpr::Op(b2_op) => todo!(),
            LexExpr::Struct {
                identifier,
                field_implementations,
            } => todo!(),
            LexExpr::StructFieldAccessing { identifier, field } => todo!(),
            LexExpr::Enum {
                identifier,
                instance,
                values,
            } => todo!(),
        }
    }

    fn function_call(
        &'a mut self,
        ident: &'a str,
        args: Vec<LexExpr<'a>>,
    ) -> Result<LexExpr<'a>, String> {
        let fun = self
            .lookup_fn_impl(ident)
            .ok_or(format!("Missing function implementation: {ident}"))?;

        let args_len = args.len();
        let pars_len = fun.parameters().len();
        if args_len > pars_len {
            return Err(format!(
                "Too many arguments supplied to function: {ident}, expected {pars_len}, got {args_len}"
            ));
        }

        for (val, (i, opt)) in args
            .into_iter()
            .map(|e| Some(e))
            .chain((0..).into_iter().map(|_| None))
            .zip(fun.parameters())
        {
            match (&val, opt) {
                (None, Some(val)) | (Some(val), _) => {
                    let t = self.get_type(val.clone());
                    self.insert_var_ass(
                        i,
                        VariableDeclarationAssignment::new(i, Some(t), val.clone()),
                    );
                }
                (None, None) => {
                    return Err(format!(
                        "Missing value to argument {i}, in function {ident}"
                    ));
                }
            }
        }

        for stmt in fun.body() {
            match stmt {
                LexStmt::Return { value } => match value {
                    Some(val) => {
                        return Ok(val.clone());
                    }
                    None => {
                        return Ok(LexExpr::Nil);
                    }
                },
                _ => {
                    self.interpret(stmt.clone());
                    todo!()
                }
            }
        }
        Ok(LexExpr::Nil)
    }

    fn get_type(&self, expr: LexExpr<'_>) -> LexType<'_> {
        match expr {
            LexExpr::Nil => LexType::Nil,
            LexExpr::Literal(primitive) => match primitive {
                Primitive::Int(_) => LexType::Int,
                Primitive::Float(_) => LexType::Float,
                Primitive::Str(_) => LexType::Str,
                Primitive::Bool(_) => LexType::Bool,
            },
            LexExpr::Tuple(f, s) => LexType::Tuple {
                fst: Box::new(self.get_type(*f)),
                snd: Box::new(self.get_type(*s)),
            },
            LexExpr::List(lex_exprs) => match lex_exprs.last() {
                Some(e) => LexType::List(Box::new(self.get_type(e))),
                None => LexType::List(Box::new(LexType::Nil)),
            },
            LexExpr::Variable(ident) => self.symbol_table.lookup_type(ident).unwrap_or_default(),
            LexExpr::Group(lex_expr) => self.get_type(*lex_expr),
            LexExpr::FunctionCall { identifier, .. } => self
                .symbol_table
                .lookup_type(identifier)
                .unwrap_or_default(),
            LexExpr::Op(op) => match op.inner() {
                B2OpInner::Prefix(_, _) => LexType::Bool,
                B2OpInner::Postfix(e, o) => match o {
                    Postfix::Incr | Postfix::Decr => LexType::Int,
                    Postfix::Index(_) => match self.get_type(e.clone()) {
                        LexType::List(lex_type) => *lex_type.clone(),
                        _ => LexType::default(),
                    },
                },
                B2OpInner::Binary(l, op, r) => match op {
                    BinOp::Add => {
                        let lt = self.get_type(l.clone());
                        if lt == LexType::Str {
                            lt
                        } else if self.get_type(r.clone()) == LexType::Float || lt == LexType::Float
                        {
                            LexType::Float
                        } else {
                            lt
                        }
                    }
                    BinOp::Sub | BinOp::Div | BinOp::Pow => {
                        let lt = self.get_type(l.clone());
                        if self.get_type(r.clone()) == LexType::Float || lt == LexType::Float {
                            LexType::Float
                        } else {
                            lt
                        }
                    }
                    BinOp::Mul => {
                        let lt = self.get_type(l.clone());
                        let rt = self.get_type(r.clone());
                        if rt == LexType::Int && lt == LexType::Str {
                            lt
                        } else if rt == LexType::Float || lt == LexType::Float {
                            LexType::Float
                        } else {
                            lt
                        }
                    }
                    BinOp::Eq
                    | BinOp::Geq
                    | BinOp::Gt
                    | BinOp::Leq
                    | BinOp::Lt
                    | BinOp::Neq
                    | BinOp::And
                    | BinOp::Or => LexType::Bool,
                    BinOp::Mod => LexType::Int,
                },
            },
            LexExpr::Struct { identifier, .. } => self
                .symbol_table
                .lookup_type(identifier)
                .unwrap_or_default(),
            LexExpr::StructFieldAccessing { identifier, field } => {
                match self.symbol_table.lookup_struct_decl(identifier) {
                    Some(xs) => xs
                        .iter()
                        .find(|(k, _)| k == &field)
                        .map(|(_, x)| x.clone())
                        .unwrap_or_default(),
                    None => LexType::Nil,
                }
            }
            LexExpr::Enum {
                identifier,
                instance,
                ..
            } => LexType::EnumVariant(identifier, instance),
        }
    }
}
