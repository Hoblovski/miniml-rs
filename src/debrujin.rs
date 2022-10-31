//! DeBrujin conversion pass

use std::collections::VecDeque;

use crate::{
    ast::{Expr, Ty},
    node_id::NodeInfo,
    visitor::ExprListener,
};

pub struct DeBrujin {
    // TODO: use str
    vars: VecDeque<String>,
    info: NodeInfo<usize>,
}

impl DeBrujin {
    pub fn new() -> Self {
        DeBrujin {
            vars: VecDeque::new(),
            info: NodeInfo::new(),
        }
    }

    pub fn get_info(&self) -> &NodeInfo<usize> {
        &self.info
    }

    fn get_debrujin_idx(&self, varname: &String) -> Option<usize> {
        for (i, v) in self.vars.iter().enumerate() {
            if v == varname {
                return Some(i);
            }
        }
        None
    }

    fn define_var(&mut self, id: &String) {
        self.vars.push_front(id.clone());
    }

    fn undefine_var(&mut self, id: &String) {
        assert!(self.vars.front().unwrap() == id);
        self.vars.pop_front();
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
}
