use crate::{
    common::binop::BinOp,
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::{
            decls::{
                Decl,
                variants::{
                    EnumDeclaration, FunctionDeclaration, ListUnpacking, StructUnpacking,
                    TupleUnpacking, VariableDeclaration, VariableDeclarationAssignment,
                },
            },
            impls::Impl,
            when_match::WhenMatch,
        },
        lex_type::LexType,
    },
};

pub mod decls;
pub mod impls;
mod parser;
pub mod when_match;

type FunImplComps<'a> = (
    &'a str,
    Vec<(&'a str, Option<LexExpr<'a>>)>,
    Vec<LexStmt<'a>>,
);
type FunDeclComps<'a> = (
    &'a str,
    Vec<(&'a str, Vec<&'a str>)>,
    Vec<LexType<'a>>,
    Option<LexType<'a>>,
);

#[derive(Debug, PartialEq, Clone)]
pub enum LexStmt<'a> {
    Decl(Decl<'a>),
    Impl(Impl<'a>),
    Import(Import<'a>),
    VariableReassignment {
        identifier: &'a str,
        reassignment: Option<BinOp>,
        new_value: LexExpr<'a>,
    },
    ListReassignment {
        indexee: LexExpr<'a>,
        index: LexExpr<'a>,
        reassignment: Option<BinOp>,
        new_value: LexExpr<'a>,
    },
    If {
        condition: LexExpr<'a>,
        body: Vec<Self>,
    },
    While {
        condition: LexExpr<'a>,
        body: Vec<Self>,
    },
    Block {
        body: Vec<Self>,
    },
    FunctionInvocation {
        identifier: &'a str,
        arguments: Vec<LexExpr<'a>>,
    },
    Break,
    Continue,
    Return {
        value: Option<LexExpr<'a>>,
    },
    For {
        start_stmt: Box<Self>,
        condition: LexExpr<'a>,
        incrementer: LexExpr<'a>,
        body: Vec<Self>,
    },
    StructFieldReassignment {
        identifier: &'a str,
        field: &'a str,
        reassignment: Option<BinOp>,
        new_value: LexExpr<'a>,
    },
    WhenStatement {
        identifier: &'a str,
        branches: Vec<(WhenMatch<'a>, Vec<Self>)>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import<'a> {
    identifier: &'a str,
}

impl<'a> LexStmt<'a> {
    pub fn as_decl(self) -> Option<Decl<'a>> {
        match self {
            Self::Decl(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_impl(self) -> Option<Impl<'a>> {
        match self {
            Self::Impl(i) => Some(i),
            _ => None,
        }
    }

    pub fn new_var_decl(identifier: &'a str, variable_type: LexType<'a>) -> Self {
        Self::Decl(Decl::VarDecl(VariableDeclaration::new(
            identifier,
            variable_type,
        )))
    }

    pub fn new_var_decl_ass(
        identifier: &'a str,
        variable_type: Option<LexType<'a>>,
        value: LexExpr<'a>,
    ) -> Self {
        Self::Decl(Decl::VarDeclAss(VariableDeclarationAssignment::new(
            identifier,
            variable_type,
            value,
        )))
    }

    pub fn new_fn_decl(
        identifier: &'a str,
        parameters: Vec<LexType<'a>>,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        return_type: Option<LexType<'a>>,
    ) -> Self {
        Self::Decl(Decl::FnDecl(FunctionDeclaration::new(
            identifier,
            parameters,
            generics,
            return_type,
        )))
    }

    pub fn new_fn_impl(
        identifier: &'a str,
        parameters: Vec<(&'a str, Option<LexExpr<'a>>)>,
        body: Vec<Self>,
    ) -> Self {
        Self::Impl(Impl::new_fn(identifier, parameters, body))
    }

    pub fn new_struct_decl(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        fields: Vec<(&'a str, LexType<'a>)>,
    ) -> Self {
        Self::Decl(Decl::new_struct(identifier, generics, fields))
    }

    pub fn new_type_alias(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        b2_type: LexType<'a>,
    ) -> Self {
        Self::Decl(Decl::new_type(identifier, generics, b2_type))
    }

    pub fn new_tuple_unpack(identifiers: Vec<&'a str>, value: LexExpr<'a>) -> Self {
        Self::Decl(Decl::TupUnpk(TupleUnpacking::new(identifiers, value)))
    }

    pub fn new_list_unpack(
        identifiers: Vec<&'a str>,
        remainder: Option<&'a str>,
        value: LexExpr<'a>,
    ) -> Self {
        Self::Decl(Decl::LstUnpk(ListUnpacking::new(
            identifiers,
            remainder,
            value,
        )))
    }

    pub fn new_struct_unpack(identifiers: Vec<&'a str>, value: LexExpr<'a>) -> Self {
        Self::Decl(Decl::StrUnpk(StructUnpacking::new(identifiers, value)))
    }

    pub fn new_enum_decl(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        enumerations: Vec<(&'a str, Vec<LexType<'a>>)>,
    ) -> Self {
        Self::Decl(Decl::EnumDecl(EnumDeclaration::new(
            identifier,
            generics,
            enumerations,
        )))
    }

    pub fn new_trait_impl(
        trait_identifier: &'a str,
        type_identifier: &'a str,
        body: Vec<LexStmt<'a>>,
    ) -> Self {
        Self::Impl(Impl::new_trait(trait_identifier, type_identifier, body))
    }

    pub fn new_trait_decl(
        identifier: &'a str,
        restrictions: Vec<&'a str>,
        decls: Vec<FunDeclComps<'a>>,
        impls: Vec<FunImplComps<'a>>,
    ) -> Self {
        Self::Decl(Decl::new_trait(identifier, restrictions, decls, impls))
    }
}
