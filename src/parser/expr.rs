//! Parsing of expressions.
use crate::ast::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use super::{ops::*, types::*};

pub fn builtin(i: &str) -> IResult<&str, Expr> {
    // XXX
    let (i, op) = ws(alt((tag("println"), tag("println"))))(i)?;
    let op = match op {
        "println" => BuiltinOp::Println,
        _ => unreachable!(),
    };
    let o = Expr::Builtin { op };
    Ok((i, o))
}

pub fn unitlit(i: &str) -> IResult<&str, Expr> {
    let (i, _) = pair(wstag("("), wstag(")"))(i)?;
    let o = Expr::UnitLit {};
    Ok((i, o))
}

pub fn tuplee(i: &str) -> IResult<&str, Expr> {
    let (i, o) = delimited(wstag("("), separated_list1(wstag(","), expr), wstag(")"))(i)?;
    let subs = o.into_iter().map(Box::new).collect();
    let o = Expr::Tuple { subs };
    Ok((i, o))
}

pub fn nth(i: &str) -> IResult<&str, Expr> {
    // Making nth a builtin requires some kind of dependent unification
    // So for now it's a separate primitive
    let (i, (idx, sub)) = preceded(wstag("nth"), tuple((integer, expr)))(i)?;
    let sub = Box::new(sub);
    let o = Expr::Nth { idx, sub };
    Ok((i, o))
}

pub fn atom(i: &str) -> IResult<&str, Expr> {
    alt((
        unitlit,
        intlit,
        builtin,
        map(ident, |id| Expr::VarRef { id }),
        paren,
        nth,
        tuplee,
    ))(i)
}

pub fn app(i: &str) -> IResult<&str, Expr> {
    let (i, head) = atom(i)?;
    let (i, tail) = many0(atom)(i)?;
    let o = tail.into_iter().fold(head, |acc, arg| {
        let fun = Box::new(acc);
        let arg = Box::new(arg);
        Expr::App { fun, arg }
    });
    Ok((i, o))
}

pub fn una(i: &str) -> IResult<&str, Expr> {
    let (i, ops) = many0(una_op)(i)?;
    let (i, expr) = app(i)?;
    let o = ops.into_iter().rfold(expr, |acc, op| {
        let sub = Box::new(acc);
        Expr::Unary { op, sub }
    });
    Ok((i, o))
}

pub fn mul(i: &str) -> IResult<&str, Expr> {
    let (i, head) = una(i)?;
    let (i, tail) = many0(|i| {
        let (i, (op, expr)) = tuple((ws(mul_op), ws(una)))(i)?;
        Ok((i, (op, expr)))
    })(i)?;
    let o = tail.into_iter().fold(head, |acc, (op, expr)| {
        let lhs = Box::new(acc);
        let rhs = Box::new(expr);
        Expr::Binary { lhs, op, rhs }
    });
    Ok((i, o))
}

pub fn add(i: &str) -> IResult<&str, Expr> {
    let (i, head) = mul(i)?;
    let (i, tail) = many0(|i| {
        let (i, (op, expr)) = tuple((ws(add_op), ws(mul)))(i)?;
        Ok((i, (op, expr)))
    })(i)?;
    let o = tail.into_iter().fold(head, |acc, (op, expr)| {
        let lhs = Box::new(acc);
        let rhs = Box::new(expr);
        Expr::Binary { lhs, op, rhs }
    });
    Ok((i, o))
}

pub fn rel(i: &str) -> IResult<&str, Expr> {
    let (i, head) = add(i)?;
    let (i, tail) = many0(|i| {
        let (i, (op, expr)) = tuple((ws(rel_op), ws(add)))(i)?;
        Ok((i, (op, expr)))
    })(i)?;
    let o = tail.into_iter().fold(head, |acc, (op, expr)| {
        let lhs = Box::new(acc);
        let rhs = Box::new(expr);
        Expr::Binary { lhs, op, rhs }
    });
    Ok((i, o))
}

pub fn eq_op(i: &str) -> IResult<&str, BinOp> {
    map(alt((tag("=="), tag("!="))), |o| match o {
        "==" => BinOp::Eq,
        "!=" => BinOp::Ne,
        _ => unreachable!(),
    })(i)
}

