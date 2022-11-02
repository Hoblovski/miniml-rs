//! Visitor that does not mutate the AST.
use crate::ast::*;

#[allow(unused_variables)]
/// Does auto un-packing.
pub trait ExprVisitor<R> {
    fn default(&mut self) -> R;

    fn join_results(&mut self, res: Vec<R>) -> R {
        self.default()
    }

    fn visit_intlit(&mut self, val: &i64, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_unitlit(&mut self, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_binary(&mut self, lhs: &Expr, op: &BinOp, rhs: &Expr, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_unary(&mut self, op: &UnaOp, sub: &Expr, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_varref(&mut self, id: &String, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_builtin(&mut self, op: &BuiltinOp, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_app(&mut self, fun: &Expr, arg: &Expr, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_seq(&mut self, subs: &Vec<Box<Expr>>, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_abs(&mut self, arg_name: &String, arg_ty: &Ty, body: &Expr, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_let(&mut self, name: &String, ty: &Ty, val: &Expr, body: &Expr, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_tuple(&mut self, subs: &Vec<Box<Expr>>, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_nth(&mut self, idx: &i64, sub: &Expr, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_ite(&mut self, cond: &Expr, tr: &Expr, fl: &Expr, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_letrec(&mut self, arms: &Vec<LetRecArm>, body: &Expr, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_match(&mut self, sub: &Expr, arms: &Vec<MatchArm>, eself: &Expr) -> R {
        self.visit_children(eself)
    }

    fn visit_letrecarm(&mut self, e: &LetRecArm) -> R {
        self.visit(&e.body)
    }

    fn visit_matcharm(&mut self, e: &MatchArm) -> R {
        self.visit(&e.res)
    }

    fn visit_children(&mut self, e: &Expr) -> R {
        use Expr::*;
        match e {
            Binary {
                box lhs,
                op: _,
                box rhs,
            } => {
                let chs = vec![self.visit(lhs), self.visit(rhs)];
                self.join_results(chs)
            }
            Unary { op: _, box sub } => {
                let chs = vec![self.visit(sub)];
                self.join_results(chs)
            }
            App { box fun, box arg } => {
                let chs = vec![self.visit(fun), self.visit(arg)];
                self.join_results(chs)
            }
            Seq { subs } => {
                let chs = subs.iter().map(|x| self.visit(x)).collect();
                self.join_results(chs)
            }
            Abs {
                arg_name: _,
                arg_ty: _,
                box body,
            } => {
                let chs = vec![self.visit(body)];
                self.join_results(chs)
            }
            Let {
                name: _,
                ty: _,
                val,
                body,
            } => {
                let chs = vec![self.visit(val), self.visit(body)];
                self.join_results(chs)
            }
            Tuple { subs } => {
                let chs = subs.iter().map(|x| self.visit(x)).collect();
                self.join_results(chs)
            }
            Nth { idx: _, box sub } => {
                let chs = vec![self.visit(sub)];
                self.join_results(chs)
            }
            Ite {
                box cond,
                box tr,
                box fl,
            } => {
                let chs = vec![self.visit(cond), self.visit(tr), self.visit(fl)];
                self.join_results(chs)
            }
            LetRec { arms, box body } => {
                let mut chs: Vec<R> = arms.iter().map(|x| self.visit_letrecarm(x)).collect();
                chs.push(self.visit(body));
                self.join_results(chs)
            }
            Match { sub, arms } => {
                let mut chs = vec![self.visit(sub)];
                chs.extend(arms.iter().map(|x| self.visit_matcharm(x)));
                self.join_results(chs)
            }
            _ => todo!(),
        }
    }

    /// Impl can call default_visit even if it overrides visit.
    fn default_visit(&mut self, e: &Expr) -> R {
        use Expr::*;
        match e {
            IntLit { val } => self.visit_intlit(val, e),
            UnitLit {} => self.visit_unitlit(e),
            Binary { lhs, op, rhs } => self.visit_binary(lhs, op, rhs, e),
            Unary { op, sub } => self.visit_unary(op, sub, e),
            VarRef { id } => self.visit_varref(id, e),
            Builtin { op } => self.visit_builtin(op, e),
            App { fun, arg } => self.visit_app(fun, arg, e),
            Seq { subs } => self.visit_seq(subs, e),
            Abs {
                arg_name,
                arg_ty,
                body,
            } => self.visit_abs(arg_name, arg_ty, body, e),
            Let {
                name,
                ty,
                val,
                body,
            } => self.visit_let(name, ty, val, body, e),
            Tuple { subs } => self.visit_tuple(subs, e),
            Nth { idx, sub } => self.visit_nth(idx, sub, e),
            Ite { cond, tr, fl } => self.visit_ite(cond, tr, fl, e),
            LetRec { arms, body } => self.visit_letrec(arms, body, e),
            _ => todo!(),
        }
    }

    fn visit(&mut self, e: &Expr) -> R {
        self.default_visit(e)
    }
}
