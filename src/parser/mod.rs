use crate::ast::*;
use crate::error::*;
use crate::parser::ops::ws;
use nom::{combinator::eof, sequence::terminated};
use top::top;

mod expr;
mod ops;
mod top;
mod types;

pub fn parse(buf: &str) -> Result<Prog, MiniMLErr> {
    let parse_res = terminated(ws(top), eof)(buf);
    let prog = parse_res.expect("parsing failed").1;
    println!("Prog is {:#?}", prog);
    println!("===============================");
    Ok(prog)
}
