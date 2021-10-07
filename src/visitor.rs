/// Provides basic visitors.
use crate::parser::{Expr};

// TODO
// pub trait ExprVisitor<R: Default> {
//     /// The visitor itself is mut so we are able to store bookkeeping info.
// }

pub trait ExprVisitorMut<R> {
    /// The visitor itself is mut so we are able to store bookkeeping info.

    fn default(&mut self) -> R {
        panic!("undefined default")
    }

    fn join_results(&mut self, _res: Vec<R>) -> R {
        // match res.into_iter().nth(0) {
        //     Some(x) => x,
        //     None => Default::default(),
        // }
        self.default()
    }

    fn visit_binary(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_unary(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_app(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_seq(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_abs(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_let(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_tuple(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_nth(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_ite(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_letrec(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_intlit(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_unitlit(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_varref(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_builtin(&mut self, e: &mut Expr) -> R {
        self.visit_children(e)
    }

    fn visit_children(&mut self, e: &mut Expr) -> R {
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
                let chs = subs.iter_mut().map(|x| self.visit(x)).collect();
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
                let chs = subs.iter_mut().map(|x| self.visit(x)).collect();
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
                let mut chs1: Vec<R> = arms.iter_mut().map(|x| self.visit(&mut x.body)).collect();
                chs1.push(self.visit(body));
                self.join_results(chs1)
            }
            _ => self.default(),
        }
    }

    fn visit(&mut self, e: &mut Expr) -> R {
        use Expr::*;
        match e {
            IntLit { .. } => self.visit_intlit(e),
            UnitLit { .. } => self.visit_unitlit(e),
            Binary { .. } => self.visit_binary(e),
            Unary { .. } => self.visit_unary(e),
            VarRef { .. } => self.visit_varref(e),
            Builtin { .. } => self.visit_builtin(e),
            App { .. } => self.visit_app(e),
            Seq { .. } => self.visit_seq(e),
            Abs { .. } => self.visit_abs(e),
            Let { .. } => self.visit_let(e),
            Tuple { .. } => self.visit_tuple(e),
            Nth { .. } => self.visit_nth(e),
            Ite { .. } => self.visit_ite(e),
            LetRec { .. } => self.visit_letrec(e),
            _ => self.default(),
        }
    }
}
