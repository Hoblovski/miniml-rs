/// Provides basic visitors, listeners.
use crate::ast::*;

/// The visitor itself is mut so we are able to store bookkeeping info.
/// The visited node is mut so we can mutate the node.
/// Argument to specific visit functions is an entire expr,
///     so you may alter the node like `*e = IntLit()`
/// R is the return type for visitor functions.
#[allow(unused_variables)]
pub trait ExprVisitorMut<R> {
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

#[allow(unused_variables)]
/// Listeners just walk the AST. All info are to be kept in `&mut self`.
pub trait ExprListener {
    fn walk_intlit(&mut self, val: &i64, eself: &Expr) {}

    fn walk_unitlit(&mut self, eself: &Expr) {}

    fn enter_binary(&mut self, lhs: &Expr, op: &BinOp, rhs: &Expr, eself: &Expr) {}
    fn exit_binary(&mut self, lhs: &Expr, op: &BinOp, rhs: &Expr, eself: &Expr) {}

    fn enter_unary(&mut self, op: &UnaOp, sub: &Expr, eself: &Expr) {}
    fn exit_unary(&mut self, op: &UnaOp, sub: &Expr, eself: &Expr) {}

    fn walk_varref(&mut self, id: &String, eself: &Expr) {}

    fn walk_builtin(&mut self, op: &BuiltinOp, eself: &Expr) {}

    fn enter_app(&mut self, fun: &Expr, arg: &Expr, eself: &Expr) {}
    fn exit_app(&mut self, fun: &Expr, arg: &Expr, eself: &Expr) {}

    fn enter_seq(&mut self, subs: &Vec<Box<Expr>>, eself: &Expr) {}
    fn exit_seq(&mut self, subs: &Vec<Box<Expr>>, eself: &Expr) {}

    fn enter_abs(&mut self, arg_name: &String, arg_ty: &Ty, body: &Expr, eself: &Expr) {}
    fn exit_abs(&mut self, arg_name: &String, arg_ty: &Ty, body: &Expr, eself: &Expr) {}

    fn enter_let(&mut self, name: &String, ty: &Ty, val: &Expr, body: &Expr, eself: &Expr) {}
    fn exit_let(&mut self, name: &String, ty: &Ty, val: &Expr, body: &Expr, eself: &Expr) {}

    fn enter_tuple(&mut self, subs: &Vec<Box<Expr>>, eself: &Expr) {}
    fn exit_tuple(&mut self, subs: &Vec<Box<Expr>>, eself: &Expr) {}

    fn enter_nth(&mut self, idx: &i64, sub: &Expr, eself: &Expr) {}
    fn exit_nth(&mut self, idx: &i64, sub: &Expr, eself: &Expr) {}

    fn enter_ite(&mut self, cond: &Expr, tr: &Expr, fl: &Expr, eself: &Expr) {}
    fn exit_ite(&mut self, cond: &Expr, tr: &Expr, fl: &Expr, eself: &Expr) {}

    fn enter_letrec(&mut self, arms: &Vec<LetRecArm>, body: &Expr, eself: &Expr) {}
    fn exit_letrec(&mut self, arms: &Vec<LetRecArm>, body: &Expr, eself: &Expr) {}

    fn enter_match(&mut self, sub: &Expr, arms: &Vec<MatchArm>, eself: &Expr) {}
    fn exit_match(&mut self, sub: &Expr, arms: &Vec<MatchArm>, eself: &Expr) {}

    fn enter_letrecarm(&mut self, arm: &LetRecArm) {}
    fn exit_letrecarm(&mut self, arm: &LetRecArm) {}

    fn enter_matcharm(&mut self, arm: &MatchArm) {}
    fn exit_matcharm(&mut self, arm: &MatchArm) {}

    fn default_walk(&mut self, e: &Expr) {
        use Expr::*;
        match e {
            IntLit { val } => self.walk_intlit(val, e),
            UnitLit {} => self.walk_unitlit(e),
            Binary { lhs, op, rhs } => {
                self.enter_binary(lhs, op, rhs, e);
                self.walk(lhs);
                self.walk(rhs);
                self.exit_binary(lhs, op, rhs, e);
            }
            Unary { op, sub } => {
                self.enter_unary(op, sub, e);
                self.walk(sub);
                self.exit_unary(op, sub, e);
            }
            VarRef { id } => self.walk_varref(id, e),
            Builtin { op } => self.walk_builtin(op, e),
            App { fun, arg } => {
                self.enter_app(fun, arg, e);
                self.walk(fun);
                self.walk(arg);
                self.exit_app(fun, arg, e);
            }
            Seq { subs } => {
                self.enter_seq(subs, e);
                subs.iter().for_each(|x| self.walk(x));
                self.exit_seq(subs, e);
            }
            Abs {
                arg_name,
                arg_ty,
                body,
            } => {
                self.enter_abs(arg_name, arg_ty, body, e);
                self.walk(body);
                self.exit_abs(arg_name, arg_ty, body, e);
            }
            Let {
                name,
                ty,
                val,
                body,
            } => {
                self.enter_let(name, ty, val, body, e);
                self.walk(val);
                self.walk(body);
                self.exit_let(name, ty, val, body, e);
            }
            Tuple { subs } => {
                self.enter_tuple(subs, e);
                subs.iter().for_each(|x| self.walk(x));
                self.exit_tuple(subs, e);
            }
            Nth { idx, sub } => {
                self.enter_nth(idx, sub, e);
                self.walk(sub);
                self.exit_nth(idx, sub, e);
            }
            Ite { cond, tr, fl } => {
                self.enter_ite(cond, tr, fl, e);
                self.walk(cond);
                self.walk(tr);
                self.walk(fl);
                self.exit_ite(cond, tr, fl, e);
            }
            LetRec { arms, body } => {
                self.enter_letrec(arms, body, e);
                for arm in arms.iter() {
                    self.enter_letrecarm(arm);
                    self.walk(&arm.body);
                    self.exit_letrecarm(arm);
                }
                self.walk(body);
                self.exit_letrec(arms, body, e);
            }
            Match { sub, arms } => {
                self.enter_match(sub, arms, e);
                self.walk(sub);
                for arm in arms.iter() {
                    self.enter_matcharm(arm);
                    self.walk(&arm.res);
                    self.exit_matcharm(arm);
                }
                self.exit_match(sub, arms, e);
            }
        }
    }

    fn walk(&mut self, e: &Expr) {
        self.default_walk(e)
    }
}

#[allow(unused_variables)]
/// Does auto un-packing.
pub trait ExprVisitorImmut<R> {
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
