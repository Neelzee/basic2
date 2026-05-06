use parser::common::binop::BinOp;

use crate::{
    b2::{
        exp::B2Expr,
        typ::{B2Type, MonoType, PolyType},
    },
    type_checker::SymbolTable,
};

#[derive(Debug, PartialEq)]
pub enum TIType<'a> {
    Concrete(B2Type<'a>),
    TypeVar(&'a str),
    TypeRest(Box<Self>, Vec<&'a str>),
    TypeArr(Box<Self>, Box<Self>),
    TypePoly(Box<PolyType<Self>>),
}

pub struct Context<'a> {
    st: SymbolTable<'a>,
}

fn to_type<'a>(
    expr: &'a B2Expr<'a>,
    ctx: Context<'a>,
) -> Result<(TIType<'a>, Context<'a>), &'a str> {
    match expr {
        B2Expr::IntLit(_) => Ok((TIType::Concrete(B2Type::Mono(MonoType::Int)), ctx)),
        B2Expr::FloatLit(_) => Ok((TIType::Concrete(B2Type::Mono(MonoType::Float)), ctx)),
        B2Expr::StrLit(_) => Ok((TIType::Concrete(B2Type::Mono(MonoType::Str)), ctx)),
        B2Expr::BoolLit(_) => Ok((TIType::Concrete(B2Type::Mono(MonoType::Bool)), ctx)),
        B2Expr::EnumLit { ident, instance } => Ok((
            TIType::Concrete(B2Type::Mono(MonoType::EnumVariant(ident, instance))),
            ctx,
        )),
        B2Expr::Struct { ident, .. } => {
            Ok((TIType::Concrete(B2Type::Mono(MonoType::Struct(ident))), ctx))
        }
        B2Expr::UniOp(op, expr) => todo!(),
        B2Expr::PostFix(expr, op) => todo!(),
        B2Expr::BinOp(l, op, r) => match op {
            BinOp::Add => todo!(),
            BinOp::Mul => todo!(),
            BinOp::Sub => todo!(),
            BinOp::Div => todo!(),
            BinOp::Pow => todo!(),
            BinOp::Eq => todo!(),
            BinOp::Geq => todo!(),
            BinOp::Gt => todo!(),
            BinOp::Leq => todo!(),
            BinOp::Lt => todo!(),
            BinOp::Neq => todo!(),
            BinOp::Mod => todo!(),
            BinOp::And => todo!(),
            BinOp::Or => todo!(),
        },
        B2Expr::Variable { ident, b2_type } => todo!(),
        B2Expr::List(xs) => todo!(),
        B2Expr::Tuple(f, s) => todo!(),
        B2Expr::Fn { ident, args } => todo!(),
    }
}
