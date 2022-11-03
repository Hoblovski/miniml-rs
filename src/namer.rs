//! The namer pass.
//! TODO: should the namer modify the ast directly or should it create some hashmap?

use crate::ast::*;
use crate::pass::ExprTransformer;
use crate::utils::*;

use std::collections::HashMap;
use std::result::Result;
use std::vec::Vec;

/// An alpha conversion pass: rename variables into unique names.
/// Variable `name` gets renamed into `_name@12` where 12 is a numerical suffix.
pub struct Namer {
    name_suffix: HashMap<String, usize>,
    old_new_varname: Vec<(String, String)>,
}

#[derive(Debug)]
pub enum NamerErrKind {
    UnknownVarRef { id: String },
    DuplicateLetRecFn {},
}

type NamerResult = Result<(), NamerErrKind>;

impl Namer {
    pub fn new() -> Self {
        let name_suffix = HashMap::new();
        let old_new_varname = Vec::new();
        Namer {
            name_suffix,
            old_new_varname,
        }
    }

    fn gen_name(&mut self, name: &str) -> String {
        let suffix = *self.name_suffix.get(name).unwrap_or(&0);
        self.name_suffix.insert(name.to_string(), suffix + 1);
        return format!("{}{}@{}", "_", name, suffix);
    }

    fn define_var(&mut self, old: &str) -> String {
        let new = self.gen_name(old);
        self.old_new_varname.push((old.to_string(), new.clone()));
        return new;
    }

    fn undefine_var(&mut self, new: &String) {
        let (_old, new_) = self.old_new_varname.pop().unwrap();
        if &new_ != new {
            panic!("undef var unmatch: {} != {}", new, new_);
        }
    }
}

impl ExprTransformer<NamerResult> for Namer {
    fn default(&mut self) -> NamerResult {
        Ok(())
    }

    fn join_results(&mut self, res: Vec<NamerResult>) -> NamerResult {
        res.into_iter().find(|x| x.is_err()).unwrap_or(Ok(()))
    }

    fn visit_varref(&mut self, e: &mut Expr) -> NamerResult {
        if let Expr::VarRef { id } = e {
            for (old, new) in self.old_new_varname.iter().rev() {
                if old == id {
                    *id = new.clone();
                    return Ok(());
                }
            }
            Err(NamerErrKind::UnknownVarRef { id: id.clone() })
        } else {
            unreachable!()
        }
    }

    fn visit_abs(&mut self, e: &mut Expr) -> NamerResult {
        if let Expr::Abs {
            arg_name,
            arg_ty: _,
            box body,
        } = e
        {
            let new = self.define_var(arg_name);
            *arg_name = new;
            self.visit(body)?;
            self.undefine_var(arg_name);
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn visit_let(&mut self, e: &mut Expr) -> NamerResult {
        if let Expr::Let {
            name,
            ty: _,
            box val,
            box body,
        } = e
        {
            *name = self.define_var(name);
            self.visit(val)?;
            self.visit(body)?;
            self.undefine_var(name);
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn visit_letrec(&mut self, e: &mut Expr) -> NamerResult {
        if let Expr::LetRec { arms, box body } = e {
            if !uniq(arms.iter().map(|x| &x.fn_name)) {
                return Err(NamerErrKind::DuplicateLetRecFn {});
            }

            for arm in arms.iter_mut() {
                arm.fn_name = self.define_var(&arm.fn_name);
            }

            for arm in arms.iter_mut() {
                arm.arg_name = self.define_var(&arm.arg_name);
                self.visit(&mut arm.body)?;
                self.undefine_var(&arm.arg_name);
            }

            self.visit(body)?;

            for arm in arms.iter_mut() {
                self.undefine_var(&arm.fn_name);
            }
            Ok(())
        } else {
            unreachable!()
        }
    }
}
