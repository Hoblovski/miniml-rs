use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::{map, map_res, opt, recognize},
    error::ParseError,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use std::str::FromStr;

#[derive(Debug)]
pub struct Top {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
    Land,
    Lor,
    Lxor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaOp {
    Neg,
    Lnot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuiltinOp {
    Println,
}

#[derive(Debug)]
pub enum Ty {
    UnitTy,
    IntTy,
    BoolTy,
    AbsTy(Box<Ty>, Box<Ty>),
}

#[derive(Debug)]
pub enum Expr {
    IntLit(i64),
    UnitLit,
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaOp, Box<Expr>),
    VarRef(String),
    Builtin(BuiltinOp),
    App(Box<Expr>, Box<Expr>),
    Seq(Vec<Box<Expr>>),
    Abs(String, Option<Ty>, Box<Expr>),
    Let(String, Option<Ty>, Box<Expr>, Box<Expr>),
}

fn paren(i: &str) -> IResult<&str, Expr> {
    delimited(tag("("), ws(expr), tag(")"))(i)
}

fn intlit(i: &str) -> IResult<&str, Expr> {
    map(
        map_res(
            delimited(multispace0, digit1, multispace0),
            FromStr::from_str,
        ),
        Expr::IntLit,
    )(i)
}

fn ident(i: &str) -> IResult<&str, String> {
    // NOTE: this does not exclude keywords!
    map(
        ws(recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))),
        String::from,
    )(i)
}

fn builtin(i: &str) -> IResult<&str, Expr> {
    // XXX
    let (i, op) = ws(alt((tag("println"), tag("println"))))(i)?;
    let o = Expr::Builtin(match op {
        "println" => BuiltinOp::Println,
        _ => unreachable!(),
    });
    Ok((i, o))
}

fn unitlit(i: &str) -> IResult<&str, Expr> {
    let (i, _) = pair(ws(tag("(")), ws(tag(")")))(i)?;
    let o = Expr::UnitLit;
    Ok((i, o))
}

fn atom(i: &str) -> IResult<&str, Expr> {
    alt((unitlit, intlit, builtin, map(ident, Expr::VarRef), paren))(i)
}

fn app(i: &str) -> IResult<&str, Expr> {
    let (i, head) = atom(i)?;
    let (i, tail) = many0(atom)(i)?;
    let o = tail
        .into_iter()
        .fold(head, |acc, arg| Expr::App(Box::new(acc), Box::new(arg)));
    Ok((i, o))
}

fn una_op(i: &str) -> IResult<&str, UnaOp> {
    map(alt((tag("!"), tag("-"))), |o| match o {
        "!" => UnaOp::Lnot,
        "-" => UnaOp::Neg,
        _ => unreachable!(),
    })(i)
}

fn una(i: &str) -> IResult<&str, Expr> {
    let (i, ops) = many0(una_op)(i)?;
    let (i, expr) = app(i)?;
    let o = ops
        .into_iter()
        .rfold(expr, |acc, op| Expr::Unary(op, Box::new(acc)));
    Ok((i, o))
}

fn mul_op(i: &str) -> IResult<&str, BinOp> {
    map(alt((tag("*"), tag("/"), tag("%"))), |o| match o {
        "*" => BinOp::Mul,
        "/" => BinOp::Div,
        "%" => BinOp::Rem,
        _ => unreachable!(),
    })(i)
}

fn mul(i: &str) -> IResult<&str, Expr> {
    let (i, head) = una(i)?;
    let (i, tail) = many0(|i| {
        let (i, (op, expr)) = tuple((ws(mul_op), ws(una)))(i)?;
        Ok((i, (op, expr)))
    })(i)?;
    let o = tail.into_iter().fold(head, |acc, (op, expr)| {
        Expr::Binary(Box::new(acc), op, Box::new(expr))
    });
    Ok((i, o))
}

fn add_op(i: &str) -> IResult<&str, BinOp> {
    map(alt((tag("+"), tag("-"))), |o| match o {
        "+" => BinOp::Add,
        "-" => BinOp::Sub,
        _ => unreachable!(),
    })(i)
}

fn add(i: &str) -> IResult<&str, Expr> {
    let (i, head) = mul(i)?;
    let (i, tail) = many0(|i| {
        let (i, (op, expr)) = tuple((ws(add_op), ws(mul)))(i)?;
        Ok((i, (op, expr)))
    })(i)?;
    let o = tail.into_iter().fold(head, |acc, (op, expr)| {
        Expr::Binary(Box::new(acc), op, Box::new(expr))
    });
    Ok((i, o))
}

fn rel_op(i: &str) -> IResult<&str, BinOp> {
    // NOTE: this alt order matters
    map(
        alt((tag(">="), tag("<="), tag(">"), tag("<"))),
        |o| match o {
            ">" => BinOp::Gt,
            "<" => BinOp::Lt,
            ">=" => BinOp::Ge,
            "<=" => BinOp::Le,
            _ => unreachable!(),
        },
    )(i)
}

