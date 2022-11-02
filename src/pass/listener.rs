//! Listener that walks the tree itself. Functionalities are implemented in hook function.
use crate::ast::*;

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

    fn walk_binary(&mut self, lhs: &Box<Expr>, op: &BinOp, rhs: &Box<Expr>, eself: &Expr) {
        self.enter_binary(lhs, op, rhs, eself);
        self.walk(lhs);
        self.walk(rhs);
        self.exit_binary(lhs, op, rhs, eself);
    }

    fn walk_unary(&mut self, op: &UnaOp, sub: &Box<Expr>, eself: &Expr) {
        self.enter_unary(op, sub, eself);
        self.walk(sub);
        self.exit_unary(op, sub, eself);
    }

    fn walk_app(&mut self, fun: &Box<Expr>, arg: &Box<Expr>, eself: &Expr) {
        self.enter_app(fun, arg, eself);
        self.walk(fun);
        self.walk(arg);
        self.exit_app(fun, arg, eself);
    }

    fn walk_seq(&mut self, subs: &Vec<Box<Expr>>, eself: &Expr) {
        self.enter_seq(subs, eself);
        subs.iter().for_each(|x| self.walk(x));
        self.exit_seq(subs, eself);
    }

    fn walk_abs(&mut self, arg_name: &String, arg_ty: &Ty, body: &Expr, eself: &Expr) {
        self.enter_abs(arg_name, arg_ty, body, eself);
        self.walk(body);
        self.exit_abs(arg_name, arg_ty, body, eself);
    }

    fn walk_let(
        &mut self,
        name: &String,
        ty: &Ty,
        val: &Box<Expr>,
        body: &Box<Expr>,
        eself: &Expr,
    ) {
        self.enter_let(name, ty, val, body, eself);
        self.walk(val);
        self.walk(body);
        self.exit_let(name, ty, val, body, eself);
    }

    fn walk_tuple(&mut self, subs: &Vec<Box<Expr>>, eself: &Expr) {
        self.enter_tuple(subs, eself);
        subs.iter().for_each(|x| self.walk(x));
        self.exit_tuple(subs, eself);
    }

    fn walk_nth(&mut self, idx: &i64, sub: &Box<Expr>, eself: &Expr) {
        self.enter_nth(idx, sub, eself);
        self.walk(sub);
        self.exit_nth(idx, sub, eself);
    }

    fn walk_ite(&mut self, cond: &Box<Expr>, tr: &Box<Expr>, fl: &Box<Expr>, eself: &Expr) {
        self.enter_ite(cond, tr, fl, eself);
        self.walk(cond);
        self.walk(tr);
        self.walk(fl);
        self.exit_ite(cond, tr, fl, eself);
    }

    fn walk_letrec(&mut self, arms: &Vec<LetRecArm>, body: &Box<Expr>, eself: &Expr) {
        self.enter_letrec(arms, body, eself);
        for arm in arms.iter() {
            self.enter_letrecarm(arm);
            self.walk(&arm.body);
            self.exit_letrecarm(arm);
        }
        self.walk(body);
        self.exit_letrec(arms, body, eself);
    }

    fn walk_match(&mut self, sub: &Box<Expr>, arms: &Vec<MatchArm>, eself: &Expr) {
        self.enter_match(sub, arms, eself);
        self.walk(sub);
        for arm in arms.iter() {
            self.enter_matcharm(arm);
            self.walk(&arm.res);
            self.exit_matcharm(arm);
        }
        self.exit_match(sub, arms, eself);
    }

    fn default_walk(&mut self, e: &Expr) {
        use Expr::*;
        match e {
            IntLit { val } => self.walk_intlit(val, e),
            UnitLit {} => self.walk_unitlit(e),
            Binary { lhs, op, rhs } => self.walk_binary(lhs, op, rhs, e),
            Unary { op, sub } => self.walk_unary(op, sub, e),
            VarRef { id } => self.walk_varref(id, e),
            Builtin { op } => self.walk_builtin(op, e),
            App { fun, arg } => self.walk_app(fun, arg, e),
            Seq { subs } => self.walk_seq(subs, e),
            Abs {
                arg_name,
                arg_ty,
                body,
            } => self.walk_abs(arg_name, arg_ty, body, e),
            Let {
                name,
                ty,
                val,
                body,
            } => self.walk_let(name, ty, val, body, e),
            Tuple { subs } => self.walk_tuple(subs, e),
            Nth { idx, sub } => self.walk_nth(idx, sub, e),
            Ite { cond, tr, fl } => self.walk_ite(cond, tr, fl, e),
            LetRec { arms, body } => self.walk_letrec(arms, body, e),
            Match { sub, arms } => self.walk_match(sub, arms, e),
        }
    }

    fn walk(&mut self, e: &Expr) {
        self.default_walk(e)
    }
}
