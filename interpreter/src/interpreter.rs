use crate::interpreter::{
    gst::GlobalSymbolTable,
    gstate::{IE, IR},
};
use parser::{
    common::{
        AsB2Type, B2Op, B2OpInner, binop::BinOp, postfix::Postfix, primitive::Primitive,
        uniop::UniOp,
    },
    lexer::{
        lex_expr::LexExpr,
        lex_stmt::{
            Import, LexStmt,
            decls::{
                Decl,
                variants::{VariableDeclaration, VariableDeclarationAssignment},
            },
            impls::Impl,
        },
        lex_type::{LexMonoType, LexPolyType, LexType},
    },
};
use std::collections::HashMap;

pub enum InterpreterError<'a> {
    Foo(&'a str),
}

mod gst;
mod gstate;

pub fn interpret<'a>(
    scope: usize,
    gst: GlobalSymbolTable<'a>,
    stmt: LexStmt<'a>,
) -> IR<'a, LexExpr<'a>> {
    match stmt {
        LexStmt::Decl(dcl) => interpret_decl(scope, gst, dcl).map(|_| LexExpr::Nil),
        LexStmt::Impl(ipl) => interpret_impl(scope, gst, ipl).map(|_| LexExpr::Nil),
        LexStmt::Import(import) => import_module(scope, gst, import).map(|_| LexExpr::Nil),
        LexStmt::VariableReassignment {
            identifier,
            reassignment,
            new_value,
        } => todo!(),
        LexStmt::ListReassignment {
            indexee,
            index,
            reassignment,
            new_value,
        } => todo!(),
        LexStmt::If { condition, body } => todo!(),
        LexStmt::While { condition, body } => todo!(),
        LexStmt::Block { body } => todo!(),
        LexStmt::FunctionInvocation {
            identifier,
            arguments,
        } => todo!(),
        LexStmt::Break => todo!(),
        LexStmt::Continue => todo!(),
        LexStmt::Return { value } => todo!(),
        LexStmt::For {
            start_stmt,
            condition,
            incrementer,
            body,
        } => todo!(),
        LexStmt::StructFieldReassignment {
            identifier,
            field,
            reassignment,
            new_value,
        } => todo!(),
        LexStmt::WhenStatement {
            identifier,
            branches,
        } => todo!(),
    }
}

pub fn interpret_decl<'a>(scope: usize, gst: GlobalSymbolTable<'a>, dcl: Decl<'a>) -> IR<'a, ()> {
    match dcl {
        Decl::VarDecl(var) => IR::ok(scope, gst.insert_var(scope, var.identifier, var), ()),
        Decl::TupUnpk(tup) => todo!(),
        Decl::StrUnpk(sct) => todo!(),
        Decl::LstUnpk(lst) => todo!(),
        Decl::VarDeclAss(vda) => todo!(),
        Decl::FnDecl(fdl) => todo!(),
        Decl::StructDecl(sdl) => todo!(),
        Decl::TypeDecl(tal) => todo!(),
        Decl::TraitDecl(tdl) => todo!(),
        Decl::EnumDecl(edl) => todo!(),
    }
}

pub fn interpret_impl<'a>(scope: usize, gst: GlobalSymbolTable<'a>, ipl: Impl<'a>) -> IR<'a, ()> {
    match ipl {
        Impl::FnImpl(function_implementation) => todo!(),
        Impl::TraitImpl(trait_impl) => todo!(),
    }
}

pub fn import_module<'a>(
    scope: usize,
    gst: GlobalSymbolTable<'a>,
    import: Import<'a>,
) -> IR<'a, ()> {
    todo!("Implement imports of modules in the interpreter")
}

