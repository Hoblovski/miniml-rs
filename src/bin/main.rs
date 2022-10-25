use std::io::Read;

extern crate tut;

use tut::parser::parse;

fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .expect("cannot read from stdin");

    let prog = parse(&buf).unwrap();
    println!("{:#?}", prog);
    // let mut namer = Namer::new();
    // namer.visit(&mut prog.main_expr).unwrap();
    // println!("{:#?}", prog)
}
