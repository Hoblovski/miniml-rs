//! Visitor that may mutate the AST.
use crate::ast::*;

/// The visitor itself is mut so we are able to store bookkeeping info.
/// The visited node is mut so we can mutate the node.
/// Argument to specific visit functions is an entire expr,
///     so you may alter the node like `*e = IntLit()`
/// R is the return type for visitor functions.
#[allow(unused_variables)]
pub trait ExprTransformer<R> {
    fn default(&mut self) -> R;

    fn join_results(&mut self, res: Vec<R>) -> R {
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

    fn visit_letrecarm(&mut self, e: &mut LetRecArm) -> R {
        self.visit(&mut e.body)
    }

    fn visit_matcharm(&mut self, e: &mut MatchArm) -> R {
        self.visit(&mut e.res)
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
                let mut chs: Vec<R> = arms.iter_mut().map(|x| self.visit_letrecarm(x)).collect();
                chs.push(self.visit(body));
                self.join_results(chs)
            }
            Match { sub, arms } => {
                let mut chs = vec![self.visit(sub)];
                chs.extend(arms.iter_mut().map(|x| self.visit_matcharm(x)));
                self.join_results(chs)
            }
            _ => self.default(),
        }
    }

    /// Impl can call default_visit even if it overrides visit.
    fn default_visit(&mut self, e: &mut Expr) -> R {
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
            _ => todo!(),
        }
    }

    fn visit(&mut self, e: &mut Expr) -> R {
        self.default_visit(e)
    }
}
