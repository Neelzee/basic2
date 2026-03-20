use crate::lexer::{
    lex_stmt::{
        FunDeclComps, FunImplComps,
        decls::variants::{
            EnumDeclaration, FunctionDeclaration, ListUnpacking, StructDeclaration,
            StructUnpacking, TraitDecl, TupleUnpacking, TypeAlias, VariableDeclaration,
            VariableDeclarationAssignment,
        },
    },
    lex_type::LexType,
};

pub mod variants;

#[derive(Debug, PartialEq, Clone)]
pub enum Decl<'a> {
    VarDecl(VariableDeclaration<'a>),
    TupUnpk(TupleUnpacking<'a>),
    StrUnpk(StructUnpacking<'a>),
    LstUnpk(ListUnpacking<'a>),
    VarDeclAss(VariableDeclarationAssignment<'a>),
    FnDecl(FunctionDeclaration<'a>),
    StructDecl(StructDeclaration<'a>),
    TypeDecl(TypeAlias<'a>),
    TraitDecl(TraitDecl<'a>),
    EnumDecl(EnumDeclaration<'a>),
}

impl<'a> Decl<'a> {
    pub fn as_var_decl(self) -> Option<VariableDeclaration<'a>> {
        match self {
            Decl::VarDecl(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_tuple_unpack(self) -> Option<TupleUnpacking<'a>> {
        match self {
            Decl::TupUnpk(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_struct_unpack(self) -> Option<StructUnpacking<'a>> {
        match self {
            Decl::StrUnpk(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_lst_unpack(self) -> Option<ListUnpacking<'a>> {
        match self {
            Decl::LstUnpk(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_var_decl_ass(self) -> Option<VariableDeclarationAssignment<'a>> {
        match self {
            Decl::VarDeclAss(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_fn_decl(self) -> Option<FunctionDeclaration<'a>> {
        match self {
            Decl::FnDecl(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_struct_decl(self) -> Option<StructDeclaration<'a>> {
        match self {
            Decl::StructDecl(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_type_decl(self) -> Option<TypeAlias<'a>> {
        match self {
            Decl::TypeDecl(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_trait_decl(self) -> Option<TraitDecl<'a>> {
        match self {
            Decl::TraitDecl(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_enum_decl(self) -> Option<EnumDeclaration<'a>> {
        match self {
            Decl::EnumDecl(v) => Some(v),
            _ => None,
        }
    }

    pub fn new_struct(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        fields: Vec<(&'a str, LexType<'a>)>,
    ) -> Self {
        Self::StructDecl(StructDeclaration::new(identifier, generics, fields))
    }

    pub fn new_type(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        b2_type: LexType<'a>,
    ) -> Self {
        Self::TypeDecl(TypeAlias::new(identifier, generics, b2_type))
    }

    pub fn new_trait(
        identifier: &'a str,
        restrictions: Vec<&'a str>,
        decls: Vec<FunDeclComps<'a>>,
        impls: Vec<FunImplComps<'a>>,
    ) -> Self {
        Self::TraitDecl(TraitDecl::new(identifier, restrictions, decls, impls))
    }

    pub fn new_fn(
        identifier: &'a str,
        parameters: Vec<LexType<'a>>,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        return_type: Option<LexType<'a>>,
    ) -> Self {
        Self::FnDecl(FunctionDeclaration::new(
            identifier,
            parameters,
            generics,
            return_type,
        ))
    }
}
