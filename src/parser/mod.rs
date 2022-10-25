use crate::ast::*;
use crate::error::*;
use expr::expr;
use nom::{combinator::eof, sequence::terminated};
use ops::ws;

mod expr;
/// Parse program to AST.
mod ops;
mod types;

///////////////////////////////////////////////////////////
/// Tops

pub fn parse(buf: &str) -> Result<Prog, MiniMLErr> {
    let (_, main_expr) = terminated(ws(expr), eof)(buf).unwrap();
    let prog = Prog { main_expr };
    println!("Prog is {:#?}", prog);
    println!("===============================");
    Ok(prog)
}
