/// Top-level parsing
use crate::ast::*;

use nom::{
    multi::{many0, many1},
    IResult,
};

use super::{expr::*, ops::*, types::*};

pub fn top(i: &str) -> IResult<&str, Prog> {
    let (i, data_types) = many0(data_type)(i)?;
    let (i, main_expr) = expr(i)?;
    let o = Prog {
        data_types,
        main_expr,
    };
    Ok((i, o))
}

pub fn data_type_arm(i: &str) -> IResult<&str, DataTypeArm> {
    let (i, _) = wstag("|")(i)?;
    let (i, ctor) = ident(i)?;
    let (i, arg_tys) = many1(ty)(i)?;
    let o = DataTypeArm { ctor, arg_tys };
    Ok((i, o))
}

pub fn data_type(i: &str) -> IResult<&str, DataType> {
    let (i, _) = wstag("datatype")(i)?;
    let (i, name) = ident(i)?;
    let (i, _) = wstag("=")(i)?;
    let (i, arms) = many0(data_type_arm)(i)?;
    let (i, _) = wstag("end")(i)?;
    let o = DataType { name, arms };
    Ok((i, o))
}
