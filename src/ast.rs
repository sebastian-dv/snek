#[derive(Debug, Clone)]
pub struct Program {
    pub defs: Vec<FunDef>,
    pub main: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct FunDef {
    pub name: String,
    pub args: Vec<String>,
    pub arg_types: Vec<Type>,
    pub ret_type: Option<Type>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i64),
    Bool(bool),
    Id(String),
    Input,
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
    Call(String, Vec<Expr>),
    Cast(Type, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Op1 {
    Add1,
    Sub1,
    Negate,
    IsNum,
    IsBool,
    Print,
}

#[derive(Debug, Clone)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Num,
    Bool,
    Nothing,
    Any,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Num => "Num".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Nothing => "Nothing".to_string(),
            Type::Any => "Any".to_string(),
        }
    }
}

pub enum ReplEntry {
    Define(String, Box<Expr>),
    FunDef(FunDef),
    Expr(Box<Expr>),
}
