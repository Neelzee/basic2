use std::collections::HashMap;

use crate::{
    common::AsB2Type,
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::{FunDeclComps, FunImplComps},
        lex_type::{LexMonoType, LexPolyType, LexType},
    },
};

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration<'a> {
    pub identifier: &'a str,
    pub variable_type: LexType<'a>,
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

    pub fn identifier(&self) -> &str {
        self.identifier
    }

    pub fn value(&self) -> &LexExpr<'a> {
        &self.value
    }

    pub fn b2_type(&self) -> Option<&LexType<'a>> {
        self.variable_type.as_ref()
    }
}

impl<'a> From<VariableDeclaration<'a>> for VariableDeclarationAssignment<'a> {
    fn from(value: VariableDeclaration<'a>) -> Self {
        Self {
            identifier: value.identifier,
            variable_type: Some(value.variable_type),
            value: LexExpr::default(),
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

impl<'a> AsB2Type<'a> for FunctionDeclaration<'a> {
    fn as_b2_type(&self) -> LexType<'a> {
        match self.parameters.first() {
            Some(input) => {
                let mut pars = vec![input.clone()];
                let mut parameters = self.parameters.clone();
                pars.append(&mut parameters);
                pars.push(self.return_type.clone().unwrap_or_default());
                LexType::fold_funs(pars).unwrap_or(LexType::Poly(LexPolyType::FnType {
                    input: Box::new(LexType::default()),
                    output: Box::new(self.return_type.clone().unwrap_or_default()),
                }))
            }
            None => LexType::Poly(LexPolyType::FnType {
                input: Box::new(LexType::default()),
                output: Box::new(self.return_type.clone().unwrap_or_default()),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructDeclaration<'a> {
    identifier: &'a str,
    generics: Vec<(&'a str, Vec<&'a str>)>,
    fields: HashMap<&'a str, LexType<'a>>,
}

impl<'a> StructDeclaration<'a> {
    pub fn new(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        fields: HashMap<&'a str, LexType<'a>>,
    ) -> Self {
        Self {
            identifier,
            generics,
            fields,
        }
    }

    pub fn has_field(&self, field: &str) -> bool {
        self.fields.contains_key(field)
    }

    pub fn get_field(&self, field: &str) -> Option<&LexType<'a>> {
        self.fields.get(field)
    }
}

impl<'a> AsB2Type<'a> for StructDeclaration<'a> {
    fn as_b2_type(&self) -> LexType<'a> {
        LexType::Mono(LexMonoType::TypeVar(self.identifier))
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

impl<'a> AsB2Type<'a> for TypeAlias<'a> {
    fn as_b2_type(&self) -> LexType<'a> {
        self.b2_type.clone()
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
