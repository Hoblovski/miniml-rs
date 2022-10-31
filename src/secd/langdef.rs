//! SECD language syntax definition.

#[derive(Debug, Clone)]
pub enum SECDVal {
    IntVal(isize),
    UnitVal,
    TupleVal(Vec<SECDVal>),
    ClosureVal {
        // Functions are represented with pc.
        focused_fn: Option<usize>,
        mutrec_fns: Vec<usize>,
        env: Vec<SECDVal>, // TODO: optimize this
    },
    BuiltinVal(BuiltinOp),
    EnvVal(Vec<SECDVal>),
    PCVal(usize),
}

#[derive(Debug, Clone)]
pub enum SECDInstr {
    Halt,
    Pop(usize),
    Apply,
    Const(SECDVal),
    Access(usize),
    Focus(usize),
    Return,
    Closure(String),
    Closures(Vec<String>),
    Builtin(BuiltinOp),
    Binary(BinOp),
    Unary(UnaOp),
    Branch(BrOp, String),
    Label(String),
    PushEnv,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
    Land,
    Lor,
    Lxor,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum UnaOp {
    Neg,
    Lnot,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BrOp {
    Br,
    BrFalse,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BuiltinOp {
    Println,
}
