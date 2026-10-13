use crate::interpreter::{
    gst::GlobalSymbolTable,
    gstate::{IE, IR},
};
use common::types::{MonoType, Type};
use parser::{
    common::{B2Op, B2OpInner, binop::BinOp, postfix::Postfix, primitive::Primitive, uniop::UniOp},
    pexpr::PExpr,
    pstmt::{
        Import, PStmt,
        decls::{
            Decl::{self, FnDecl},
            variants::{
                EnumDeclaration, FunctionDeclaration, ListUnpacking, StructDeclaration,
                StructUnpacking, TraitDecl, TupleUnpacking, TypeAlias, VariableDeclaration,
                VariableDeclarationAssignment,
            },
        },
        impls::{FunctionImplementation, Impl, TraitImpl},
        when_match::WhenMatch,
    },
    ptypes::TypeParser,
};
use std::collections::HashMap;

pub enum InterpreterError<'a> {
    Foo(&'a str),
}

pub mod gst;
pub mod gstate;

pub fn index_expr<'a>(
    gst: &GlobalSymbolTable<'a>,
    i: i32,
    indexee: PExpr<'a>,
    new_value: PExpr<'a>,
) -> Result<PExpr<'a>, IE<'a>> {
    match indexee {
        PExpr::Nil
        | PExpr::Literal(_)
        | PExpr::Op(_)
        | PExpr::Struct { .. }
        | PExpr::Enum { .. } => Err(IE::IndexingErrorInvalidIndexee { indexee }),

        /* NOTE: This changes the tree, technically, by removing the group, does
           it matter? I dont think so.
        */
        PExpr::Group(inner) => index_expr(gst, i, *inner, new_value),

        /* NOTE: Maybe stupid to allow this:
        LET (foo, bar, qaz) >< (1, (2, 3));

        But disallow this:

        LET qaz = (1, (2, 3))[2];

        This would be the "correct way"
        LET qaz = (1, (2, 3))[1][0];

        but idk.
        */
        PExpr::Tuple(l, r) => match i {
            0 => Ok(PExpr::Tuple(Box::new(new_value), r)),
            1 => Ok(PExpr::Tuple(l, Box::new(new_value))),
            _ => Err(IE::IndexingErrorOutOfBounds {
                bounds: 2,
                index: i as isize,
            }),
        },

        PExpr::List(mut xs) => {
            let bounds = xs.len();
            if i < 0 || i as usize <= bounds {
                return Err(IE::IndexingErrorOutOfBounds {
                    index: i as isize,
                    bounds,
                });
            }
            xs[i as usize] = new_value;

            Ok(PExpr::List(xs))
        }

        PExpr::Variable(ident) => match gst.lookup_var(ident).cloned() {
            Some(VariableDeclarationAssignment { value, .. }) => {
                index_expr(gst, i, value, new_value)
            }
            None => Err(IE::MissingVariableDeclaration(ident)),
        },

        PExpr::FunctionCall {
            ident: identifier,
            arguments,
        } => invoke_function(gst, identifier, arguments)
            .and_then(|idxe| index_expr(gst, i, idxe, new_value)),

        PExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => {
            todo!("I think this is not implemented well enough")
        }
    }
}

