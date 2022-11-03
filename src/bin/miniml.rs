use clap::Parser;
use std::{
    fs::{self, File},
    io::{stdout, Write},
    path::PathBuf,
};

extern crate tut;

use tut::{
    debrujin::DeBrujin,
    namer::Namer,
    parser::parse,
    pass::{ExprListener, ExprTransformer},
    secd::secdgen::secdgen,
};

#[derive(Debug, Clone, clap::ValueEnum)]
enum Stage {
    Parse,
    SECD,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    infile: PathBuf,

    #[arg(short, long)]
    outfile: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value = "secd")]
    stage: Stage,
}

fn main() {
    let cli = Cli::parse();

    let buf = fs::read_to_string(cli.infile).unwrap();

    let mut prog = parse(&buf).unwrap();
    if let Stage::Parse = cli.stage {
        println!("{:#?}", prog);
    }

    let mut os: Box<dyn Write> = match cli.outfile {
        Some(outfile) => Box::new(File::create(outfile).unwrap()),
        None => Box::new(stdout()),
    };

    let mut namer = Namer::new();
    namer.visit(&mut prog.main_expr).unwrap();

    let mut db = DeBrujin::new();
    db.walk(&mut prog.main_expr);
    let debrujin_info = db.get_info();
    let secd_instrs = secdgen(debrujin_info, &prog.main_expr);
    if let Stage::SECD = cli.stage {
        writeln!(os, "{}", secd_instrs).unwrap();
    }
}
