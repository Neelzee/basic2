use crate::interpreter::gst::GlobalSymbolTable;
use parser::{lex_expr::LexExpr, lex_stmt::when_match::WhenMatch, lex_type::LexType};

#[derive(Debug)]
pub enum IE<'a> {
    WhenErrorExpectedTypeGotList {
        r#match: WhenMatch<'a>,
        list: Vec<LexExpr<'a>>,
        r#type: LexType<'a>,
    },
    WhenErrorExpectedStructGotList {
        r#match: WhenMatch<'a>,
        list: Vec<LexExpr<'a>>,
    },
    ExpectedBoolGot(LexExpr<'a>),
    UnpackingNonUnpackableType(LexType<'a>),
    IndexingErrorOutOfBounds {
        index: isize,
        bounds: usize,
    },
    IndexingErrorInvalidIndexee {
        indexee: LexExpr<'a>,
    },
    IndexingErrorInvalidType {
        index: LexExpr<'a>,
        indexee: LexExpr<'a>,
    },
    UnpackingCountMissmatch {
        ident_count: usize,
        expr_count: usize,
    },
    FnErrMissingArgument(&'a str, usize, &'a str),
    UndefinedVariable(&'a str),
    FnErrMissingImplementation(&'a str),
    FnErrMissingDeclaration(&'a str),
    FnErrArgumentMissMatch(&'a str, usize, usize),
    StructAccessingOnNonStruct(&'a str, &'a str, LexType<'a>),
    StructAccessingOnNonExistingField {
        ident: &'a str,
        field: &'a str,
    },
    StructAccessingOnNonExistingStruct {
        ident: &'a str,
        field: &'a str,
    },
    TraitErrMissingDeclaration(&'a str),
    MoreArgumentsSuppliedToFunctionThanExpected(
        &'a str,
        Vec<LexExpr<'a>>,
        Vec<(&'a str, Option<LexExpr<'a>>)>,
    ),
    // NOTE: Needed?
    MissingVariableDeclaration(&'a str),
}

pub enum IR<'a, T> {
    Ok { gst: GlobalSymbolTable<'a>, val: T },
    Err(IE<'a>),
}

impl<'a, T> IR<'a, T> {
    pub fn map<F, K>(self, f: F) -> IR<'a, K>
    where
        F: Fn(T) -> K,
    {
        match self {
            Self::Ok { gst, val } => IR::Ok { val: f(val), gst },
            Self::Err(err) => IR::Err(err),
        }
    }

    pub fn scope<F, K>(self, f: F) -> IR<'a, K>
    where
        F: FnOnce((GlobalSymbolTable<'a>, T)) -> IR<'a, K>,
    {
        match self {
            IR::Ok { gst, val } => f((gst.scope(), val)).reduce_scope(),
            IR::Err(ie) => IR::Err(ie),
        }
    }

    pub fn and_then<F, K>(self, f: F) -> IR<'a, K>
    where
        F: FnOnce((GlobalSymbolTable<'a>, T)) -> IR<'a, K>,
    {
        match self {
            Self::Ok { gst, val } => f((gst, val)),
            Self::Err(err) => IR::Err(err),
        }
    }

    fn reduce_scope(self) -> Self {
        match self {
            Self::Ok { gst, val } => Self::Ok {
                gst: gst.drop_scope(),
                val,
            },
            err @ Self::Err(_) => err,
        }
    }

    pub fn unwrap_or(self, fallback: T) -> T {
        match self {
            IR::Ok { val, .. } => val,
            IR::Err(_) => fallback,
        }
    }
}

impl<'a, T> From<Result<IR<'a, T>, IE<'a>>> for IR<'a, T> {
    fn from(value: Result<IR<'a, T>, IE<'a>>) -> Self {
        match value {
            Ok(val) => val,
            Err(err) => Self::Err(err),
        }
    }
}
