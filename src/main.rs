use std::io::Read;

extern crate tut;
use tut::parser::parse_top;

fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .expect("cannot read from stdin");

    parse_top(&buf);
}
