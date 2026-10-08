use crate::interpreter::{
    gst::GlobalSymbolTable,
    gstate::{IE, IR},
};
use parser::{
    common::{
        AsB2Type, B2Op, B2OpInner, binop::BinOp, postfix::Postfix, primitive::Primitive,
        uniop::UniOp,
    },
    lex_expr::LexExpr,
    lex_stmt::{
        Import, LexStmt,
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
    lex_type::{LexMonoType, LexPolyType, LexType},
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
    indexee: LexExpr<'a>,
    new_value: LexExpr<'a>,
) -> Result<LexExpr<'a>, IE<'a>> {
    match indexee {
        LexExpr::Nil
        | LexExpr::Literal(_)
        | LexExpr::Op(_)
        | LexExpr::Struct { .. }
        | LexExpr::Enum { .. } => Err(IE::IndexingErrorInvalidIndexee { indexee }),

        /* NOTE: This changes the tree, technically, by removing the group, does
           it matter? I dont think so.
        */
        LexExpr::Group(inner) => index_expr(gst, i, *inner, new_value),

        /* NOTE: Maybe stupid to allow this:
        LET (foo, bar, qaz) >< (1, (2, 3));

        But disallow this:

        LET qaz = (1, (2, 3))[2];

        This would be the "correct way"
        LET qaz = (1, (2, 3))[1][0];

        but idk.
        */
        LexExpr::Tuple(l, r) => match i {
            0 => Ok(LexExpr::Tuple(Box::new(new_value), r)),
            1 => Ok(LexExpr::Tuple(l, Box::new(new_value))),
            _ => Err(IE::IndexingErrorOutOfBounds {
                bounds: 2,
                index: i as isize,
            }),
        },

        LexExpr::List(mut xs) => {
            let bounds = xs.len();
            if i < 0 || i as usize <= bounds {
                return Err(IE::IndexingErrorOutOfBounds {
                    index: i as isize,
                    bounds,
                });
            }
            xs[i as usize] = new_value;

            Ok(LexExpr::List(xs))
        }

        LexExpr::Variable(ident) => match gst.lookup_var(ident).cloned() {
            Some(VariableDeclarationAssignment { value, .. }) => {
                index_expr(gst, i, value, new_value)
            }
            None => Err(IE::MissingVariableDeclaration(ident)),
        },

        LexExpr::FunctionCall {
            ident: identifier,
            arguments,
        } => invoke_function(gst, identifier, arguments)
            .and_then(|idxe| index_expr(gst, i, idxe, new_value)),

        LexExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => {
            todo!("I think this is not implemented well enough")
        }
    }
}

pub fn interpret<'a>(gst: GlobalSymbolTable<'a>, stmt: LexStmt<'a>) -> IR<'a, LexExpr<'a>> {
    match stmt {
        LexStmt::Decl(dcl) => interpret_decl(gst, dcl).map(|_| LexExpr::Nil),
        LexStmt::Impl(ipl) => interpret_impl(gst, ipl).map(|_| LexExpr::Nil),
        LexStmt::Import(import) => import_module(gst, import).map(|_| LexExpr::Nil),
        LexStmt::VariableReassignment {
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
            val: LexExpr::Nil,
        },
        LexStmt::ListReassignment {
            indexee,
            index,
            reassignment,
            new_value,
        } => match &index {
            LexExpr::Literal(Primitive::Int(i)) => match index_expr(&gst, *i, indexee, new_value) {
                Ok(val) => IR::Ok { gst, val },
                Err(err) => IR::Err(err),
            },
            LexExpr::Variable(_) => todo!(),
            LexExpr::Group(lex_expr) => todo!(),
            LexExpr::FunctionCall {
                ident: identifier,
                arguments,
            } => todo!(),
            LexExpr::StructFieldAccessing {
                ident: identifier,
                field,
            } => todo!(),

            LexExpr::Nil
            | LexExpr::Tuple(_, _)
            | LexExpr::List(_)
            | LexExpr::Op(_)
            | LexExpr::Struct { .. }
            | LexExpr::Literal(_)
            | LexExpr::Enum { .. } => IR::Err(IE::IndexingErrorInvalidType { indexee, index }),
        },
        LexStmt::If { condition, body } => match eval_expr(&gst, condition) {
            Ok(LexExpr::Literal(Primitive::Bool(b))) if b => IR::Ok {
                gst,
                val: LexExpr::Nil,
            }
            .scope(|(gst, _)| {
                body.into_iter().fold(
                    IR::Ok {
                        gst,
                        val: LexExpr::Nil,
                    },
                    |acc, stmt| acc.and_then(|(gst_prime, _)| interpret(gst_prime, stmt)),
                )
            }),
            Ok(LexExpr::Literal(Primitive::Bool(_))) => IR::Ok {
                gst,
                val: LexExpr::Nil,
            },
            Ok(v) => IR::Err(IE::ExpectedBoolGot(v)),
            Err(err) => IR::Err(err),
        },
        LexStmt::While { condition, body } => IR::Ok {
            gst,
            val: LexExpr::Nil,
        }
        .scope(|(gst, _)| interpret_while(gst, condition, body)),
        LexStmt::Block { body } => IR::Ok {
            gst,
            val: LexExpr::Nil,
        }
        .scope(|(gst, val)| {
            body.into_iter().fold(IR::Ok { gst, val }, |acc, cur| {
                acc.and_then(|(gst, _)| interpret(gst, cur))
            })
        }),
        LexStmt::FunctionInvocation { ident, arguments } => invoke_function(&gst, ident, arguments)
            .map(|val| IR::Ok { gst, val })
            .into(),
        LexStmt::Break => todo!(),
        LexStmt::Continue => todo!(),
        LexStmt::Return { value } => IR::Ok {
            gst,
            val: value.unwrap_or_default(),
        },
        LexStmt::For {
            start_stmt,
            condition,
            incrementer,
            body,
        } => IR::Ok {
            gst,
            val: LexExpr::Nil,
        }
        .scope(|(gst, _)| {
            let ident = start_stmt.identifier;
            let gst = gst.insert_var_ass(ident, start_stmt);
            interpret_for(gst, ident, incrementer, condition, body)
        }),
        LexStmt::StructFieldReassignment {
            ident,
            field,
            reassignment,
            new_value,
        } => match gst.lookup_var(ident) {
            Some(VariableDeclarationAssignment {
                value:
                    LexExpr::Struct {
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
                                    variable_type: Some(LexType::r#struct(struct_ident)),
                                    value: LexExpr::Struct {
                                        ident: *struct_ident,
                                        field_implementations,
                                    },
                                },
                            ),
                            val: LexExpr::Nil,
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
        LexStmt::WhenStatement { ident, branches } => match gst.lookup_var(ident) {
            Some(VariableDeclarationAssignment {
                value: LexExpr::List(xs),
                ..
            }) => {
                for (mtch, body) in branches {
                    match (mtch, xs.as_slice()) {
                        (WhenMatch::EmptyList { condition }, []) => {
                            match condition.map(|expr| eval_expr(&gst, expr)) {
                                Some(Err(err)) => {
                                    return IR::Err(err);
                                }
                                Some(Ok(LexExpr::Literal(Primitive::Bool(b)))) if b => {
                                    return IR::Ok {
                                        gst,
                                        val: LexExpr::Nil,
                                    }
                                    .scope(|(gst, _)| {
                                        body.into_iter().fold(
                                            IR::Ok {
                                                gst,
                                                val: LexExpr::Nil,
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
                                        val: LexExpr::Nil,
                                    }
                                    .scope(|(gst, _)| {
                                        body.into_iter().fold(
                                            IR::Ok {
                                                gst,
                                                val: LexExpr::Nil,
                                            },
                                            move |acc, stmt| {
                                                acc.and_then(|(gst, _)| interpret(gst, stmt))
                                            },
                                        )
                                    });
                                }
                                Some(Ok(LexExpr::Literal(Primitive::Bool(_)))) => continue,
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
                                Some(Ok(LexExpr::Literal(Primitive::Bool(b)))) if b => {
                                    return IR::Ok {
                                        gst,
                                        val: LexExpr::Nil,
                                    }
                                    .scope(|(gst, _)| {
                                        body.into_iter().fold(
                                            IR::Ok {
                                                gst,
                                                val: LexExpr::Nil,
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
                                        val: LexExpr::Nil,
                                    }
                                    .scope(|(gst, _)| {
                                        body.into_iter().fold(
                                            IR::Ok {
                                                gst,
                                                val: LexExpr::Nil,
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
                                Some(Ok(LexExpr::Literal(Primitive::Bool(_)))) => continue,
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
                                        value: LexExpr::List(ys),
                                    },
                                );
                            }

                            match condition
                                .map(|expr| eval_expr(&gst, expr))
                                .unwrap_or_else(|| Ok(LexExpr::bool(true)))
                            {
                                Ok(LexExpr::Literal(Primitive::Bool(b))) => {
                                    if b {
                                        return body
                                            .into_iter()
                                            .fold(
                                                IR::Ok {
                                                    gst,
                                                    val: LexExpr::Nil,
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
                                    value: LexExpr::List(xs.clone()),
                                },
                            );
                            match condition
                                .map(|expr| eval_expr(&gst, expr))
                                .unwrap_or_else(|| Ok(LexExpr::bool(true)))
                            {
                                Ok(LexExpr::Literal(Primitive::Bool(b))) => {
                                    if b {
                                        return body
                                            .into_iter()
                                            .fold(
                                                IR::Ok {
                                                    gst,
                                                    val: LexExpr::Nil,
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
                    val: LexExpr::Nil,
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
    condition: LexExpr<'a>,
    body: Vec<LexStmt<'a>>,
) -> IR<'a, LexExpr<'a>> {
    match eval_expr(&gst, condition) {
        Ok(LexExpr::Literal(Primitive::Bool(b))) if b => body
            .clone()
            .into_iter()
            .fold(
                IR::Ok {
                    gst,
                    val: LexExpr::Nil,
                },
                |acc, cur| acc.and_then(|(gst, _)| interpret(gst, cur)),
            )
            .and_then(|(gst, _)| IR::Ok {
                gst: gst.insert_var_ass(ident, incrementer),
                val: LexExpr::Nil,
            }),
        Ok(LexExpr::Literal(Primitive::Bool(_))) => IR::Ok {
            gst,
            val: LexExpr::Nil,
        },
        Ok(_) => unreachable!("non-bool should not appear as condition in for-stmt"),
        Err(err) => IR::Err(err),
    }
}

fn interpret_while<'a>(
    gst: GlobalSymbolTable<'a>,
    condition: LexExpr<'a>,
    body: Vec<LexStmt<'a>>,
) -> IR<'a, LexExpr<'a>> {
    match eval_expr(&gst, condition.clone()) {
        Ok(LexExpr::Literal(Primitive::Bool(b))) if b => body
            .clone()
            .into_iter()
            .fold(
                IR::Ok {
                    gst,
                    val: LexExpr::Nil,
                },
                |acc, stmt| acc.and_then(|(gst_prime, _)| interpret(gst_prime, stmt)),
            )
            .and_then(|(gst, _)| interpret_while(gst, condition, body)),
        Ok(LexExpr::Literal(Primitive::Bool(_))) => IR::Ok {
            gst,
            val: LexExpr::Nil,
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
                                    value: LexExpr::List(
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
            LexExpr::Struct {
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
            let mut fst_lst = unpack_expr(gst, *fst.clone()).unwrap_or(vec![]);
            let mut snd_lst = unpack_expr(gst, *snd.clone()).unwrap_or(vec![*snd.clone()]);
            fst_lst.append(&mut snd_lst);
            Ok(fst_lst)
        }
        LexExpr::List(lst) => Ok(lst),
        LexExpr::Variable(var) => match gst.lookup_var(var) {
            Some(val) => unpack_expr(gst, val.value().clone()),
            None => Err(IE::UndefinedVariable(var)),
        },
        LexExpr::Group(inner) => unpack_expr(gst, *inner),
        LexExpr::FunctionCall {
            ident: identifier,
            arguments,
        } => unpack_expr(gst, invoke_function(&gst, identifier, arguments)?),
        LexExpr::Op(b2_op) => Ok(vec![eval_operation(&gst, *b2_op)?]),
        LexExpr::Struct {
            field_implementations: _xs,
            ..
        } => todo!("Cant unpack struct"), //Ok(_xs.into_iter().map(|(_, x)| x).collect()),
        LexExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => match gst.lookup_var(identifier) {
            Some(var) => {
                let b2_type = var
                    .b2_type()
                    .cloned()
                    .unwrap_or_else(|| infer_type(gst, var.value()));
                match b2_type {
                    LexType::Mono(LexMonoType::TypeVar(ident)) => {
                        match gst.lookup_struct_decl(ident) {
                            Some(struct_decl) => match struct_decl.has_field(field) {
                                true => match var.value() {
                                    LexExpr::Struct {
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
                                false => {
                                    Err(IE::StructAccessingOnNonExistingField { ident, field })
                                }
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

fn infer_type<'a>(gst: &GlobalSymbolTable<'a>, value: &LexExpr<'a>) -> LexType<'a> {
    match value {
        LexExpr::Nil => LexType::nil(),
        LexExpr::Literal(primitive) => match primitive {
            Primitive::Int(_) => LexType::int(),
            Primitive::Float(_) => LexType::float(),
            Primitive::Str(_) => LexType::str(),
            Primitive::Bool(_) => LexType::bool(),
        },
        LexExpr::Tuple(fst, snd) => LexType::tuple(infer_type(gst, fst), infer_type(gst, snd)),
        LexExpr::List(xs) => {
            LexType::list(xs.first().map(|x| infer_type(gst, x)).unwrap_or_default())
        }
        LexExpr::Variable(var) => match gst.lookup_var(var) {
            Some(var) => var
                .b2_type()
                .cloned()
                .unwrap_or_else(|| infer_type(gst, var.value())),
            None => todo!(),
        },
        LexExpr::Group(lex_expr) => infer_type(gst, lex_expr),
        LexExpr::FunctionCall { arguments, .. } => {
            let xs = arguments
                .iter()
                .map(|x| infer_type(gst, x))
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
                UniOp::Neg => infer_type(gst, x),
            },
            B2OpInner::Postfix(x, o) => match o {
                Postfix::Incr | Postfix::Decr | Postfix::Index(_) => infer_type(gst, x),
            },
            B2OpInner::Binary(l, o, r) => match o {
                BinOp::Mul | BinOp::Sub | BinOp::Add => {
                    match (infer_type(gst, l), infer_type(gst, r)) {
                        (LexType::Mono(LexMonoType::Str), LexType::Mono(_)) => LexType::str(),
                        (LexType::Mono(_), LexType::Mono(LexMonoType::Str)) => LexType::str(),
                        (LexType::Mono(LexMonoType::Float), LexType::Mono(_)) => LexType::float(),
                        (LexType::Mono(_), LexType::Mono(LexMonoType::Float)) => LexType::float(),
                        (l, _) => l,
                    }
                }
                BinOp::Mod => LexType::int(),
                BinOp::Div | BinOp::Pow => match (infer_type(gst, l), infer_type(gst, r)) {
                    (LexType::Mono(LexMonoType::Float), LexType::Mono(_)) => LexType::float(),
                    (LexType::Mono(_), LexType::Mono(LexMonoType::Float)) => LexType::float(),
                    (l, _) => l,
                },
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
        LexExpr::Struct {
            ident: identifier, ..
        } => LexType::var(identifier),
        LexExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => gst
            .lookup_struct_decl(identifier)
            .and_then(|d| d.get_field(field))
            .cloned()
            .unwrap_or_default(),
        LexExpr::Enum {
            ident: identifier,
            instance,
            ..
        } => LexType::r#enum(identifier, instance),
    }
}

fn invoke_function<'a>(
    gst: &GlobalSymbolTable<'a>,
    identifier: &'a str,
    arguments: Vec<LexExpr<'a>>,
) -> Result<LexExpr<'a>, IE<'a>> {
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
            LexStmt::Return { value } => {
                let val: LexExpr<'a> = eval_expr(&function_gst, value.clone().unwrap_or_default())?;
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

    Ok(LexExpr::Nil)
}

fn eval_operation<'a>(gst: &GlobalSymbolTable<'a>, op: B2Op<'a>) -> Result<LexExpr<'a>, IE<'a>> {
    todo!()
}

fn eval_expr<'a>(gst: &GlobalSymbolTable<'a>, expr: LexExpr<'a>) -> Result<LexExpr<'a>, IE<'a>> {
    match expr {
        LexExpr::Variable(ident) => match gst.lookup_var(ident) {
            Some(dcl) => eval_expr(gst, dcl.value().clone()),
            None => Err(IE::UndefinedVariable(ident)),
        },
        LexExpr::Group(inner) => eval_expr(gst, *inner),
        LexExpr::FunctionCall {
            ident: identifier,
            arguments,
        } => invoke_function(&gst, identifier, arguments),
        LexExpr::Op(op) => eval_operation(&gst, *op),
        LexExpr::StructFieldAccessing {
            ident: identifier,
            field,
        } => match gst.lookup_var(identifier) {
            Some(dcl) => match dcl.value() {
                LexExpr::Struct {
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
        LexExpr::Nil
        | LexExpr::Literal(_)
        | LexExpr::Tuple(_, _)
        | LexExpr::List(_)
        | LexExpr::Struct { .. }
        | LexExpr::Enum { .. } => Ok(expr),
    }
}
