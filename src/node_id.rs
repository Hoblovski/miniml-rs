use std::collections::HashMap;

use crate::ast::Expr;

// see rustc_ast. They use a `NodeId`.
// Why use references as keys:
//  1. Python/Haskell/Coq tradition.
//  2. If rust is safe, then the hashmap is safe i.e. all keys will be valid.
// Seems the requirement is exfalso.
// https://doc.rust-lang.org/beta/nightly-rustc/src/rustc_ast/node_id.rs.html
//
// TODO: THIS FUNCTION IS VERY UNSAFE. ensure Nodes are pinned.
pub unsafe fn node_id(e: &Expr) -> NodeId {
    NodeId(e as *const _ as usize)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

pub struct NodeInfo<T> {
    info: HashMap<NodeId, T>,
}

impl<T> NodeInfo<T> {
    pub fn new() -> Self {
        Self {
            info: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: &Expr, v: T) {
        self.info.insert(unsafe { node_id(k) }, v);
    }

    pub fn get(&self, k: &Expr) -> Option<&T> {
        self.info.get(&unsafe { node_id(k) })
    }

    pub fn contains(&self, k: &Expr) -> bool {
        self.info.contains_key(&unsafe { node_id(k) })
    }
}
