use phf::phf_map;

/// AST node definitions.
///
/// String literals are String rather than &str.
/// Because a node can be mutated and the lifetime of string literals is unclear.
/// Maybe an alternative is to use immutable trees, but at copying cost.
///
/// Sub-nodes are Boxes rather than owned values: to prevent over-size nodes.
/// They are not Rc: not very necessary (since all manipulations will be conducted in a visitor), plus Rc cannot be pattern matched.
/// They are not simple references: nodes can be created and replaced like `*node = CreateNewNode`.

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
    Nth,
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    UnitTy,
    IntTy,
    BoolTy,
    UnkTy, // use this variant than Option<Ty>
    AbsTy(Box<Ty>, Box<Ty>),
    DataTy(String),
}

#[derive(Debug)]
pub struct LetRecArm {
    pub fn_name: String,
    pub fn_ty: Ty,
    pub arg_name: String,
    pub arg_ty: Ty,
    pub body: Box<Expr>,
}

#[derive(Debug)]
pub enum MatchPattern {
    Binder {
        name: String,
    },
    Tuple {
        subs: Vec<MatchPattern>,
    },
    Lit {
        val: Expr,
    },
    DataType {
        ctor: String,
        subs: Vec<MatchPattern>,
    },
}

#[derive(Debug)]
pub struct MatchArm {
    pub ptn: MatchPattern,
    pub res: Expr,
}

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
    Match {
        sub: Box<Expr>,
        arms: Vec<MatchArm>,
    },
}

#[derive(Debug, PartialEq)]
pub struct DataTypeArm {
    pub ctor: String,
    pub arg_tys: Vec<Ty>,
}

#[derive(Debug, PartialEq)]
pub struct DataType {
    pub name: String,
    pub arms: Vec<DataTypeArm>,
}

#[derive(Debug)]
pub struct Prog {
    pub data_types: Vec<DataType>,
    pub main_expr: Expr,
}

pub static BUILTIN_PARSE: phf::Map<&'static str, BuiltinOp> = phf_map! {
    "println" => BuiltinOp::Println,
    "true" => BuiltinOp::True,
    "false" => BuiltinOp::False,
    "nth" => BuiltinOp::Nth,
};

pub fn builtin_print(op: BuiltinOp) -> &'static str {
    match op {
        BuiltinOp::Println => "println",
        BuiltinOp::Nth => "nth",
        BuiltinOp::True => "true",
        BuiltinOp::False => "false",
    }
}
