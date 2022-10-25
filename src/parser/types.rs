//! Parsing of types
use super::ops::*;
use crate::ast::*;

use nom::{branch::alt, combinator::map_opt, multi::separated_list1, sequence::delimited, IResult};

pub fn ty_base(i: &str) -> IResult<&str, Ty> {
    map_opt(identlike, |s: &str| match s {
        "bool" => Some(Ty::BoolTy),
        "int" => Some(Ty::IntTy),
        "unit" => Some(Ty::UnitTy),
        _ => None,
    })(i)
}

pub fn ty_paren(i: &str) -> IResult<&str, Ty> {
    delimited(wstag("("), ty, wstag(")"))(i)
}

pub fn ty_data_type(i: &str) -> IResult<&str, Ty> {
    let (i, name) = ident(i)?;
    let o = Ty::DataTy(name);
    Ok((i, o))
}

pub fn ty_atom(i: &str) -> IResult<&str, Ty> {
    alt((ty_paren, ty_base, ty_data_type))(i)
}

pub fn ty_lam(i: &str) -> IResult<&str, Ty> {
    let (i, o) = separated_list1(wstag("->"), ty_atom)(i)?;
    if o.len() == 1 {
        // prevent redundant lam's
        let o = o.into_iter().nth(0).unwrap();
        Ok((i, o))
    } else {
        // rev because -> is r-assoc
        let o = o
            .into_iter()
            .rev()
            .reduce(|rhs, lhs| Ty::AbsTy(Box::new(lhs), Box::new(rhs)))
            .unwrap();
        Ok((i, o))
    }
}

pub fn ty(i: &str) -> IResult<&str, Ty> {
    ty_lam(i)
}