fn rel(i: &str) -> IResult<&str, Expr> {
    let (i, head) = add(i)?;
    let (i, tail) = many0(|i| {
        let (i, (op, expr)) = tuple((ws(rel_op), ws(add)))(i)?;
        Ok((i, (op, expr)))
    })(i)?;
    let o = tail.into_iter().fold(head, |acc, (op, expr)| {
        Expr::Binary(Box::new(acc), op, Box::new(expr))
    });
    Ok((i, o))
}

fn eq_op(i: &str) -> IResult<&str, BinOp> {
    map(alt((tag("=="), tag("!="))), |o| match o {
        "==" => BinOp::Eq,
        "!=" => BinOp::Ne,
        _ => unreachable!(),
    })(i)
}

fn eq(i: &str) -> IResult<&str, Expr> {
    let (i, head) = rel(i)?;
    let (i, tail) = many0(|i| {
        let (i, (op, expr)) = tuple((ws(eq_op), ws(rel)))(i)?;
        Ok((i, (op, expr)))
    })(i)?;
    let o = tail.into_iter().fold(head, |acc, (op, expr)| {
        Expr::Binary(Box::new(acc), op, Box::new(expr))
    });
    Ok((i, o))
}

fn seq(i: &str) -> IResult<&str, Expr> {
    let (i, o) = separated_list1(tag(";"), eq)(i)?;
    if o.len() == 1 {
        // prevent redundant seq's
        let o = o.into_iter().nth(0).unwrap();
        Ok((i, o))
    } else {
        let o = Expr::Seq(o.into_iter().map(|x| Box::new(x)).collect());
        Ok((i, o))
    }
}

fn ty(i: &str) -> IResult<&str, Ty> {
    // TODO
    Ok((i, Ty::UnitTy))
}

fn lam1(i: &str) -> IResult<&str, Expr> {
    let (i, _) = ws(tag("\\"))(i)?;
    let (i, arg_name) = ws(ident)(i)?;
    let (i, arg_ty) = opt(preceded(ws(tag(":")), ty))(i)?;
    let (i, _) = ws(tag("->"))(i)?;
    let (i, body) = ws(expr)(i)?;
    let o = Expr::Abs(String::from(arg_name), arg_ty, Box::new(body));
    Ok((i, o))
}

fn lam(i: &str) -> IResult<&str, Expr> {
    alt((lam1, seq))(i)
}

fn let1(i: &str) -> IResult<&str, Expr> {
    let (i, _) = ws(tag("let"))(i)?;
    let (i, name) = ws(ident)(i)?;
    let (i, ty) = opt(preceded(ws(tag(":")), ty))(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, val) = ws(atom)(i)?;
    let (i, _) = ws(tag("in"))(i)?;
    let (i, body) = ws(expr)(i)?;
    let o = Expr::Let(name, ty, Box::new(val), Box::new(body));
    Ok((i, o))
}

fn lett(i: &str) -> IResult<&str, Expr> {
    alt((let1, lam))(i)
}

fn expr(i: &str) -> IResult<&str, Expr> {
    lett(i)
}

pub fn parse_top(buf: &str) {
    let r = expr(buf);
    println!("{:?}", r);
}

// From recipe
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_until_add_1() {
        let res1 = format!("{:?}", add("2 + 3 5 * 6"));
        let res2 = r#"Ok(("", Binary(IntLit(2), Add, Binary(App(IntLit(3), IntLit(5)), Mul, IntLit(6)))))"#;
        assert_eq!(res1, res2);
    }

    #[test]
    fn test_until_add_2() {
        let res1 = format!("{:?}", add("(2 + 3) * 4 5"));
        let res2 = r#"Ok(("", Binary(Binary(IntLit(2), Add, IntLit(3)), Mul, App(IntLit(4), IntLit(5)))))"#;
        assert_eq!(res1, res2);
    }

    #[test]
    fn test_until_eq_1() {
        let res1 = format!("{:?}", eq("2 * f x == 6 <= 3 % 3"));
        let res2 = r#"Ok(("", Binary(Binary(IntLit(2), Mul, App(VarRef("f"), VarRef("x"))), Eq, Binary(IntLit(6), Le, Binary(IntLit(3), Rem, IntLit(3))))))"#;
        assert_eq!(res1, res2);
    }

    #[test]
    fn test_until_seq_1() {
        let res1 = format!("{:?}", seq("2 ; 3 5 ; 5 == 6"));
        let res2 = r#"Ok(("", Seq([IntLit(2), App(IntLit(3), IntLit(5)), Binary(IntLit(5), Eq, IntLit(6))])))"#;
        assert_eq!(res1, res2);
    }

    #[test]
    fn test_until_let_1() {
        let res1 = format!("{:?}", lett(r"let f = (let x=2 in \y -> y+x) in f 5"));
        let res2 = r#"Ok(("", Let("f", None, Let("x", None, IntLit(2), Abs("y", None, Binary(VarRef("y"), Add, VarRef("x")))), App(VarRef("f"), IntLit(5)))))"#;
        assert_eq!(res1, res2);
    }
}
