//! DeBrujin conversion pass

use std::collections::VecDeque;

use crate::{
    ast::{Expr, LetRecArm, Ty},
    node_id::NodeInfo,
    pass::ExprListener,
};

#[derive(Debug, Clone)]
pub enum DeBrujinIdx {
    Var(usize),
    Rec(usize, usize),
}

#[derive(Debug)]
enum VarBundle {
    Var(String),
    Rec(Vec<String>),
}

pub type DeBrujinInfo = NodeInfo<DeBrujinIdx>;

pub struct DeBrujin {
    // TODO: use str
    vars: VecDeque<VarBundle>,
    info: DeBrujinInfo,
}

impl DeBrujin {
    pub fn new() -> Self {
        DeBrujin {
            vars: VecDeque::new(),
            info: NodeInfo::new(),
        }
    }

    pub fn get_info(self) -> DeBrujinInfo {
        self.info
    }

    fn get_debrujin_idx(&self, id: &String) -> Option<DeBrujinIdx> {
        for (i, v) in self.vars.iter().enumerate() {
            match v {
                VarBundle::Var(v) if v == id =>
                    return Some(DeBrujinIdx::Var(i)),
                VarBundle::Rec(vs) if let Some(idx) = vs.iter().position(|x| x == id) =>
                    return Some(DeBrujinIdx::Rec(i, idx)),
                _ => (),
            }
        }
        None
    }

    fn define_var(&mut self, id: &String) {
        self.vars.push_front(VarBundle::Var(id.clone()));
    }

    fn define_rec(&mut self, rec: &Vec<String>) {
        self.vars.push_front(VarBundle::Rec(rec.clone()));
    }

    fn undefine_var(&mut self, id: &String) {
        if let Some(VarBundle::Var(id_)) = self.vars.pop_front() {
            if &id_ == id {
                return;
            }
        }
        unreachable!()
    }

    fn undefine_rec(&mut self, rec: &Vec<String>) {
        if let Some(VarBundle::Rec(vs)) = self.vars.pop_front() {
            if &vs == rec {
                return;
            }
        }
        unreachable!()
    }
}

impl ExprListener for DeBrujin {
    fn walk_varref(&mut self, id: &String, eself: &Expr) {
        let dbid = self.get_debrujin_idx(id).unwrap();
        self.info.insert(eself, dbid);
    }

    fn enter_abs(&mut self, arg_name: &String, _arg_ty: &Ty, _body: &Expr, _eself: &Expr) {
        self.define_var(arg_name);
    }

    fn exit_abs(&mut self, arg_name: &String, _arg_ty: &Ty, _body: &Expr, _eself: &Expr) {
        self.undefine_var(arg_name);
    }

    fn walk_let(
        &mut self,
        name: &String,
        _ty: &Ty,
        val: &Box<Expr>,
        body: &Box<Expr>,
        _eself: &Expr,
    ) {
        self.walk(val);
        self.define_var(name);
        self.walk(body);
        self.undefine_var(name);
    }

    fn enter_letrec(&mut self, arms: &Vec<crate::ast::LetRecArm>, _body: &Expr, _eself: &Expr) {
        let rec = arms.iter().map(|x| x.fn_name.clone()).collect::<Vec<_>>();
        self.define_rec(&rec);
    }

    fn exit_letrec(&mut self, arms: &Vec<crate::ast::LetRecArm>, _body: &Expr, _eself: &Expr) {
        let rec = arms.iter().map(|x| x.fn_name.clone()).collect::<Vec<_>>();
        self.undefine_rec(&rec);
    }

    fn enter_letrecarm(&mut self, arm: &LetRecArm) {
        self.define_var(&arm.arg_name);
    }

    fn exit_letrecarm(&mut self, arm: &LetRecArm) {
        self.undefine_var(&arm.arg_name);
    }
}