pub fn interpret<'a>(gst: GlobalSymbolTable<'a>, stmt: PStmt<'a>) -> IR<'a, PExpr<'a>> {
    match stmt {
        PStmt::Decl(dcl) => interpret_decl(gst, dcl).map(|_| PExpr::Nil),
        PStmt::Impl(ipl) => interpret_impl(gst, ipl).map(|_| PExpr::Nil),
        PStmt::Import(import) => import_module(gst, import).map(|_| PExpr::Nil),
        PStmt::VariableReassignment {
            ident: identifier,
            reassignment,
            new_value,
        } => IR::Ok {
            gst: gst.insert_var_ass(
                identifier,
                VariableDeclarationAssignment {
                    identifier,
                    variable_type: None,
                    value: new_value,
                },
            ),
            val: PExpr::Nil,
        },
        PStmt::ListReassignment {
            indexee,
            index,
            reassignment,
            new_value,
        } => match &index {
            PExpr::Literal(Primitive::Int(i)) => match index_expr(&gst, *i, indexee, new_value) {
                Ok(val) => IR::Ok { gst, val },
                Err(err) => IR::Err(err),
            },
            PExpr::Variable(_) => todo!(),
            PExpr::Group(lex_expr) => todo!(),
            PExpr::FunctionCall {
                ident: identifier,
                arguments,
            } => todo!(),
            PExpr::StructFieldAccessing {
                ident: identifier,
                field,
            } => todo!(),

            PExpr::Nil
            | PExpr::Tuple(_, _)
            | PExpr::List(_)
            | PExpr::Op(_)
            | PExpr::Struct { .. }
            | PExpr::Literal(_)
            | PExpr::Enum { .. } => IR::Err(IE::IndexingErrorInvalidType { indexee, index }),
        },
        PStmt::If { condition, body } => match eval_expr(&gst, condition) {
            Ok(PExpr::Literal(Primitive::Bool(b))) if b => IR::Ok {
                gst,
                val: PExpr::Nil,
            }
            .scope(|(gst, _)| {
                body.into_iter().fold(
                    IR::Ok {
                        gst,
                        val: PExpr::Nil,
                    },
                    |acc, stmt| acc.and_then(|(gst_prime, _)| interpret(gst_prime, stmt)),
                )
            }),
            Ok(PExpr::Literal(Primitive::Bool(_))) => IR::Ok {
                gst,
                val: PExpr::Nil,
            },
            Ok(v) => IR::Err(IE::ExpectedBoolGot(v)),
            Err(err) => IR::Err(err),
        },
        PStmt::While { condition, body } => IR::Ok {
            gst,
            val: PExpr::Nil,
        }
        .scope(|(gst, _)| interpret_while(gst, condition, body)),
        PStmt::Block { body } => IR::Ok {
            gst,
            val: PExpr::Nil,
        }
        .scope(|(gst, val)| {
            body.into_iter().fold(IR::Ok { gst, val }, |acc, cur| {
                acc.and_then(|(gst, _)| interpret(gst, cur))
            })
        }),
        PStmt::FunctionInvocation { ident, arguments } => invoke_function(&gst, ident, arguments)
            .map(|val| IR::Ok { gst, val })
            .into(),
        PStmt::Break => todo!(),
        PStmt::Continue => todo!(),
        PStmt::Return { value } => IR::Ok {
            gst,
            val: value.unwrap_or_default(),
        },
        PStmt::For {
            start_stmt,
            condition,
            incrementer,
            body,
        } => IR::Ok {
            gst,
            val: PExpr::Nil,
        }
        .scope(|(gst, _)| {
            let ident = start_stmt.identifier;
            let gst = gst.insert_var_ass(ident, start_stmt);
            interpret_for(gst, ident, incrementer, condition, body)
        }),
        PStmt::StructFieldReassignment {
            ident,
            field,
            reassignment,
            new_value,
        } => match gst.lookup_var(ident) {
            Some(VariableDeclarationAssignment {
                value:
                    PExpr::Struct {
                        ident: struct_ident,
                        field_implementations,
                        ..
                    },
                ..
            }) => match gst.lookup_struct_decl(struct_ident) {
                Some(decl) => {
                    if decl.has_field(field) {
                        let mut field_implementations = field_implementations.clone();
                        field_implementations.insert(field, new_value);
                        IR::Ok {
                            gst: gst.clone().insert_var_ass(
                                ident,
                                VariableDeclarationAssignment {
                                    identifier: ident,
                                    variable_type: Some(Type::r#struct(struct_ident)),
                                    value: PExpr::Struct {
                                        ident: *struct_ident,
                                        field_implementations,
                                    },
                                },
                            ),
                            val: PExpr::Nil,
                        }
                    } else {
                        IR::Err(IE::StructAccessingOnNonExistingField { ident, field })
                    }
                }
                None => IR::Err(IE::StructAccessingOnNonExistingStruct { ident, field }),
            },
            Some(_) => todo!("Catch on non-struct?"),
            None => IR::Err(IE::MissingVariableDeclaration(ident)),
        },
        PStmt::WhenStatement { ident, branches } => match gst.lookup_var(ident) {
            Some(VariableDeclarationAssignment {
                value: PExpr::List(xs),
                ..
            }) => {
                for (mtch, body) in branches {
                    match (mtch, xs.as_slice()) {
                        (WhenMatch::EmptyList { condition }, []) => {
                            match condition.map(|expr| eval_expr(&gst, expr)) {
                                Some(Err(err)) => {
                                    return IR::Err(err);
                                }
                                Some(Ok(PExpr::Literal(Primitive::Bool(b)))) if b => {
                                    return IR::Ok {
                                        gst,
                                        val: PExpr::Nil,
                                    }
                                    .scope(|(gst, _)| {
                                        body.into_iter().fold(
                                            IR::Ok {
                                                gst,
                                                val: PExpr::Nil,
                                            },
                                            move |acc, stmt| {
                                                acc.and_then(|(gst, _)| interpret(gst, stmt))
                                            },
                                        )
                                    });
                                }
                                None => {
                                    return IR::Ok {
                                        gst,
                                        val: PExpr::Nil,
                                    }
                                    .scope(|(gst, _)| {
                                        body.into_iter().fold(
                                            IR::Ok {
                                                gst,
                                                val: PExpr::Nil,
                                            },
                                            move |acc, stmt| {
                                                acc.and_then(|(gst, _)| interpret(gst, stmt))
                                            },
                                        )
                                    });
                                }
                                Some(Ok(PExpr::Literal(Primitive::Bool(_)))) => continue,
                                Some(_) => todo!("Invalid type"),
                            }
                        }
                        (WhenMatch::Singleton { ident, condition }, [x]) => {
                            let gst = gst.clone().scope().insert_var_ass(
                                ident,
                                VariableDeclarationAssignment {
                                    identifier: ident,
                                    variable_type: None,
                                    value: x.clone(),
                                },
                            );
                            match condition.map(|expr| eval_expr(&gst, expr)) {
                                Some(Err(err)) => {
                                    return IR::Err(err);
                                }
                                Some(Ok(PExpr::Literal(Primitive::Bool(b)))) if b => {
                                    return IR::Ok {
                                        gst,
                                        val: PExpr::Nil,
                                    }
                                    .scope(|(gst, _)| {
                                        body.into_iter().fold(
                                            IR::Ok {
                                                gst,
                                                val: PExpr::Nil,
                                            },
                                            move |acc, stmt| {
                                                acc.and_then(|(gst, _)| interpret(gst, stmt))
                                            },
                                        )
                                    })
                                    .and_then(|(gst, val)| IR::Ok {
                                        gst: gst.drop_scope(),
                                        val,
                                    });
                                }
                                None => {
                                    return IR::Ok {
                                        gst,
                                        val: PExpr::Nil,
                                    }
                                    .scope(|(gst, _)| {
                                        body.into_iter().fold(
                                            IR::Ok {
                                                gst,
                                                val: PExpr::Nil,
                                            },
                                            move |acc, stmt| {
                                                acc.and_then(|(gst, _)| interpret(gst, stmt))
                                            },
                                        )
                                    })
                                    .and_then(|(gst, val)| IR::Ok {
                                        gst: gst.drop_scope(),
                                        val,
                                    });
                                }
                                Some(Ok(PExpr::Literal(Primitive::Bool(_)))) => continue,
                                Some(_) => todo!("Invalid type"),
                            }
                        }
                        (
                            WhenMatch::VariadicList {
                                idents,
                                remainder,
                                condition,
                            },
                            _,
                        ) => {
                            let ident_count = idents.len();
                            let expr_count = xs.len();

                            if ident_count > expr_count {
                                return IR::Err(IE::UnpackingCountMissmatch {
                                    ident_count,
                                    expr_count,
                                });
                            }

                            let mut ys = xs.clone();
                            ys.reverse();
                            let mut gst = gst.clone().scope();

                            loop {
                                match idents.first() {
                                    Some(ident) => {
                                        gst = gst.insert_var_ass(
                                            ident,
                                            VariableDeclarationAssignment {
                                                identifier: ident,
                                                variable_type: None,
                                                value: ys.pop().unwrap().clone(),
                                            },
                                        );
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            }

                            if let Some(ident) = remainder {
                                ys.reverse();
                                gst = gst.insert_var_ass(
                                    ident,
                                    VariableDeclarationAssignment {
                                        identifier: ident,
                                        variable_type: None,
                                        value: PExpr::List(ys),
                                    },
                                );
                            }

                            match condition
                                .map(|expr| eval_expr(&gst, expr))
                                .unwrap_or_else(|| Ok(PExpr::bool(true)))
                            {
                                Ok(PExpr::Literal(Primitive::Bool(b))) => {
                                    if b {
                                        return body
                                            .into_iter()
                                            .fold(
                                                IR::Ok {
                                                    gst,
                                                    val: PExpr::Nil,
                                                },
                                                move |acc, stmt| {
                                                    acc.and_then(|(gst, _)| interpret(gst, stmt))
                                                },
                                            )
                                            .and_then(|(gst, val)| IR::Ok {
                                                gst: gst.drop_scope(),
                                                val,
                                            });
                                    } else {
                                        continue;
                                    }
                                }
                                Ok(_) => todo!(),
                                Err(err) => {
                                    return IR::Err(err);
                                }
                            }
                        }
                        (WhenMatch::CatchAll { ident, condition }, _) => {
                            let gst = gst.clone().scope().insert_var_ass(
                                ident,
                                VariableDeclarationAssignment {
                                    identifier: ident,
                                    variable_type: None,
                                    value: PExpr::List(xs.clone()),
                                },
                            );
                            match condition
                                .map(|expr| eval_expr(&gst, expr))
                                .unwrap_or_else(|| Ok(PExpr::bool(true)))
                            {
                                Ok(PExpr::Literal(Primitive::Bool(b))) => {
                                    if b {
                                        return body
                                            .into_iter()
                                            .fold(
                                                IR::Ok {
                                                    gst,
                                                    val: PExpr::Nil,
                                                },
                                                move |acc, stmt| {
                                                    acc.and_then(|(gst, _)| interpret(gst, stmt))
                                                },
                                            )
                                            .and_then(|(gst, val)| IR::Ok {
                                                gst: gst.drop_scope(),
                                                val,
                                            });
                                    } else {
                                        continue;
                                    }
                                }
                                Ok(_) => todo!(),
                                Err(err) => {
                                    return IR::Err(err);
                                }
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }

                IR::Ok {
                    gst,
                    val: PExpr::Nil,
                }
            }
            Some(_) => todo!(),
            None => todo!(),
        },
    }
}

fn interpret_for<'a>(
    gst: GlobalSymbolTable<'a>,
    ident: &'a str,
    incrementer: VariableDeclarationAssignment<'a>,
    condition: PExpr<'a>,
    body: Vec<PStmt<'a>>,
) -> IR<'a, PExpr<'a>> {
    match eval_expr(&gst, condition) {
        Ok(PExpr::Literal(Primitive::Bool(b))) if b => body
            .clone()
            .into_iter()
            .fold(
                IR::Ok {
                    gst,
                    val: PExpr::Nil,
                },
                |acc, cur| acc.and_then(|(gst, _)| interpret(gst, cur)),
            )
            .and_then(|(gst, _)| IR::Ok {
                gst: gst.insert_var_ass(ident, incrementer),
                val: PExpr::Nil,
            }),
        Ok(PExpr::Literal(Primitive::Bool(_))) => IR::Ok {
            gst,
            val: PExpr::Nil,
        },
        Ok(_) => unreachable!("non-bool should not appear as condition in for-stmt"),
        Err(err) => IR::Err(err),
    }
}

fn interpret_while<'a>(
    gst: GlobalSymbolTable<'a>,
    condition: PExpr<'a>,
    body: Vec<PStmt<'a>>,
) -> IR<'a, PExpr<'a>> {
    match eval_expr(&gst, condition.clone()) {
        Ok(PExpr::Literal(Primitive::Bool(b))) if b => body
            .clone()
            .into_iter()
            .fold(
                IR::Ok {
                    gst,
                    val: PExpr::Nil,
                },
                |acc, stmt| acc.and_then(|(gst_prime, _)| interpret(gst_prime, stmt)),
            )
            .and_then(|(gst, _)| interpret_while(gst, condition, body)),
        Ok(PExpr::Literal(Primitive::Bool(_))) => IR::Ok {
            gst,
            val: PExpr::Nil,
        },
        Ok(v) => IR::Err(IE::ExpectedBoolGot(v)),
        Err(err) => IR::Err(err),
    }
}

pub fn interpret_decl<'a>(gst: GlobalSymbolTable<'a>, dcl: Decl<'a>) -> IR<'a, ()> {
    match dcl {
        Decl::VarDecl(var) => IR::Ok {
            gst: gst.insert_var(var.identifier, var),
            val: (),
        },

        Decl::LstUnpk(ListUnpacking {
            identifiers,
            remainder,
            value,
        }) => match unpack_expr(&gst, value) {
            Ok(exprs) => {
                let ident_count = identifiers.len();
                let expr_count = exprs.len();
                if ident_count < expr_count {
                    return IR::Err(IE::UnpackingCountMissmatch {
                        ident_count,
                        expr_count,
                    });
                }
                match remainder {
                    Some(ident) => IR::Ok {
                        gst: identifiers
                            .into_iter()
                            .zip(exprs.clone().into_iter().take(ident_count).into_iter())
                            .fold(gst, |gst_prime, (identifier, value)| {
                                gst_prime.insert_var_ass(
                                    identifier,
                                    VariableDeclarationAssignment {
                                        identifier,
                                        variable_type: None,
                                        value,
                                    },
                                )
                            })
                            .insert_var_ass(
                                ident,
                                VariableDeclarationAssignment {
                                    identifier: ident,
                                    variable_type: None,
                                    value: PExpr::List(
                                        exprs.into_iter().skip(ident_count).collect(),
                                    ),
                                },
                            ),
                        val: (),
                    },
                    None => IR::Ok {
                        gst: identifiers.into_iter().zip(exprs.into_iter()).fold(
                            gst,
                            |gst_prime, (i, v)| {
                                gst_prime.insert_var_ass(
                                    i,
                                    VariableDeclarationAssignment {
                                        identifier: i,
                                        variable_type: None,
                                        value: v,
                                    },
                                )
                            },
                        ),
                        val: (),
                    },
                }
            }
            Err(err) => IR::Err(err),
        },
        Decl::TupUnpk(TupleUnpacking { identifiers, value }) => match unpack_expr(&gst, value) {
            Ok(exprs) => {
                let ident_count = identifiers.len();
                let expr_count = exprs.len();
                if ident_count < expr_count {
                    IR::Err(IE::UnpackingCountMissmatch {
                        ident_count,
                        expr_count,
                    })
                } else {
                    IR::Ok {
                        gst: identifiers.into_iter().zip(exprs.into_iter()).fold(
                            gst,
                            |gst_prime, (i, v)| {
                                gst_prime.insert_var_ass(
                                    i,
                                    VariableDeclarationAssignment {
                                        identifier: i,
                                        variable_type: None,
                                        value: v,
                                    },
                                )
                            },
                        ),
                        val: (),
                    }
                }
            }
            Err(err) => IR::Err(err),
        },
        Decl::StrUnpk(StructUnpacking { identifiers, value }) => match value {
            PExpr::Struct {
                ident: identifier,
                field_implementations,
            } => {
                let mut gst = gst;
                for (var_id, field_key) in identifiers {
                    match field_implementations.get(field_key) {
                        Some(field_val) => {
                            let ident = var_id.unwrap_or(field_key);
                            gst = gst.insert_var_ass(
                                ident,
                                VariableDeclarationAssignment {
                                    identifier: ident,
                                    variable_type: None,
                                    value: field_val.clone(),
                                },
                            )
                        }
                        None => {
                            return IR::Err(IE::StructAccessingOnNonExistingField {
                                ident: identifier,
                                field: field_key,
                            });
                        }
                    }
                }
                IR::Ok { gst, val: () }
            }
            _ => todo!("How to unpack this?"),
        },
        Decl::VarDeclAss(vda @ VariableDeclarationAssignment { identifier, .. }) => IR::Ok {
            gst: gst.insert_var_ass(identifier, vda),
            val: (),
        },
        Decl::FnDecl(fdd @ FunctionDeclaration { identifier, .. }) => IR::Ok {
            gst: gst.insert_fn_decl(identifier, fdd),
            val: (),
        },
        Decl::StructDecl(sdl @ StructDeclaration { identifier, .. }) => IR::Ok {
            gst: gst.insert_struct_decl(identifier, sdl),
            val: (),
        },
        Decl::TypeDecl(tal @ TypeAlias { identifier, .. }) => IR::Ok {
            gst: gst.insert_type_decl(identifier, tal),
            val: (),
        },
        Decl::TraitDecl(tdl @ TraitDecl { identifier, .. }) => IR::Ok {
            gst: gst.insert_trait_decl(identifier, tdl),
            val: (),
        },
        Decl::EnumDecl(edl @ EnumDeclaration { identifier, .. }) => IR::Ok {
            gst: gst.insert_enum_decl(identifier, edl),
            val: (),
        },
    }
}

pub fn interpret_impl<'a>(gst: GlobalSymbolTable<'a>, ipl: Impl<'a>) -> IR<'a, ()> {
    match ipl {
        Impl::FnImpl(FunctionImplementation {
            identifier,
            parameters,
            body,
        }) => match gst.lookup_fn_decl(identifier) {
            Some(decl) => {
                let args = decl.parameters.len();
                let pars = parameters.len();
                if pars != args {
                    return IR::Err(IE::FnErrArgumentMissMatch(identifier, pars, args));
                }
                body.into_iter()
                    .fold(
                        IR::Ok {
                            gst: gst.scope(),
                            val: (),
                        },
                        move |gst_p, stmt| {
                            gst_p.and_then(|(gst_q, _)| interpret(gst_q, stmt).map(|_| ()))
                        },
                    )
                    .and_then(|(gst, val)| IR::Ok {
                        gst: gst.drop_scope(),
                        val,
                    })
            }
            None => IR::Err(IE::FnErrMissingDeclaration(identifier)),
        },
        Impl::TraitImpl(
            til @ TraitImpl {
                trait_identifier, ..
            },
        ) => match gst.lookup_trait_decl(trait_identifier) {
            Some(_) => IR::Ok {
                gst: gst.insert_trait_impl(trait_identifier, til),
                val: (),
            },
            None => IR::Err(IE::TraitErrMissingDeclaration(trait_identifier)),
        },
    }
}

pub fn import_module<'a>(gst: GlobalSymbolTable<'a>, import: Import<'a>) -> IR<'a, ()> {
    todo!("Implement imports of modules in the interpreter")
}

pub fn unpack_expr<'a>(
    gst: &GlobalSymbolTable<'a>,
    expr: PExpr<'a>,
) -> Result<Vec<PExpr<'a>>, IE<'a>> {
    match expr {
        PExpr::Nil => Err(IE::UnpackingNonUnpackableType(Type::nil())),
        PExpr::Literal(p) => match p {
            Primitive::Int(_) => Err(IE::UnpackingNonUnpackableType(Type::int())),
            Primitive::Float(_) => Err(IE::UnpackingNonUnpackableType(Type::float())),
            Primitive::Str(_) => Err(IE::UnpackingNonUnpackableType(Type::str())),
            Primitive::Bool(_) => Err(IE::UnpackingNonUnpackableType(Type::bool())),
        },
        PExpr::Tuple(fst, snd) => {
            let mut fst_lst = unpack_expr(gst, *fst.clone()).unwrap_or(vec![]);
            let mut snd_lst = unpack_expr(gst, *snd.clone()).unwrap_or(vec![*snd.clone()]);
            fst_lst.append(&mut snd_lst);
            Ok(fst_lst)
        }
        PExpr::List(lst) => Ok(lst),
        PExpr::Variable(var) => match gst.lookup_var(var) {
            Some(val) => unpack_expr(gst, val.value().clone()),
            None => Err(IE::UndefinedVariable(var)),
        },
        PExpr::Group(inner) => unpack_expr(gst, *inner),
        PExpr::FunctionCall {
            ident: identifier,
            arguments,
        } => unpack_expr(gst, invoke_function(&gst, identifier, arguments)?),
        PExpr::Op(b2_op) => Ok(vec![eval_operation(&gst, *b2_op)?]),
        PExpr::Struct {
            field_implementations: _xs,
            ..
        } => todo!("Cant unpack struct"), //Ok(_xs.into_iter().map(|(_, x)| x).collect()),
        PExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => match gst.lookup_var(identifier) {
            Some(var) => {
                let b2_type = var
                    .b2_type()
                    .cloned()
                    .unwrap_or_else(|| infer_type(gst, var.value()));
                match b2_type {
                    Type::Mono(MonoType::TypeVar(ident)) => match gst.lookup_struct_decl(ident) {
                        Some(struct_decl) => match struct_decl.has_field(field) {
                            true => match var.value() {
                                PExpr::Struct {
                                    field_implementations,
                                    ..
                                } => match field_implementations.get(field) {
                                    Some(expr) => unpack_expr(gst, expr.clone()),
                                    None => unreachable!(
                                        "Struct was checked for the field: {field}'s existance in an earlier branch, but did not exist: {var:?}"
                                    ),
                                },
                                v => unreachable!(
                                    "Struct was either typed or inferred as a struct, it was: {v:?}"
                                ),
                            },
                            false => Err(IE::StructAccessingOnNonExistingField { ident, field }),
                        },
                        None => Err(IE::StructAccessingOnNonStruct(identifier, field, b2_type)),
                    },
                    _ => Err(IE::StructAccessingOnNonStruct(identifier, field, b2_type)),
                }
            }
            None => Err(IE::UndefinedVariable(identifier)),
        },
        PExpr::Enum { values, .. } => Ok(values),
    }
}

fn infer_type<'a>(gst: &GlobalSymbolTable<'a>, value: &PExpr<'a>) -> Type<'a> {
    match value {
        PExpr::Nil => Type::nil(),
        PExpr::Literal(primitive) => match primitive {
            Primitive::Int(_) => Type::int(),
            Primitive::Float(_) => Type::float(),
            Primitive::Str(_) => Type::str(),
            Primitive::Bool(_) => Type::bool(),
        },
        PExpr::Tuple(fst, snd) => Type::tuple(infer_type(gst, fst), infer_type(gst, snd)),
        PExpr::List(xs) => Type::list(xs.first().map(|x| infer_type(gst, x)).unwrap_or_default()),
        PExpr::Variable(var) => match gst.lookup_var(var) {
            Some(var) => var
                .b2_type()
                .cloned()
                .unwrap_or_else(|| infer_type(gst, var.value())),
            None => todo!(),
        },
        PExpr::Group(lex_expr) => infer_type(gst, lex_expr),
        PExpr::FunctionCall { arguments, .. } => {
            let xs = arguments
                .iter()
                .map(|x| infer_type(gst, x))
                .collect::<Vec<_>>();
            match xs.as_slice() {
                [] => Type::fun(Type::nil(), Type::nil()),
                [x] => Type::fun(x.clone(), Type::nil()),
                [x, y] => Type::fun(x.clone(), y.clone()),
                xs => TypeParser::fold_funs(xs.to_vec())
                    .unwrap_or_else(|_| Type::fun(Type::nil(), Type::nil())),
            }
        }
        PExpr::Op(b2_op) => match b2_op.inner() {
            B2OpInner::Prefix(o, x) => match o {
                UniOp::Neg => infer_type(gst, x),
            },
            B2OpInner::Postfix(x, o) => match o {
                Postfix::Incr | Postfix::Decr | Postfix::Index(_) => infer_type(gst, x),
            },
            B2OpInner::Binary(l, o, r) => match o {
                BinOp::Mul | BinOp::Sub | BinOp::Add => {
                    match (infer_type(gst, l), infer_type(gst, r)) {
                        (Type::Mono(MonoType::Str), Type::Mono(_)) => Type::str(),
                        (Type::Mono(_), Type::Mono(MonoType::Str)) => Type::str(),
                        (Type::Mono(MonoType::Float), Type::Mono(_)) => Type::float(),
                        (Type::Mono(_), Type::Mono(MonoType::Float)) => Type::float(),
                        (l, _) => l,
                    }
                }
                BinOp::Mod => Type::int(),
                BinOp::Div | BinOp::Pow => match (infer_type(gst, l), infer_type(gst, r)) {
                    (Type::Mono(MonoType::Float), Type::Mono(_)) => Type::float(),
                    (Type::Mono(_), Type::Mono(MonoType::Float)) => Type::float(),
                    (l, _) => l,
                },
                BinOp::Eq
                | BinOp::Geq
                | BinOp::Gt
                | BinOp::Leq
                | BinOp::Lt
                | BinOp::Neq
                | BinOp::And
                | BinOp::Or => Type::bool(),
            },
        },
        PExpr::Struct {
            ident: identifier, ..
        } => Type::var(identifier),
        PExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => gst
            .lookup_struct_decl(identifier)
            .and_then(|d| d.get_field(field))
            .cloned()
            .unwrap_or_default(),
        PExpr::Enum {
            ident: identifier,
            instance,
            ..
        } => Type::r#enum(identifier, instance),
    }
}

fn invoke_function<'a>(
    gst: &GlobalSymbolTable<'a>,
    identifier: &'a str,
    arguments: Vec<PExpr<'a>>,
) -> Result<PExpr<'a>, IE<'a>> {
    let fun = gst
        .lookup_fn_impl(identifier)
        .ok_or(IE::FnErrMissingImplementation(identifier))?;

    let parameters = &fun.parameters;

    if arguments.len() > parameters.len() {
        return Err(IE::MoreArgumentsSuppliedToFunctionThanExpected(
            identifier,
            arguments,
            parameters.clone(),
        ));
    }

    let mut function_gst = gst.clone().scope();

    for (i, (arg, (par_id, opt_par))) in arguments
        .into_iter()
        .map(|x| Some(x))
        .chain((0..(parameters.len())).map(|_| None))
        .zip(parameters)
        .enumerate()
    {
        match (arg, opt_par.clone()) {
            (None, None) => {
                return Err(IE::FnErrMissingArgument(identifier, i, par_id));
            }
            (Some(arg), _) | (_, Some(arg)) => {
                function_gst = function_gst.insert_var_ass(
                    par_id,
                    VariableDeclarationAssignment::new(par_id, None, arg),
                );
            }
        }
    }

    for stmt in fun.body.iter() {
        match stmt {
            PStmt::Return { value } => {
                let val: PExpr<'a> = eval_expr(&function_gst, value.clone().unwrap_or_default())?;
                return Ok(val.clone());
            }
            _ => match interpret(function_gst.clone(), stmt.clone()) {
                IR::Ok { gst, .. } => {
                    function_gst = gst;
                }
                IR::Err(ie) => {
                    return Err(ie);
                }
            },
        }
    }

    Ok(PExpr::Nil)
}

fn eval_operation<'a>(gst: &GlobalSymbolTable<'a>, op: B2Op<'a>) -> Result<PExpr<'a>, IE<'a>> {
    todo!()
}

