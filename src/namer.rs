use crate::ast::*;
/// The namer pass.
use crate::utils::*;
use crate::visitor::ExprVisitorMut;

use std::collections::HashMap;
use std::result::Result;
use std::vec::Vec;

/// Visits the AST and renames the variables into unique names.
/// Basically does an alpha conversion pass.
pub struct Namer {
    name_cnt: HashMap<String, i64>,
    vars: Vec<(String, String)>,
}

#[derive(Debug)]
pub enum NamerErrKind {
    UnknownVarRef { id: String },
    DuplicateLetRecFn {},
}

type NamerResult = Result<(), NamerErrKind>;

impl Namer {
    pub fn new() -> Self {
        let name_cnt = HashMap::new();
        let vars = Vec::new();
        Namer { name_cnt, vars }
    }

    fn gen_name(&mut self, name: &str) -> String {
        let suffix = *self.name_cnt.get(name).unwrap_or(&0);
        self.name_cnt.insert(name.to_string(), suffix + 1);
        return format!("{}{}@{}", "_", name, suffix);
    }

    fn def_var(&mut self, old: &str) -> String {
        let new = self.gen_name(old);
        self.vars.push((old.to_string(), new.clone()));
        // OPT: redundant to_string as already in gen_name
        return new;
    }

    fn undef_var(&mut self, new: &String) {
        let (_old, new_) = self.vars.pop().unwrap();
        if &new_ != new {
            panic!("undef var unmatch: {} != {}", new, new_);
        }
    }
}

impl ExprVisitorMut<NamerResult> for Namer {
    fn default(&mut self) -> NamerResult {
        Ok(())
    }

    fn join_results(&mut self, res: Vec<NamerResult>) -> NamerResult {
        res.into_iter().find(|x| x.is_err()).unwrap_or(Ok(()))
    }

    fn visit_varref(&mut self, e: &mut Expr) -> NamerResult {
        match e {
            Expr::VarRef { id } => {
                for (old, new) in self.vars.iter().rev() {
                    if old == id {
                        *id = new.clone();
                        return Ok(());
                    }
                }
                Err(NamerErrKind::UnknownVarRef { id: id.clone() })
            }
            _ => unreachable!(),
        }
    }

    fn visit_abs(&mut self, e: &mut Expr) -> NamerResult {
        match e {
            Expr::Abs {
                arg_name,
                arg_ty: _,
                box body,
            } => {
                let new = self.def_var(arg_name);
                *arg_name = new;
                self.visit(body)?;
                self.undef_var(arg_name);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn visit_let(&mut self, e: &mut Expr) -> NamerResult {
        match e {
            Expr::Let {
                name,
                ty: _,
                box val,
                box body,
            } => {
                *name = self.def_var(name);
                self.visit(val)?;
                self.visit(body)?;
                self.undef_var(name);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn visit_letrec(&mut self, e: &mut Expr) -> NamerResult {
        match e {
            Expr::LetRec { arms, box body } => {
                if !uniq(arms.iter().map(|x| &x.fn_name)) {
                    return Err(NamerErrKind::DuplicateLetRecFn {});
                }

                for arm in arms.iter_mut() {
                    arm.fn_name = self.def_var(&arm.fn_name);
                }

                for arm in arms.iter_mut() {
                    arm.arg_name = self.def_var(&arm.arg_name);
                    self.visit(&mut arm.body)?;
                    self.undef_var(&arm.arg_name);
                }

                self.visit(body)?;

                for arm in arms.iter_mut() {
                    self.undef_var(&arm.fn_name);
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}
