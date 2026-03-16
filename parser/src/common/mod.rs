use crate::{
    common::{binop::BinOp, postfix::Postfix, uniop::UniOp},
    lexer::lex_expr::LexExpr,
};
use nom_language::precedence::Operation;

pub mod binop;
pub mod postfix;
pub mod primitive;
pub mod uniop;

type B2OpInner<'a> = Operation<UniOp, Postfix<'a>, BinOp, LexExpr<'a>>;

pub struct B2Op<'a>(B2OpInner<'a>);

impl<'a> B2Op<'a> {
    pub fn prefix(op: UniOp, expr: LexExpr<'a>) -> Self {
        Self(B2OpInner::Prefix(op, expr))
    }

    pub fn postfix(expr: LexExpr<'a>, op: Postfix<'a>) -> Self {
        Self(B2OpInner::Postfix(expr, op))
    }

    pub fn binary(l: LexExpr<'a>, op: BinOp, r: LexExpr<'a>) -> Self {
        Self(B2OpInner::Binary(l, op, r))
    }
}

impl<'a> Into<B2OpInner<'a>> for B2Op<'a> {
    fn into(self) -> B2OpInner<'a> {
        self.0
    }
}

impl<'a> From<B2OpInner<'a>> for B2Op<'a> {
    fn from(value: B2OpInner<'a>) -> Self {
        Self(value)
    }
}

impl<'a> PartialEq for B2Op<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (B2OpInner::Prefix(ll, lr), B2OpInner::Prefix(rl, rr)) => ll == rl && lr == rr,
            (B2OpInner::Postfix(ll, lr), B2OpInner::Postfix(rl, rr)) => ll == rl && lr == rr,
            (B2OpInner::Binary(ll, lm, lr), B2OpInner::Binary(rl, rm, rr)) => {
                ll == rl && lr == rr && lm == rm
            }
            _ => false,
        }
    }
}

impl<'a> std::fmt::Debug for B2Op<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            B2OpInner::Prefix(op, val) => {
                f.debug_tuple("B2Op::Prefix").field(op).field(val).finish()
            }
            B2OpInner::Postfix(val, op) => {
                f.debug_tuple("B2Op::Postfix").field(val).field(op).finish()
            }
            B2OpInner::Binary(l, op, r) => f
                .debug_tuple("B2Op::Binary")
                .field(l)
                .field(op)
                .field(r)
                .finish(),
        }
    }
}