fn eval_expr<'a>(gst: &GlobalSymbolTable<'a>, expr: PExpr<'a>) -> Result<PExpr<'a>, IE<'a>> {
    match expr {
        PExpr::Variable(ident) => match gst.lookup_var(ident) {
            Some(dcl) => eval_expr(gst, dcl.value().clone()),
            None => Err(IE::UndefinedVariable(ident)),
        },
        PExpr::Group(inner) => eval_expr(gst, *inner),
        PExpr::FunctionCall {
            ident: identifier,
            arguments,
        } => invoke_function(&gst, identifier, arguments),
        PExpr::Op(op) => eval_operation(&gst, *op),
        PExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => match gst.lookup_var(identifier) {
            Some(dcl) => match dcl.value() {
                PExpr::Struct {
                    field_implementations,
                    ..
                } => field_implementations.get(field).cloned().ok_or_else(|| {
                    IE::StructAccessingOnNonExistingField {
                        ident: identifier,
                        field,
                    }
                }),
                other => Err(IE::StructAccessingOnNonStruct(
                    identifier,
                    field,
                    infer_type(&gst, other),
                )),
            },
            None => Err(IE::StructAccessingOnNonExistingStruct {
                ident: identifier,
                field,
            }),
        },
        PExpr::Nil
        | PExpr::Literal(_)
        | PExpr::Tuple(_, _)
        | PExpr::List(_)
        | PExpr::Struct { .. }
        | PExpr::Enum { .. } => Ok(expr),
    }
}
