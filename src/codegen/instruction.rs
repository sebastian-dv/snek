#[derive(Debug, Clone)]
pub enum Instruction {
    IMov(Val, Val),
    ILea(Val, String),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
    INeg(Val),
    ISar(Val, Val),
    ICmp(Val, Val),
    ITest(Val, Val),
    ICmove(Val, Val),
    Label(String),
    Comment(String),
    ICallErr(i64),
    IJmp(Val),
    IJe(String),
    IJne(String),
    IJg(String),
    IJl(String),
    IJge(String),
    IJle(String),
    IJo(String),
    IPrint,
}

#[derive(Debug, Clone)]
pub enum Val {
    Reg(Reg),
    Mem(Reg, i32),
    I(i64),
    S(String),
}

#[derive(Debug, Clone)]
pub enum Reg {
    RAX,
    RCX,
    RSP,
    RDI
}

