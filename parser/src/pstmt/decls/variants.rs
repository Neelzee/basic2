use std::collections::HashMap;
use crate::{
    pexpr::PExpr, pstmt::{FunDeclComps, FunImplComps}, ptypes::TypeParser,
};
use common::{AsB2Type, types::{MonoType, PolyType, Type}};

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration<'a> {
    pub identifier: &'a str,
    pub variable_type: Type<'a>,
}

impl<'a> VariableDeclaration<'a> {
    pub fn new(identifier: &'a str, variable_type: Type<'a>) -> Self {
        Self {
            identifier,
            variable_type,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleUnpacking<'a> {
    pub identifiers: Vec<&'a str>,
    pub value: PExpr<'a>,
}

impl<'a> TupleUnpacking<'a> {
    pub fn new(identifiers: Vec<&'a str>, value: PExpr<'a>) -> Self {
        Self { identifiers, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructUnpacking<'a> {
    pub identifiers: Vec<(Option<&'a str>, &'a str)>,
    pub value: PExpr<'a>,
}

impl<'a> StructUnpacking<'a> {
    pub fn new(identifiers: Vec<(Option<&'a str>, &'a str)>, value: PExpr<'a>) -> Self {
        Self { identifiers, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListUnpacking<'a> {
    pub identifiers: Vec<&'a str>,
    pub remainder: Option<&'a str>,
    pub value: PExpr<'a>,
}

impl<'a> ListUnpacking<'a> {
    pub fn new(identifiers: Vec<&'a str>, remainder: Option<&'a str>, value: PExpr<'a>) -> Self {
        Self {
            identifiers,
            remainder,
            value,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationAssignment<'a> {
    pub identifier: &'a str,
    pub variable_type: Option<Type<'a>>,
    pub value: PExpr<'a>,
}

impl<'a> VariableDeclarationAssignment<'a> {
    pub fn new(
        identifier: &'a str,
        variable_type: Option<Type<'a>>,
        value: PExpr<'a>,
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

    pub fn value(&self) -> &PExpr<'a> {
        &self.value
    }

    pub fn b2_type(&self) -> Option<&Type<'a>> {
        self.variable_type.as_ref()
    }
}

impl<'a> From<VariableDeclaration<'a>> for VariableDeclarationAssignment<'a> {
    fn from(value: VariableDeclaration<'a>) -> Self {
        Self {
            identifier: value.identifier,
            variable_type: Some(value.variable_type),
            value: PExpr::default(),
        }
    }
}

/* NOTE: Should I turn this into an enum, and have the builtins be specific
  enums? Like PRINT, LEN, etc?
*/
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration<'a> {
    pub identifier: &'a str,
    pub parameters: Vec<Type<'a>>,
    pub generics: Vec<(&'a str, Vec<&'a str>)>,
    pub return_type: Option<Type<'a>>,
}

impl<'a> FunctionDeclaration<'a> {
    pub fn new(
        identifier: &'a str,
        parameters: Vec<Type<'a>>,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        return_type: Option<Type<'a>>,
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
    fn as_b2_type(&self) -> Type<'a> {
        match self.parameters.first() {
            Some(input) => {
                let mut pars = vec![input.clone()];
                let mut parameters = self.parameters.clone();
                pars.append(&mut parameters);
                pars.push(self.return_type.clone().unwrap_or_default());
                TypeParser::fold_funs(pars).unwrap_or(Type::Poly(PolyType::FnType {
                    input: Box::new(Type::default()),
                    output: Box::new(self.return_type.clone().unwrap_or_default()),
                }))
            }
            None => Type::Poly(PolyType::FnType {
                input: Box::new(Type::default()),
                output: Box::new(self.return_type.clone().unwrap_or_default()),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructDeclaration<'a> {
    pub identifier: &'a str,
    pub generics: Vec<(&'a str, Vec<&'a str>)>,
    pub fields: HashMap<&'a str, Type<'a>>,
}

impl<'a> StructDeclaration<'a> {
    pub fn new(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        fields: HashMap<&'a str, Type<'a>>,
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

    pub fn get_field(&self, field: &str) -> Option<&Type<'a>> {
        self.fields.get(field)
    }
}

impl<'a> AsB2Type<'a> for StructDeclaration<'a> {
    fn as_b2_type(&self) -> Type<'a> {
        Type::Mono(MonoType::TypeVar(self.identifier))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAlias<'a> {
    pub identifier: &'a str,
    pub generics: Vec<(&'a str, Vec<&'a str>)>,
    pub b2_type: Type<'a>,
}

impl<'a> TypeAlias<'a> {
    pub fn new(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        b2_type: Type<'a>,
    ) -> Self {
        Self {
            identifier,
            generics,
            b2_type,
        }
    }
}

impl<'a> AsB2Type<'a> for TypeAlias<'a> {
    fn as_b2_type(&self) -> Type<'a> {
        self.b2_type.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumDeclaration<'a> {
    pub identifier: &'a str,
    pub generics: Vec<(&'a str, Vec<&'a str>)>,
    pub enumerations: Vec<(&'a str, Vec<Type<'a>>)>,
}

impl<'a> EnumDeclaration<'a> {
    pub fn new(
        identifier: &'a str,
        generics: Vec<(&'a str, Vec<&'a str>)>,
        enumerations: Vec<(&'a str, Vec<Type<'a>>)>,
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
    pub identifier: &'a str,
    pub restrictions: Vec<&'a str>,
    pub decls: Vec<FunDeclComps<'a>>,
    pub impls: Vec<FunImplComps<'a>>,
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
