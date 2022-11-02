//! Parsing and printing of SECD values.

use phf::phf_map;

use super::langdef::{BinOp, BrOp, BuiltinOp, SECDInstr, SECDVal, UnaOp};

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

static BUILTINOPS_PARSE: phf::Map<&'static str, BuiltinOp> = {
    use BuiltinOp::*;
    phf_map! {
        "println" => Println,
    }
};

pub fn binops_print(op: BinOp) -> &'static str {
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

pub fn unaops_print(op: UnaOp) -> &'static str {
    use UnaOp::*;
    match op {
        Neg => "neg",
        _ => todo!(),
    }
}

pub fn brops_print(op: BrOp) -> &'static str {
    use BrOp::*;
    match op {
        Br => "br",
        BrFalse => "brfl",
        _ => todo!(),
    }
}

pub fn builtinops_print(op: BuiltinOp) -> &'static str {
    use BuiltinOp::*;
    match op {
        Println => "println",
    }
}

pub fn translate_binop(op: crate::ast::BinOp) -> crate::secd::langdef::BinOp {
    match op {
        crate::ast::BinOp::Add => crate::secd::langdef::BinOp::Add,
        crate::ast::BinOp::Sub => crate::secd::langdef::BinOp::Sub,
        crate::ast::BinOp::Mul => crate::secd::langdef::BinOp::Mul,
        crate::ast::BinOp::Div => crate::secd::langdef::BinOp::Div,
        crate::ast::BinOp::Rem => crate::secd::langdef::BinOp::Rem,
        crate::ast::BinOp::Gt => crate::secd::langdef::BinOp::Gt,
        crate::ast::BinOp::Lt => crate::secd::langdef::BinOp::Lt,
        crate::ast::BinOp::Ge => crate::secd::langdef::BinOp::Ge,
        crate::ast::BinOp::Le => crate::secd::langdef::BinOp::Le,
        crate::ast::BinOp::Eq => crate::secd::langdef::BinOp::Eq,
        crate::ast::BinOp::Ne => crate::secd::langdef::BinOp::Ne,
        crate::ast::BinOp::Land => crate::secd::langdef::BinOp::Land,
        crate::ast::BinOp::Lor => crate::secd::langdef::BinOp::Lor,
        crate::ast::BinOp::Lxor => crate::secd::langdef::BinOp::Lxor,
    }
}

pub fn translate_builtinop(op: crate::ast::BuiltinOp) -> crate::secd::langdef::BuiltinOp {
    match op {
        crate::ast::BuiltinOp::Println => crate::secd::langdef::BuiltinOp::Println,
        crate::ast::BuiltinOp::Nth => todo!(),
        crate::ast::BuiltinOp::True => unreachable!(),
        crate::ast::BuiltinOp::False => unreachable!(),
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
                    Builtin(BUILTINOPS_PARSE[args[0]])
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
            SECDInstr::Builtin(op) => write!(f, "builtin {}", builtinops_print(*op)),
            SECDInstr::Binary(op) => write!(f, "{}", binops_print(*op)),
            SECDInstr::Unary(op) => write!(f, "{}", unaops_print(*op)),
            SECDInstr::Branch(op, label) => write!(f, "{} {label}", brops_print(*op)),
            SECDInstr::Label(label) => write!(f, "{label}:"),
            SECDInstr::PushEnv => write!(f, "pushenv"),
        }
    }
}

impl std::fmt::Display for SECDVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SECDVal::IntVal(v) => write!(f, "{v}"),
            SECDVal::UnitVal => write!(f, "()"),
            SECDVal::TupleVal(vs) => {
                write!(f, "(")?;
                for x in vs {
                    x.fmt(f)?;
                }
                write!(f, ")")
            }
            SECDVal::BuiltinVal(op) => write!(f, "{}", builtinops_print(*op)),
            _ => write!(f, "{:?}", self),
        }
    }
}