pub fn unpack_expr<'a>(
    scope: usize,
    gst: &'a GlobalSymbolTable<'a>,
    expr: LexExpr<'a>,
) -> Result<Vec<LexExpr<'a>>, IE<'a>> {
    match expr {
        LexExpr::Nil => Err(IE::UnpackingNonUnpackableType(LexType::nil())),
        LexExpr::Literal(p) => match p {
            Primitive::Int(_) => Err(IE::UnpackingNonUnpackableType(LexType::int())),
            Primitive::Float(_) => Err(IE::UnpackingNonUnpackableType(LexType::float())),
            Primitive::Str(_) => Err(IE::UnpackingNonUnpackableType(LexType::str())),
            Primitive::Bool(_) => Err(IE::UnpackingNonUnpackableType(LexType::bool())),
        },
        LexExpr::Tuple(fst, snd) => {
            let mut fst_lst = unpack_expr(scope, gst, *fst.clone()).unwrap_or(vec![*fst.clone()]);
            let mut snd_lst = unpack_expr(scope, gst, *snd.clone()).unwrap_or(vec![*snd.clone()]);
            fst_lst.append(&mut snd_lst);
            Ok(fst_lst)
        }
        LexExpr::List(lst) => Ok(lst),
        LexExpr::Variable(var) => match gst.lookup_var(scope, var) {
            Some(val) => unpack_expr(scope, gst, val.value().clone()),
            None => Err(IE::UndefinedVariable(var)),
        },
        LexExpr::Group(inner) => unpack_expr(scope, gst, *inner),
        LexExpr::FunctionCall {
            identifier,
            arguments,
        } => unpack_expr(
            scope,
            gst,
            invoke_function(scope, &gst, identifier, arguments)?,
        ),
        LexExpr::Op(b2_op) => Ok(vec![eval_operation(scope, &gst, *b2_op)?]),
        LexExpr::Struct {
            field_implementations,
            ..
        } => Ok(field_implementations.into_iter().map(|(_, x)| x).collect()),
        LexExpr::StructFieldAccessing { identifier, field } => match gst
            .lookup_var(scope, identifier)
        {
            Some(var) => {
                let b2_type = var
                    .b2_type()
                    .cloned()
                    .unwrap_or_else(|| infer_type(scope, gst, var.value()));
                match b2_type {
                    LexType::Mono(LexMonoType::TypeVar(ident)) => {
                        match gst.lookup_struct_decl(scope, ident) {
                            Some(struct_decl) => match struct_decl.has_field(field) {
                                true => match var.value() {
                                    LexExpr::Struct {
                                        field_implementations,
                                        ..
                                    } => match field_implementations.get(field) {
                                        Some(expr) => unpack_expr(scope, gst, expr.clone()),
                                        None => unreachable!(
                                            "Struct was checked for the field: {field}'s existance in an earlier branch, but did not exist: {var:?}"
                                        ),
                                    },
                                    v => unreachable!(
                                        "Struct was either typed or inferred as a struct, it was: {v:?}"
                                    ),
                                },
                                false => Err(IE::StructAccessingOnNonExistingField(ident, field)),
                            },
                            None => Err(IE::StructAccessingOnNonStruct(identifier, field, b2_type)),
                        }
                    }
                    _ => Err(IE::StructAccessingOnNonStruct(identifier, field, b2_type)),
                }
            }
            None => Err(IE::UndefinedVariable(identifier)),
        },
        LexExpr::Enum { values, .. } => Ok(values),
    }
}

fn infer_type<'a>(
    scope: usize,
    gst: &'a GlobalSymbolTable<'a>,
    value: &'a LexExpr<'a>,
) -> LexType<'a> {
    match value {
        LexExpr::Nil => LexType::nil(),
        LexExpr::Literal(primitive) => match primitive {
            Primitive::Int(_) => LexType::int(),
            Primitive::Float(_) => LexType::float(),
            Primitive::Str(_) => LexType::str(),
            Primitive::Bool(_) => LexType::bool(),
        },
        LexExpr::Tuple(fst, snd) => {
            LexType::tuple(infer_type(scope, gst, fst), infer_type(scope, gst, snd))
        }
        LexExpr::List(xs) => LexType::list(
            xs.first()
                .map(|x| infer_type(scope, gst, x))
                .unwrap_or_default(),
        ),
        LexExpr::Variable(var) => match gst.lookup_var(scope, var) {
            Some(var) => var
                .b2_type()
                .cloned()
                .unwrap_or_else(|| infer_type(scope, gst, var.value())),
            None => todo!(),
        },
        LexExpr::Group(lex_expr) => infer_type(scope, gst, lex_expr),
        LexExpr::FunctionCall { arguments, .. } => {
            let xs = arguments
                .iter()
                .map(|x| infer_type(scope, gst, x))
                .collect::<Vec<_>>();
            match xs.as_slice() {
                [] => LexType::fun(LexType::nil(), LexType::nil()),
                [x] => LexType::fun(x.clone(), LexType::nil()),
                [x, y] => LexType::fun(x.clone(), y.clone()),
                xs => LexType::fold_funs(xs.to_vec())
                    .unwrap_or_else(|_| LexType::fun(LexType::nil(), LexType::nil())),
            }
        }
        LexExpr::Op(b2_op) => match b2_op.inner() {
            B2OpInner::Prefix(o, x) => match o {
                UniOp::Neg => infer_type(scope, gst, x),
            },
            B2OpInner::Postfix(x, o) => match o {
                Postfix::Incr | Postfix::Decr | Postfix::Index(_) => infer_type(scope, gst, x),
            },
            B2OpInner::Binary(l, o, r) => match o {
                BinOp::Mul | BinOp::Sub | BinOp::Add => {
                    match (infer_type(scope, gst, l), infer_type(scope, gst, r)) {
                        (LexType::Mono(LexMonoType::Str), LexType::Mono(_)) => LexType::str(),
                        (LexType::Mono(_), LexType::Mono(LexMonoType::Str)) => LexType::str(),
                        (LexType::Mono(LexMonoType::Float), LexType::Mono(_)) => LexType::float(),
                        (LexType::Mono(_), LexType::Mono(LexMonoType::Float)) => LexType::float(),
                        (l, _) => l,
                    }
                }
                BinOp::Mod => LexType::int(),
                BinOp::Div | BinOp::Pow => {
                    match (infer_type(scope, gst, l), infer_type(scope, gst, r)) {
                        (LexType::Mono(LexMonoType::Float), LexType::Mono(_)) => LexType::float(),
                        (LexType::Mono(_), LexType::Mono(LexMonoType::Float)) => LexType::float(),
                        (l, _) => l,
                    }
                }
                BinOp::Eq
                | BinOp::Geq
                | BinOp::Gt
                | BinOp::Leq
                | BinOp::Lt
                | BinOp::Neq
                | BinOp::And
                | BinOp::Or => LexType::bool(),
            },
        },
        LexExpr::Struct { identifier, .. } => LexType::var(identifier),
        LexExpr::StructFieldAccessing { identifier, field } => gst
            .lookup_struct_decl(scope, identifier)
            .and_then(|d| d.get_field(field))
            .cloned()
            .unwrap_or_default(),
        LexExpr::Enum {
            identifier,
            instance,
            ..
        } => LexType::r#enum(identifier, instance),
    }
}

