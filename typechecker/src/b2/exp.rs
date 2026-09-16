use crate::{b2::typ::B2Type, type_checker::SymbolTable};
use parser::common::{binop::BinOp, uniop::UniOp};

#[derive(Debug, Clone)]
pub enum Postfix<'a> {
    Incr,
    Decr,
    Index(Box<B2Expr<'a>>),
}

#[derive(Debug, Clone)]
pub enum B2Expr<'a> {
    IntLit(i32),
    FloatLit(i32),
    StrLit(&'a str),
    BoolLit(bool),
    EnumLit {
        ident: &'a str,
        instance: &'a str,
    },
    Struct {
        ident: &'a str,
        fields: Vec<StructField>,
    },
    UniOp(UniOp, Box<Self>),
    PostFix(Box<Self>, Postfix<'a>),
    BinOp(Box<Self>, BinOp, Box<Self>),
    Variable {
        ident: &'a str,
        b2_type: Option<B2Type<'a>>,
    },
    List(Vec<Self>),
    Tuple(Box<Self>, Box<Self>),
    Fn {
        ident: &'a str,
        args: Vec<Self>,
    },
}

type StructField = ();

impl<'a> B2Expr<'a> {
    pub fn get_type(&self, st: &'a SymbolTable<'a>) -> B2Type<'a> {
        match &self {
            B2Expr::IntLit(_) => todo!(),
            B2Expr::FloatLit(_) => todo!(),
            B2Expr::StrLit(_) => todo!(),
            B2Expr::BoolLit(_) => todo!(),
            B2Expr::EnumLit { ident, instance } => todo!(),
            B2Expr::Struct { ident, fields } => todo!(),
            B2Expr::UniOp(uni_op, b2_expr) => todo!(),
            B2Expr::PostFix(b2_expr, postfix) => todo!(),
            B2Expr::BinOp(b2_expr, bin_op, b2_expr1) => todo!(),
            B2Expr::Variable { ident, b2_type } => todo!(),
            B2Expr::List(b2_exprs) => todo!(),
            B2Expr::Tuple(b2_expr, b2_expr1) => todo!(),
            B2Expr::Fn { ident, args } => todo!(),
        }
    }
}
