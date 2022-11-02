//! Prints the AST, but also tag node with addition info.

use std::fmt::Debug;

use crate::{node_id::NodeInfo, pass::ExprListener};

pub struct Inspector<'a, T: Debug> {
    info: &'a NodeInfo<T>,
}

impl<'a, T: Debug> Inspector<'a, T> {
    pub fn new(info: &'a NodeInfo<T>) -> Self {
        Inspector { info }
    }
}

impl<'a, T: Debug> ExprListener for Inspector<'a, T> {
    fn walk_varref(&mut self, id: &String, eself: &crate::ast::Expr) {
        println!("inspector: varref got {id} -> {:?}", self.info.get(eself));
    }
}
