//! Operators, delimiters, keywords etc terminal parsing.
use std::str::FromStr;

use crate::ast::*;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::*,
    combinator::{map, map_res, opt, recognize, value, verify},
    error::ParseError,
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};
use phf::phf_set;

use super::expr::expr;

static KEYWORDS: phf::Set<&'static str> = phf_set! {
    "in",
    "let",
    "if",
    "else",
    "then",
    "rec",
    "and",
    "match",
    "end",

    "datatype",
    "int",
    "bool",
    "unit",
};

pub fn paren(i: &str) -> IResult<&str, Expr> {
    delimited(tag("("), ws(expr), tag(")"))(i)
}

pub fn integer(i: &str) -> IResult<&str, i64> {
    map_res(ws(digit1), FromStr::from_str)(i)
}

pub fn intlit(i: &str) -> IResult<&str, Expr> {
    map(integer, |val| Expr::IntLit { val })(i)
}

pub fn is_keyword(i: &str) -> bool {
    KEYWORDS.contains(i)
}

/// Simply a regex like ident, but does not filter out keywords.
pub fn identlike(i: &str) -> IResult<&str, &str> {
    ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    )))(i)
}

/// Identifier i.e. satisfies the regex and is not a keyword.
pub fn ident(i: &str) -> IResult<&str, String> {
    let (i, s) = verify(identlike, |s: &str| !is_keyword(s))(i)?;
    Ok((i, String::from(s)))
}

pub fn una_op(i: &str) -> IResult<&str, UnaOp> {
    map(alt((tag("!"), tag("-"))), |o| match o {
        "!" => UnaOp::Lnot,
        "-" => UnaOp::Neg,
        _ => unreachable!(),
    })(i)
}

pub fn mul_op(i: &str) -> IResult<&str, BinOp> {
    map(alt((tag("*"), tag("/"), tag("%"))), |o| match o {
        "*" => BinOp::Mul,
        "/" => BinOp::Div,
        "%" => BinOp::Rem,
        _ => unreachable!(),
    })(i)
}

pub fn add_op(i: &str) -> IResult<&str, BinOp> {
    map(alt((tag("+"), tag("-"))), |o| match o {
        "+" => BinOp::Add,
        "-" => BinOp::Sub,
        _ => unreachable!(),
    })(i)
}

pub fn rel_op(i: &str) -> IResult<&str, BinOp> {
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

pub fn eol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        pair(tag("--"), is_not("\n\r")),
    )(i)
}

pub fn ignored<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    let (i, _) = opt(eol_comment)(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, ()))
}

// From recipe
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(ignored, inner, ignored)
}

pub fn wstag<'a, E: ParseError<&'a str>>(
    s: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> {
    delimited(ignored, tag(s), ignored)
}
