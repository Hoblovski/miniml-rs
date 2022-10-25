/// Provides basic visitors.
use crate::ast::*;

// TODO: use proc macro
pub trait ExprVisitorMut<R> {
    /// The visitor itself is mut so we are able to store bookkeeping info.
    /// The visited node is mut so we can mutate the node.
    /// Argument to specific visit functions is an entire expr,
    ///     so you may alter the node like `*e = IntLit()`
    /// R is the return type for visitor functions.

    fn default(&mut self) -> R;

    fn join_results(&mut self, _res: Vec<R>) -> R {
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

// pub trait ExprVisitorImmut<R> {
//     /// Does auto un-packing.

//     fn default(&mut self) -> R;

//     fn join_results(&mut self, _res: Vec<R>) -> R {
//         self.default()
//     }

//     fn visit_intlit(&mut self, val: i64) -> R {
//         self.default()
//     }

//     fn visit_unitlit(&mut self) -> R {
//         self.default()
//     }

//     fn visit_binary(&mut self, lhs: &Expr, op: BinOp, rhs: &Expr) -> R {
//         self.default()
//     }

//     fn visit_unary(&mut self, op: UnaOp, sub: &Expr) -> R {
//         self.default()
//     }

//     fn visit_varref(&mut self, id: String) -> R {
//         self.default()
//     }

//     fn visit_builtin(&mut self, op: BuiltinOp) -> R {
//         self.default()
//     }

//     fn visit_app(&mut self, fun: &Expr, arg: &Expr) -> R {
//         self.default()
//     }

//     fn visit_seq(&mut self, subs: Vec<&Expr>) -> R {
//         self.default()
//     }

//     fn visit_abs(&mut self, arg_name: String, arg_ty: Ty, body: &Expr) -> R {
//         self.default()
//     }

//     fn visit_let(&mut self, name: String, ty: Ty, val: &Expr, body: &Expr) -> R {
//         self.default()
//     }

//     fn visit_tuple(&mut self, subs: Vec<&Expr>) -> R {
//         self.default()
//     }

//     fn visit_nth(&mut self, idx: i64, sub: &Expr) -> R {
//         self.default()
//     }

//     fn visit_ite(&mut self, cond: &Expr, tr: &Expr, fl: &Expr) -> R {
//         self.default()
//     }

//     fn visit_letrec(&mut self, arms: Vec<LetRecArm>, body: &Expr) -> R {
//         self.default()
//     }

//     fn visit_match(&mut self, sub: &Expr, arms: Vec<MatchArm>) -> R {
//         self.default()
//     }

//     fn visit(&mut self, e: &Expr) -> R {
//         use Expr::*;
//         match e {
//             IntLit { val } => self.visit_intlit(*val),
//             UnitLit { } => self.visit_unitlit(),
//             Binary { lhs, op, rhs } => self.visit_binary(lhs, *op, rhs),
//             Unary { op, sub } => self.visit_unary(*op, sub),
//             VarRef { id }  => self.visit_varref(*id),
//             Builtin { op } => self.visit_builtin(*op),
//             App { fun, arg } => self.visit_app(fun, arg),
//             Seq { subs } => self.visit_seq(subs.iter().collect()),
//             Abs { arg_name, arg_ty, body } => self.visit_abs(arg_name, arg_ty, body),
//             Let { name, ty, val, body } => self.visit_let(name, ty, val, body),
//             Tuple { subs } => self.visit_tuple(e),
//             Nth { idx, sub } => self.visit_nth(e),
//             Ite { cond, tr, fl } => self.visit_ite(e),
//             LetRec { arms, body } => self.visit_letrec(e),
//             _ => self.default(),
//         }
//     }
// }
