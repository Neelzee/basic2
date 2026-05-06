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
            B2Expr::IntLit(_) => B2Type::Int,
            B2Expr::FloatLit(_) => B2Type::Float,
            B2Expr::StrLit(_) => B2Type::Str,
            B2Expr::BoolLit(_) => B2Type::Bool,
            B2Expr::BinOp(l, op, r) => match op {
                BinOp::Add => {
                    if l.get_type(st) == B2Type::Str || r.get_type(st) == B2Type::Str {
                        return B2Type::Str;
                    }
                    if l.get_type(st) == B2Type::Float || r.get_type(st) == B2Type::Float {
                        return B2Type::Float;
                    }
                    return B2Type::Int;
                }
                BinOp::Mul => todo!(),
                BinOp::Sub => todo!(),
                BinOp::Div => todo!(),
                BinOp::Pow => todo!(),
                BinOp::Eq
                | BinOp::Geq
                | BinOp::Gt
                | BinOp::Leq
                | BinOp::Lt
                | BinOp::Neq
                | BinOp::And
                | BinOp::Or => B2Type::Bool,
                BinOp::Mod => B2Type::Int,
            },
            B2Expr::UniOp(_, b2_expr) => b2_expr.get_type(st),
            B2Expr::PostFix(b2_expr, postfix) => match postfix {
                Postfix::Incr | Postfix::Decr => b2_expr.get_type(st),
                Postfix::Index(_) => match &**b2_expr {
                    B2Expr::List { b2_type, .. } => B2Type::List(Box::new(b2_type.clone())),
                    t => t.get_type(st),
                },
            },
            B2Expr::EnumLit { ident, instance } => B2Type::EnumVariant(ident, instance),
            B2Expr::Variable { b2_type, .. } => b2_type.clone(),
            B2Expr::List { b2_type, .. } => B2Type::List(Box::new(b2_type.clone())),
            B2Expr::Tuple(f, s) => {
                B2Type::Tuple(Box::new(f.get_type(st)), Box::new(s.get_type(st)))
            }
            B2Expr::Struct { ident } => B2Type::Struct(ident),
        }
    }
}
