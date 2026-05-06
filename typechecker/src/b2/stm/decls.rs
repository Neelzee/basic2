use crate::b2::{
    exp::B2Expr,
    stm::{FunDeclComps, FunImplComps},
    typ::B2Type,
};

#[derive(Debug, Clone)]
pub enum Decl<'a> {
    VarDecl(VarDecl<'a>),
    TupleUnPack(TupleUnPack<'a>),
    ListUnPack(ListUnPack<'a>),
    StructUnPack(StructUnPack<'a>),
    FnDecl(FnDecl<'a>),
    StructDecl(StructDecl<'a>),
    TypeAlias(TypeAlias<'a>),
    EnumDecl(EnumDecl<'a>),
    TraitDecl(TraitDecl<'a>),
    Import(Import<'a>),
}

impl<'a> Decl<'a> {
    pub fn as_var(&'a self) -> Option<&'a VarDecl<'a>> {
        match self {
            Decl::VarDecl(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_fn(&'a self) -> Option<&'a FnDecl<'a>> {
        match self {
            Decl::FnDecl(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_import(&'a self) -> Option<&'a Import<'a>> {
        match self {
            Decl::Import(i) => Some(i),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarDecl<'a> {
    ident: &'a str,
    b2_type: Option<B2Type<'a>>,
    val: Option<B2Expr<'a>>,
}

impl<'a> VarDecl<'a> {
    pub fn get_type(&'a self) -> Option<&'a B2Type<'a>> {
        self.b2_type.as_ref()
    }

    pub fn get_expr(&'a self) -> Option<&'a B2Expr<'a>> {
        self.val.as_ref()
    }

    pub(crate) fn new(
        ident: &'a str,
        b2_type: Option<B2Type<'a>>,
        val: Option<B2Expr<'a>>,
    ) -> Self {
        Self {
            ident,
            b2_type,
            val,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TupleUnPack<'a> {
    idents: Vec<&'a str>,
    val: B2Expr<'a>,
}

#[derive(Debug, Clone)]
pub struct ListUnPack<'a> {
    idents: Vec<&'a str>,
    rem: Option<&'a str>,
    val: B2Expr<'a>,
}

#[derive(Debug, Clone)]
pub struct StructUnPack<'a> {
    idents: Vec<&'a str>,
    val: B2Expr<'a>,
}

#[derive(Debug, Clone)]
pub struct FnDecl<'a> {
    idents: &'a str,
    pars: Vec<B2Type<'a>>,
    gens: Vec<(&'a str, Vec<&'a str>)>,
    ret: B2Type<'a>,
}

impl<'a> FnDecl<'a> {
    pub fn get_return(&'a self) -> &'a B2Type<'a> {
        &self.ret
    }
}

#[derive(Debug, Clone)]
pub struct StructDecl<'a> {
    ident: &'a str,
    fields: Vec<(&'a str, B2Type<'a>)>,
}

#[derive(Debug, Clone)]
pub struct TypeAlias<'a> {
    ident: &'a str,
    b2_type: B2Type<'a>,
}

#[derive(Debug, Clone)]
pub struct EnumDecl<'a> {
    ident: &'a str,
    vars: Vec<&'a str>,
}

#[derive(Debug, Clone)]
pub struct TraitDecl<'a> {
    ident: &'a str,
    decls: Vec<FunDeclComps<'a>>,
    impls: Vec<FunImplComps<'a>>,
}

#[derive(Debug, Clone)]
pub struct Import<'a>(&'a str);

impl<'a> Import<'a> {
    pub fn module(&self) -> &str {
        self.0
    }
}