fn invoke_function<'a>(
    scope: usize,
    gst: &'a GlobalSymbolTable<'a>,
    identifier: &'a str,
    arguments: Vec<LexExpr<'a>>,
) -> Result<LexExpr<'a>, IE<'a>> {
    let fun = gst
        .lookup_fn_impl(scope, identifier)
        .ok_or(IE::MissingFunctionImplementation(identifier))?;

    let parameters = &fun.parameters;

    if arguments.len() > parameters.len() {
        return Err(IE::MoreArgumentsSuppliedToFunctionThanExpected(
            identifier, arguments, parameters,
        ));
    }

    let mut function_scope = scope + 1;
    let mut function_gst = gst.clone();

    for (i, (arg, (par_id, opt_par))) in arguments
        .into_iter()
        .map(|x| Some(x))
        .chain((0..(parameters.len())).map(|_| None))
        .zip(parameters)
        .enumerate()
    {
        match (arg, opt_par.clone()) {
            (None, None) => {
                return Err(IE::MissingFunctionArgument(identifier, i, par_id));
            }
            (Some(arg), _) | (_, Some(arg)) => {
                function_gst.insert_var_ass(
                    function_scope,
                    par_id,
                    VariableDeclarationAssignment::new(par_id, None, arg),
                );
            }
        }
    }

    for stmt in fun.body.iter() {
        match stmt {
            LexStmt::Return { value } => {
                let val: LexExpr<'a> = eval_expr(function_scope, &function_gst, value.clone().unwrap_or_default())?;
                return Ok(val.clone());
            }
            _ => match interpret(function_scope, function_gst.clone(), stmt.clone()) {
                IR::Ok { scope, gst, .. } => {
                    function_scope = scope;
                    function_gst = gst;
                }
                IR::Err(ie) => {
                    return Err(ie);
                }
            },
        }
    }

    Ok(LexExpr::Nil)
}

fn eval_operation<'a>(
    scope: usize,
    gst: &'a GlobalSymbolTable<'a>,
    op: B2Op<'a>,
) -> Result<LexExpr<'a>, IE<'a>> {
    todo!()
}

fn eval_expr<'a, 'b: 'a>(
    scope: usize,
    gst: &'b GlobalSymbolTable<'a>,
    expr: LexExpr<'a>,
) -> Result<LexExpr<'a>, IE<'a>> {
    match expr {
        LexExpr::Variable(ident) => match gst.lookup_var(scope, ident) {
            Some(dcl) => eval_expr(scope, gst, dcl.value().clone()),
            None => Err(IE::UndefinedVariable(ident)),
        },
        LexExpr::Group(inner) => eval_expr(scope, gst, *inner),
        LexExpr::FunctionCall {
            identifier,
            arguments,
        } => invoke_function(scope, &gst, identifier, arguments),
        LexExpr::Op(op) => eval_operation(scope, &gst, *op),
        LexExpr::StructFieldAccessing { identifier, field } => {
            match gst.lookup_var(scope, identifier) {
                Some(dcl) => match dcl.value() {
                    LexExpr::Struct {
                        field_implementations,
                        ..
                    } => field_implementations
                        .get(field)
                        .cloned()
                        .ok_or_else(|| IE::StructAccessingOnNonExistingField(identifier, field)),
                    other => Err(IE::StructAccessingOnNonStruct(
                        identifier,
                        field,
                        infer_type(scope, &gst, other),
                    )),
                },
                None => Err(IE::StructAccessingOnNonExistingStruct(identifier, field)),
            }
        }
        _ => Ok(expr),
    }
}
