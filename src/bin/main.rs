use std::io::Read;

extern crate tut;

use tut::{debrujin::DeBrujin, inspector::Inspector, parser::parse, visitor::ExprListener};

fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .expect("cannot read from stdin");

    let mut prog = parse(&buf).unwrap();
    println!("{:#?}", prog);
    println!("above: parse ==== \n\n\n\n\n\n\n\n");
    // let mut namer = Namer::new();
    // namer.visit(&mut prog.main_expr).unwrap();
    // println!("{:#?}", prog)
    let mut db = DeBrujin::new();
    db.walk(&mut prog.main_expr);

    let mut insp = Inspector::new(db.get_info());
    insp.walk(&prog.main_expr);
}
