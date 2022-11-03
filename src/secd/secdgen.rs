use std::collections::HashMap;

use crate::{
    ast::{Expr, Ty},
    debrujin::{DeBrujinIdx, DeBrujinInfo},
    pass::ExprVisitor,
};

use super::{
    langdef::{BrOp, SECDInstr, SECDVal},
    repr::{translate_binop, translate_builtinop, translate_unaop},
};

/// * `label_instrs`: maps function name to its instructions.
pub struct SECDGen {
    label_instrs: HashMap<String, Vec<SECDInstr>>,
    label_suffix: HashMap<String, usize>,
    debrujin_info: DeBrujinInfo,
}

// todo: str than String
impl SECDGen {
    pub fn new(debrujin_info: DeBrujinInfo) -> Self {
        Self {
            label_instrs: HashMap::new(),
            label_suffix: HashMap::new(),
            debrujin_info,
        }
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let suffix = self.label_suffix.get(prefix).unwrap_or(&0);
        let res = format!("{}{}", prefix, suffix);
        self.label_suffix.insert(prefix.to_string(), suffix + 1);
        res
    }

    pub fn visit_main_expr(&mut self, main_expr: &Expr) {
        let mut main_instrs = self.visit(main_expr);
        main_instrs.push(SECDInstr::Halt);
        self.label_instrs.insert("main".to_string(), main_instrs);
    }

    pub fn assemble(&self) -> String {
        let mut lines = Vec::<String>::new();
        for (fnlabel, fninstrs) in self.label_instrs.iter() {
            lines.push(format!("{fnlabel}:"));
            lines.extend(fninstrs.iter().map(|x| format!("{x}")));
            lines.push("\n".to_string());
        }
        lines.join("\n")
    }
}

impl ExprVisitor<Vec<SECDInstr>> for SECDGen {
    fn default(&mut self) -> Vec<SECDInstr> {
        todo!()
    }

    fn join_results(&mut self, res: Vec<Vec<SECDInstr>>) -> Vec<SECDInstr> {
        res.concat()
    }

    fn visit_seq(&mut self, subs: &Vec<Box<Expr>>, _eself: &Expr) -> Vec<SECDInstr> {
        subs.iter()
            .map(|x| self.visit(x))
            .collect::<Vec<_>>()
            .join(&SECDInstr::Pop(1))
    }

    fn visit_app(&mut self, fun: &Expr, arg: &Expr, _eself: &Expr) -> Vec<SECDInstr> {
        [self.visit(fun), self.visit(arg), vec![SECDInstr::Apply]].concat()
    }

    // todo: isize vs i64
    fn visit_intlit(&mut self, val: &i64, _eself: &Expr) -> Vec<SECDInstr> {
        vec![SECDInstr::Const(SECDVal::IntVal(*val as isize))]
    }

    fn visit_varref(&mut self, _id: &String, eself: &Expr) -> Vec<SECDInstr> {
        match self.debrujin_info.get(eself).unwrap() {
            DeBrujinIdx::Var(idx) => vec![SECDInstr::Access(1 + idx)],
            DeBrujinIdx::Rec(fnidx, subidx) => {
                vec![SECDInstr::Access(1 + fnidx), SECDInstr::Focus(1 + subidx)]
            }
        }
    }

    fn visit_abs(
        &mut self,
        _arg_name: &String,
        _arg_ty: &Ty,
        body: &Expr,
        _eself: &Expr,
    ) -> Vec<SECDInstr> {
        let label = self.new_label("lam");
        let mut instrs = self.visit(body);
        instrs.push(SECDInstr::Return);
        self.label_instrs.insert(label.clone(), instrs);
        vec![SECDInstr::Closure(label)]
    }

    fn visit_builtin(&mut self, op: &crate::ast::BuiltinOp, _eself: &Expr) -> Vec<SECDInstr> {
        vec![SECDInstr::Builtin(translate_builtinop(*op))]
    }

    fn visit_ite(&mut self, cond: &Expr, tr: &Expr, fl: &Expr, _eself: &Expr) -> Vec<SECDInstr> {
        let (l1, l2, l3) = (
            self.new_label("tr"),
            self.new_label("fl"),
            self.new_label("endif"),
        );
        vec![
            self.visit(cond),
            vec![
                SECDInstr::Branch(BrOp::BrFalse, l2.clone()),
                SECDInstr::Label(l1),
            ],
            self.visit(tr),
            vec![
                SECDInstr::Branch(BrOp::Br, l3.clone()),
                SECDInstr::Label(l2),
            ],
            self.visit(fl),
            vec![SECDInstr::Label(l3)],
        ]
        .concat()
    }

    fn visit_binary(
        &mut self,
        lhs: &Expr,
        op: &crate::ast::BinOp,
        rhs: &Expr,
        _eself: &Expr,
    ) -> Vec<SECDInstr> {
        vec![
            self.visit(lhs),
            self.visit(rhs),
            vec![SECDInstr::Binary(translate_binop(*op))],
        ]
        .concat()
    }

    fn visit_unary(&mut self, op: &crate::ast::UnaOp, sub: &Expr, _eself: &Expr) -> Vec<SECDInstr> {
        vec![
            self.visit(sub),
            vec![SECDInstr::Unary(translate_unaop(*op))],
        ]
        .concat()
    }

    fn visit_let(
        &mut self,
        _name: &String,
        _ty: &Ty,
        val: &Expr,
        body: &Expr,
        _eself: &Expr,
    ) -> Vec<SECDInstr> {
        vec![self.visit(val), vec![SECDInstr::PushEnv], self.visit(body)].concat()
    }

    fn visit_letrecarm(&mut self, e: &crate::ast::LetRecArm) -> Vec<SECDInstr> {
        let label = self.new_label("clos");
        let instrs = vec![self.visit(&e.body), vec![SECDInstr::Return]].concat();
        self.label_instrs.insert(label.clone(), instrs);
        // This is actually misuse since we need a label rather than an instruction.
        vec![SECDInstr::Label(label)]
    }

    fn visit_letrec(
        &mut self,
        arms: &Vec<crate::ast::LetRecArm>,
        body: &Expr,
        _eself: &Expr,
    ) -> Vec<SECDInstr> {
        let arms_labels = arms
            .iter()
            .map(|x| {
                let res = self.visit_letrecarm(x);
                assert!(res.len() == 1);
                match res.into_iter().nth(0) {
                    Some(SECDInstr::Label(l)) => l,
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();
        vec![vec![SECDInstr::Closures(arms_labels)], self.visit(body)].concat()
    }
}

// todo: borrow debrujin info
pub fn secdgen(debrujin_info: DeBrujinInfo, main_expr: &Expr) -> String {
    let mut secdgen = SECDGen::new(debrujin_info);
    secdgen.visit_main_expr(main_expr);
    let secd_prog = secdgen.assemble();
    secd_prog
}