pub fn eq(i: &str) -> IResult<&str, Expr> {
    let (i, head) = rel(i)?;
    let (i, tail) = many0(|i| {
        let (i, (op, expr)) = tuple((ws(eq_op), ws(rel)))(i)?;
        Ok((i, (op, expr)))
    })(i)?;
    let o = tail.into_iter().fold(head, |acc, (op, expr)| {
        let lhs = Box::new(acc);
        let rhs = Box::new(expr);
        Expr::Binary { lhs, op, rhs }
    });
    Ok((i, o))
}

pub fn ite1(i: &str) -> IResult<&str, Expr> {
    let (i, _) = wstag("if")(i)?;
    let (i, cond) = eq(i)?;
    let (i, _) = wstag("then")(i)?;
    let (i, tr) = eq(i)?;
    let (i, _) = wstag("else")(i)?;
    let (i, fl) = ite(i)?;
    let cond = Box::new(cond);
    let tr = Box::new(tr);
    let fl = Box::new(fl);
    let o = Expr::Ite { cond, tr, fl };
    Ok((i, o))
}

pub fn ite(i: &str) -> IResult<&str, Expr> {
    alt((ite1, eq))(i)
}

pub fn seq(i: &str) -> IResult<&str, Expr> {
    let (i, o) = separated_list1(wstag(";"), ite)(i)?;
    if o.len() == 1 {
        // prevent redundant seq's
        let o = o.into_iter().nth(0).unwrap();
        Ok((i, o))
    } else {
        let subs = o.into_iter().map(|x| Box::new(x)).collect();
        let o = Expr::Seq { subs };
        Ok((i, o))
    }
}

pub fn lam1(i: &str) -> IResult<&str, Expr> {
    let (i, _) = wstag(r"\")(i)?;
    let (i, arg_name) = ws(ident)(i)?;
    let (i, arg_ty) = opt(preceded(tag(":"), ty))(i)?;
    let (i, _) = wstag("->")(i)?;
    let (i, body) = ws(expr)(i)?;
    let arg_name = arg_name.to_string();
    let arg_ty = arg_ty.unwrap_or(Ty::UnkTy);
    let body = Box::new(body);
    let o = Expr::Abs {
        arg_name,
        arg_ty,
        body,
    };
    Ok((i, o))
}

pub fn lam(i: &str) -> IResult<&str, Expr> {
    alt((lam1, seq))(i)
}

pub fn let1(i: &str) -> IResult<&str, Expr> {
    let (i, _) = wstag("let")(i)?;
    let (i, name) = ws(ident)(i)?;
    let (i, ty) = opt(preceded(wstag(":"), ty))(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, val) = ws(expr)(i)?;
    let (i, _) = wstag("in")(i)?;
    let (i, body) = ws(expr)(i)?;
    let ty = ty.unwrap_or(Ty::UnkTy);
    let val = Box::new(val);
    let body = Box::new(body);
    let o = Expr::Let {
        name,
        ty,
        val,
        body,
    };
    Ok((i, o))
}

pub fn let2arm(i: &str) -> IResult<&str, LetRecArm> {
    let (i, fn_name) = ident(i)?;
    let (i, fn_ty) = opt(preceded(wstag(":"), ty))(i)?;
    let (i, _) = wstag("=")(i)?;
    let (i, _) = wstag(r"\")(i)?;
    let (i, arg_name) = ident(i)?;
    let (i, arg_ty) = opt(preceded(wstag(":"), ty))(i)?;
    let (i, _) = wstag("->")(i)?;
    let (i, body) = expr(i)?;
    let arg_ty = arg_ty.unwrap_or(Ty::UnkTy);
    let fn_ty = fn_ty.unwrap_or(Ty::UnkTy);
    let o = LetRecArm {
        fn_name,
        fn_ty,
        arg_name,
        arg_ty,
        body: Box::new(body),
    };
    Ok((i, o))
}

pub fn let2(i: &str) -> IResult<&str, Expr> {
    let (i, _) = wstag("let rec")(i)?;
    let (i, arms) = separated_list1(wstag("and"), let2arm)(i)?;
    let (i, _) = wstag("in")(i)?;
    let (i, body) = ws(expr)(i)?;
    let body = Box::new(body);
    let o = Expr::LetRec { arms, body };
    Ok((i, o))
}

pub fn lett(i: &str) -> IResult<&str, Expr> {
    alt((let1, let2, lam))(i)
}

pub fn expr(i: &str) -> IResult<&str, Expr> {
    lett(i)
}
