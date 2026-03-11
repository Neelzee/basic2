use crate::{
    common::{binop::BinOp, postfix::Postfix, uniop::UniOp},
    lexer::lex_expr::LexExpr,
};
use nom_language::precedence::Operation;

pub mod binop;
pub mod postfix;
pub mod primitive;
pub mod uniop;

pub type B2Op<'a> = Operation<UniOp, Postfix<'a>, BinOp, LexExpr<'a>>;
