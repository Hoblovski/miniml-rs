use std::io::Read;

extern crate tut;
use tut::namer::*;
use tut::parser::parse;
use tut::visitor::*;

fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .expect("cannot read from stdin");

    let mut prog = parse(&buf).unwrap();
    let mut namer = Namer::new();
    namer.visit(&mut prog.main_expr).unwrap();
    println!("{:#?}", prog)
}
