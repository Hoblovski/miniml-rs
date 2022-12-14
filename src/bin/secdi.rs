use clap::Parser;
use tut::secd::langdef::SECDVal;
use tut::secd::repr::secd_parse;

use std::cmp::{max, min};
use std::fmt::Write;
use std::path::PathBuf;
use std::process::exit;
use std::{
    fs,
    io::{stdin, Read},
};

use tut::secd::machine::{SECDMachine, SECDState, SECDStepResult};

extern crate tut;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    infile: Option<PathBuf>,

    #[arg(short, long)]
    interactive: bool,

    #[arg(short, long)]
    brief: bool,

    #[arg(short, long)]
    maxstep: Option<usize>,
}

pub fn clrscr() {
    println!("\x1b[H\x1b[J")
}

struct SECDInterp<'s> {
    machine: SECDMachine,
    nsteps: usize,
    lines: Vec<&'s str>,
    dumps: Vec<String>,
}

/// When dumping, these limits the portion of code being printed.
const BEFORE_MAX: usize = 10;

/// When dumping, these limits the portion of code being printed.
const AFTER_MAX: usize = 15;

impl<'s> SECDInterp<'s> {
    fn new(code: &'s str) -> Self {
        let lines = code
            .lines()
            .filter(|line| line.trim().len() != 0 && !line.trim().starts_with("#"))
            .collect::<Vec<_>>();
        let lines_parse = lines
            .iter()
            .map(|x| x.trim().to_string())
            .collect::<Vec<_>>();
        let machine = secd_parse(&lines_parse);
        let mut res = Self {
            machine: SECDMachine::init(machine),
            nsteps: 0,
            lines,
            dumps: Vec::new(),
        };
        res.dumps.push(res.dump());
        res
    }

    pub fn dump(&self) -> String {
        let mut s = String::new();
        writeln!(s, "--- Step: {}\n", self.nsteps).unwrap();
        let machine = &self.machine;
        let SECDState(pc, stk, env) = &machine.state;
        writeln!(s, "--- pc {}", pc).unwrap();
        let low = max(*pc as isize - BEFORE_MAX as isize, 0) as usize;
        let high = min(pc + AFTER_MAX, self.lines.len());
        for (i, instr) in self.lines[low..high].iter().enumerate() {
            let j = i + low;
            if j == *pc {
                writeln!(s, ">   {j:<4}: {instr}").unwrap();
            } else {
                writeln!(s, "    {j:<4}: {instr}").unwrap();
            }
        }
        writeln!(s, "\n").unwrap();
        writeln!(s, "--- stk (from bottom to top)").unwrap();
        for v in stk {
            writeln!(s, "{:?}", v).unwrap();
        }
        writeln!(s, "\n").unwrap();
        writeln!(s, "--- env (from bottom to top)").unwrap();
        for v in env {
            writeln!(s, "{:?}", v).unwrap();
        }
        writeln!(s, "\n").unwrap();
        writeln!(s, "\n--- effect").unwrap();
        for v in &machine.effects {
            writeln!(s, "{:?}", v).unwrap();
        }
        s
    }

    fn dump_brief(&self) -> String {
        let mut res = String::new();
        for v in &self.machine.effects {
            match v {
                tut::secd::machine::SECDEffect::Println(s) => writeln!(res, "{s}").unwrap(),
            }
        }
        let SECDState(_pc, stk, _env) = &self.machine.state;
        assert!(stk.len() == 1);
        match stk.get(0).unwrap() {
            SECDVal::UnitVal => writeln!(res, "()"),
            SECDVal::IntVal(v) => writeln!(res, "{v}"),
            _ => unreachable!(),
        }
        .unwrap();
        res
    }

    fn step(&mut self) -> SECDStepResult {
        self.nsteps += 1;
        let res = self.machine.step();
        self.dumps.push(self.dump());
        res
    }
}

fn main() {
    let cli = Cli::parse();

    let buf = match cli.infile {
        Some(infile) => fs::read_to_string(infile).unwrap(),
        None => {
            if cli.interactive {
                eprintln!("When interactive is set, input must not be from stdin.");
                exit(1);
            }
            let mut buf = String::new();
            stdin().read_to_string(&mut buf).unwrap();
            buf
        }
    };

    let mut interp = SECDInterp::new(buf.as_str());

    if cli.interactive {
        let mut display_step = 0;
        let mut last_op = "n".to_string();
        loop {
            clrscr();
            println!("{}", interp.dumps[display_step]);
            let mut cmd = String::new();
            stdin().read_line(&mut cmd).unwrap();
            let mut t = cmd.trim().split_whitespace();
            let default = last_op.clone();
            let op = t.next().unwrap_or(default.as_str());
            last_op = op.to_string();
            let _args: Vec<&str> = t.collect();
            match op {
                "q" => break,
                "p" => {
                    if display_step > 0 {
                        display_step -= 1;
                    }
                }
                "n" => {
                    display_step += 1;
                    if display_step >= interp.nsteps {
                        let stepres = interp.step();
                        if let Err(err) = stepres {
                            println!("Execution terminated with error: {err}");
                            break;
                        }
                    }
                }
                _ => {
                    eprintln!("bad op!");
                }
            }
        }
    } else {
        let maxstep = cli.maxstep.unwrap_or(usize::MAX);
        let res = loop {
            let stepres = interp.step();
            if interp.nsteps > maxstep {
                println!("Reached maxstep {maxstep}. ABORTED.");
                break "ABORTED".to_string();
            }
            if let Err(res) = stepres {
                break res;
            }
        };
        if cli.brief {
            println!("{}", interp.dump_brief());
        } else {
            println!("Execution result: {}\n", res);
            println!("--- terminal state:\n{}", interp.dumps[interp.nsteps - 1]);
        }
    }
}
