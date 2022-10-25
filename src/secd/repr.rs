//! Parsing and printing of SECD values.

use phf::phf_map;

use super::langdef::{BinOp, BrOp, SECDInstr, SECDVal, UnaOp};

static BINOPS_PARSE: phf::Map<&'static str, BinOp> = {
    use BinOp::*;
    phf_map! {
        "add" => Add,
        "sub" => Sub,
        "mul" => Mul,
        "div" => Div,
        "rem" => Rem,
        "eq" => Eq,
        "ne" => Ne,
        "ge" => Ge,
        "le" => Le,
        "gt" => Gt,
        "lt" => Lt,
    }
};

static UNAOPS_PARSE: phf::Map<&'static str, UnaOp> = {
    use UnaOp::*;
    phf_map! {
        "neg" => Neg,
    }
};

static BROPS_PARSE: phf::Map<&'static str, BrOp> = {
    use BrOp::*;
    phf_map! {
        "br" => Br,
        "brfl" => BrFalse,
    }
};

fn binops_print(op: BinOp) -> &'static str {
    use BinOp::*;
    match op {
        Add => "add",
        Sub => "sub",
        Mul => "mul",
        Div => "div",
        Rem => "rem",
        Eq => "eq",
        Ne => "ne",
        Ge => "ge",
        Le => "le",
        Gt => "gt",
        Lt => "lt",
        _ => todo!(),
    }
}

fn unaops_print(op: UnaOp) -> &'static str {
    use UnaOp::*;
    match op {
        Neg => "neg",
        _ => todo!(),
    }
}

fn brops_print(op: BrOp) -> &'static str {
    use BrOp::*;
    match op {
        Br => "br",
        BrFalse => "brfl",
        _ => todo!(),
    }
}

pub fn secd_parse(lines: &Vec<String>) -> Vec<SECDInstr> {
    use SECDInstr::*;
    lines
        .iter()
        .map(|line| {
            let mut t = line.split_whitespace();
            let op = t.next().unwrap();
            let args: Vec<&str> = t.collect();
            if op.ends_with(":") {
                assert_eq!(args.len(), 0);
                return Label(line[..line.len() - 1].to_string());
            }
            if BINOPS_PARSE.contains_key(op) {
                return Binary(BINOPS_PARSE[op]);
            }
            if UNAOPS_PARSE.contains_key(op) {
                return Unary(UNAOPS_PARSE[op]);
            }
            if BROPS_PARSE.contains_key(op) {
                assert_eq!(args.len(), 1);
                return Branch(BROPS_PARSE[op], args[0].to_string());
            }
            match op {
                "access" => {
                    assert_eq!(args.len(), 1);
                    let n: usize = args[0].parse().unwrap();
                    Access(n)
                }
                "closure" => {
                    assert_eq!(args.len(), 1);
                    let focused_fn = args[0];
                    Closure(focused_fn.to_string())
                }
                "closures" => {
                    assert!(args.len() >= 1);
                    let mutrec_fns = args.iter().map(|x| x.to_string()).collect();
                    Closures(mutrec_fns)
                }
                "return" => Return,
                "halt" => Halt,
                "focus" => {
                    assert_eq!(args.len(), 1);
                    let n: usize = args[0].parse().unwrap();
                    Focus(n)
                }
                "apply" => Apply,
                "builtin" => {
                    assert_eq!(args.len(), 1);
                    Builtin(args[0].to_string())
                }
                "pushenv" => PushEnv,
                "const" => {
                    assert_eq!(args.len(), 1);
                    if let Ok(v) = args[0].parse::<isize>() {
                        return Const(SECDVal::IntVal(v));
                    }
                    eprintln!("bad const line: {}", line);
                    std::process::exit(1)
                }
                "pop" => {
                    assert_eq!(args.len(), 1);
                    let n: usize = args[0].parse().unwrap();
                    Pop(n)
                }
                _ => {
                    eprintln!("bad line: {}", line);
                    std::process::exit(1);
                }
            }
        })
        .collect()
}

impl std::fmt::Display for SECDInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SECDInstr::Halt => write!(f, "halt"),
            SECDInstr::Pop(n) => write!(f, "pop {n}"),
            SECDInstr::Apply => write!(f, "apply"),
            SECDInstr::Const(v) => match v {
                SECDVal::IntVal(v) => write!(f, "const {v}"),
                _ => todo!(),
            },
            SECDInstr::Access(n) => write!(f, "access {n}"),
            SECDInstr::Focus(n) => write!(f, "focus {n}"),
            SECDInstr::Return => write!(f, "return"),
            SECDInstr::Closure(fnn) => write!(f, "closure {}", fnn),
            SECDInstr::Closures(fns) => write!(f, "closures {}", fns.join(" ")),
            SECDInstr::Builtin(op) => write!(f, "builtin {op}"),
            SECDInstr::Binary(op) => write!(f, "{}", binops_print(*op)),
            SECDInstr::Unary(op) => write!(f, "{}", unaops_print(*op)),
            SECDInstr::Branch(op, label) => write!(f, "{} {label}", brops_print(*op)),
            SECDInstr::Label(label) => write!(f, "{label}:"),
            SECDInstr::PushEnv => write!(f, "pushenv"),
        }
    }
}
