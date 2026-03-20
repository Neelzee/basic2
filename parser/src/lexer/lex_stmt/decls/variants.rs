use crate::lexer::{
    lex_expr::LexExpr,
    lex_stmt::{FunDeclComps, FunImplComps},
    lex_type::LexType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration<'a> {
    identifier: &'a str,
    variable_type: LexType<'a>,
}

impl<'a> VariableDeclaration<'a> {
    pub fn new(identifier: &'a str, variable_type: LexType<'a>) -> Self {
        Self {
            identifier,
            variable_type,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleUnpacking<'a> {
    identifiers: Vec<&'a str>,
    value: LexExpr<'a>,
}

impl<'a> TupleUnpacking<'a> {
    pub fn new(identifiers: Vec<&'a str>, value: LexExpr<'a>) -> Self {
        Self { identifiers, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructUnpacking<'a> {
    identifiers: Vec<&'a str>,
    value: LexExpr<'a>,
}

impl<'a> StructUnpacking<'a> {
    pub fn new(identifiers: Vec<&'a str>, value: LexExpr<'a>) -> Self {
        Self { identifiers, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListUnpacking<'a> {
    identifiers: Vec<&'a str>,
    remainder: Option<&'a str>,
    value: LexExpr<'a>,
}

impl<'a> ListUnpacking<'a> {
    pub fn new(identifiers: Vec<&'a str>, remainder: Option<&'a str>, value: LexExpr<'a>) -> Self {
        Self {
            identifiers,
            remainder,
            value,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationAssignment<'a> {
    identifier: &'a str,
    variable_type: Option<LexType<'a>>,
    value: LexExpr<'a>,
}

impl<'a> VariableDeclarationAssignment<'a> {
    pub fn new(
        identifier: &'a str,
        variable_type: Option<LexType<'a>>,
        value: LexExpr<'a>,
    ) -> Self {
        Self {
            identifier,
            variable_type,
            value,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration<'a> {
    identifier: &'a str,
    parameters: Vec<LexType<'a>>,
    generics: Vec<(&'a str, Vec<&'a str>)>,
    return_type: Option<LexType<'a>>,
}

impl<'a> FunctionDeclaration<'a> {
    pub fn new(
        identifier: &'a str,
        parameters: Vec<LexType<'a>>,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        return_type: Option<LexType<'a>>,
    ) -> Self {
        Self {
            identifier,
            parameters,
            generics,
            return_type,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructDeclaration<'a> {
    identifier: &'a str,
    generics: Vec<(&'a str, Vec<&'a str>)>,
    fields: Vec<(&'a str, LexType<'a>)>,
}

impl<'a> StructDeclaration<'a> {
    pub fn new(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        fields: Vec<(&'a str, LexType<'a>)>,
    ) -> Self {
        Self {
            identifier,
            generics,
            fields,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAlias<'a> {
    identifier: &'a str,
    generics: Vec<(&'a str, Vec<&'a str>)>,
    b2_type: LexType<'a>,
}

impl<'a> TypeAlias<'a> {
    pub fn new(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        b2_type: LexType<'a>,
    ) -> Self {
        Self {
            identifier,
            generics,
            b2_type,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumDeclaration<'a> {
    identifier: &'a str,
    generics: Vec<(&'a str, Vec<&'a str>)>,
    enumerations: Vec<(&'a str, Vec<LexType<'a>>)>,
}

impl<'a> EnumDeclaration<'a> {
    pub fn new(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        enumerations: Vec<(&'a str, Vec<LexType<'a>>)>,
    ) -> Self {
        Self {
            identifier,
            generics,
            enumerations,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitDecl<'a> {
    identifier: &'a str,
    restrictions: Vec<&'a str>,
    decls: Vec<FunDeclComps<'a>>,
    impls: Vec<FunImplComps<'a>>,
}

impl<'a> TraitDecl<'a> {
    pub fn new(
        identifier: &'a str,
        restrictions: Vec<&'a str>,
        decls: Vec<FunDeclComps<'a>>,
        impls: Vec<FunImplComps<'a>>,
    ) -> Self {
        Self {
            identifier,
            restrictions,
            decls,
            impls,
        }
    }
}
