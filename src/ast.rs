/// AST node definitions.

#[derive(Debug)]
pub struct Top {}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaOp {
    Neg,
    Lnot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuiltinOp {
    Println,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    UnitTy,
    IntTy,
    BoolTy,
    UnkTy, // use this variant than Option<Ty>
    AbsTy(Box<Ty>, Box<Ty>),
}

#[derive(Debug)]
pub struct LetRecArm {
    pub fn_name: String,
    pub fn_ty: Ty,
    pub arg_name: String,
    pub arg_ty: Ty,
    pub body: Box<Expr>,
}

// OPT: less Strings and Boxes
#[derive(Debug)]
pub enum Expr {
    IntLit {
        val: i64,
    },
    UnitLit {},
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnaOp,
        sub: Box<Expr>,
    },
    VarRef {
        id: String,
    },
    Builtin {
        op: BuiltinOp,
    },
    App {
        fun: Box<Expr>,
        arg: Box<Expr>,
    },
    Seq {
        subs: Vec<Box<Expr>>,
    },
    Abs {
        arg_name: String,
        arg_ty: Ty,
        body: Box<Expr>,
    },
    Let {
        name: String,
        ty: Ty,
        val: Box<Expr>,
        body: Box<Expr>,
    },
    Tuple {
        subs: Vec<Box<Expr>>,
    },
    Nth {
        idx: i64,
        sub: Box<Expr>,
    },
    Ite {
        cond: Box<Expr>,
        tr: Box<Expr>,
        fl: Box<Expr>,
    },
    LetRec {
        arms: Vec<LetRecArm>,
        body: Box<Expr>,
    },
}

#[derive(Debug)]
pub struct Prog {
    pub main_expr: Expr,
}
