use super::instruction::{Instruction, Val, Reg, Instruction::*, Val::*, Reg::*};
use crate::{
    runtime::start::{snek_error_repl, snek_print},
};

use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi, DynamicLabel};
use im::HashMap;

pub fn instructions_to_str(instrs: &Vec<Instruction>) -> String {
    instrs.iter()
      .map(|i| instr_to_str(i))
      .collect::<Vec<_>>()
      .join("\n")
}

fn instr_to_str(i: &Instruction) -> String {
    match i {
        IMov(r, v) => format!("mov {}, {}", val_to_str(r), val_to_str(v)),
        ILea(r, v) => format!("lea {}, [rel {}]", val_to_str(r), v),
        IAdd(r, v) => format!("add {}, {}", val_to_str(r), val_to_str(v)),
        ISub(r, v) => format!("sub {}, {}", val_to_str(r), val_to_str(v)),
        IMul(r, v) => format!("imul {}, {}", val_to_str(r), val_to_str(v)),
        INeg(r) => format!("neg {}", val_to_str(r)),
        ISar(r, v) => format!("sar {}, {}", val_to_str(r), val_to_str(v)),
        ICmp(r, v) => format!("cmp {}, {}", val_to_str(r), val_to_str(v)),
        ITest(r, v) => format!("test {}, {}", val_to_str(r), val_to_str(v)),
        ICmove(r, v) => format!("cmove {}, {}", val_to_str(r), val_to_str(v)),
        Label(s) => format!("{}:", s),
        Comment(s) => format!(";{}", s),
        ICallErr(e) => format!("mov rdi, {}\ncall snek_error", e),
        IJmp(v) => format!("jmp {}", val_to_str(v)),
        IJe(s) => format!("je {}", s),
        IJne(s) => format!("jne {}", s),
        IJg(s) => format!("jg {}", s),
        IJl(s) => format!("jl {}", s),
        IJge(s) => format!("jge {}", s),
        IJle(s) => format!("jle {}", s),
        IJo(s) => format!("jo {}", s),
        IPrint => format!("mov rdi, rax\npush rsp\npush rax\ncall snek_print\npop rax\npop rsp"),
    }
}

fn val_to_str(v: &Val) -> String {
    match v {
        Reg(r) => reg_to_str(r),
        Mem(r, offset) => format!("[{} - {}]", reg_to_str(r), offset),
        I(n) => format!("{}", n),
        S(s) => s.clone(),
    }
}

fn reg_to_str(r: &Reg) -> String {
    match r {
        RAX => "rax".to_string(),
        RCX => "rcx".to_string(),
        RSP => "rsp".to_string(),
        RDI => "rdi".to_string(),
    }
}

pub fn instructions_to_asm(instrs: &Vec<Instruction>,
    ops: &mut dynasmrt::x64::Assembler, labels: &HashMap<String, DynamicLabel>) {
    for i in instrs {
        match i {
            IMov(dst, src) => match (dst, src) {
                (Reg(RAX), I(n)) => dynasm!(ops ; .arch x64 ; mov rax, QWORD *n),
                (Reg(RAX), Reg(RDI)) => dynasm!(ops ; .arch x64 ; mov rax, rdi),
                (Reg(RAX), Mem(RAX, off)) => dynasm!(ops ; .arch x64 ; mov rax, [rax - *off]),
                (Reg(RAX), Mem(RSP, off)) => dynasm!(ops ; .arch x64 ; mov rax, [rsp - *off]),
                (Reg(RCX), I(n)) => dynasm!(ops ; .arch x64 ; mov rcx, QWORD *n),
                (Reg(RCX), Reg(RAX)) => dynasm!(ops ; .arch x64 ; mov rcx, rax),
                (Mem(RCX, off), Reg(RAX)) => dynasm!(ops ; .arch x64 ; mov [rcx - *off], rax),
                (Reg(RDI), I(n)) => dynasm!(ops ; .arch x64 ; mov rax, QWORD *n),
                (Mem(RSP, off), I(n)) => dynasm!(ops ; .arch x64 ; mov QWORD [rsp - *off], *n as i32),
                (Mem(RSP, off), Reg(RAX)) => dynasm!(ops ; .arch x64 ; mov [rsp - *off], rax),
                (Mem(RSP, off), Reg(RCX)) => dynasm!(ops ; .arch x64 ; mov [rsp - *off], rcx),
                _ => unimplemented!("IMov: {:?} -> {:?}", dst, src),
            },
            ILea(dst, src) => match (dst, src) {
                (Reg(RAX), name) => {
                    let label = labels.get(name).unwrap();
                    dynasm!(ops ; .arch x64 ; lea rax, [=>*label])
                },
                (Reg(RCX), name) => {
                    let label = labels.get(name).unwrap();
                    dynasm!(ops ; .arch x64 ; lea rcx, [=>*label])
                },
                _ => unimplemented!("IMov: {:?} -> {:?}", dst, src),
            },
            IAdd(dst, src) => match (dst, src) {
                (Reg(RAX), I(n)) => dynasm!(ops ; .arch x64 ; add rax, *n as i32),
                (Reg(RAX), Mem(RSP, off)) => dynasm!(ops ; .arch x64 ; add rax, [rsp - *off]),
                (Reg(RSP), I(n)) => dynasm!(ops ; .arch x64 ; add rsp, *n as i32),
                _ => unimplemented!("IAdd: {:?} + {:?}", dst, src),
            },
            ISub(dst, src) => match (dst, src) {
                (Reg(RAX), I(n)) => dynasm!(ops ; .arch x64 ; sub rax, *n as i32),
                (Reg(RAX), Mem(RSP, off)) => dynasm!(ops ; .arch x64 ; sub rax, [rsp - *off]),
                (Reg(RSP), I(n)) => dynasm!(ops ; .arch x64 ; sub rsp, *n as i32),
                _ => unimplemented!("ISub: {:?} - {:?}", dst, src),
            },
            IMul(dst, src) => match (dst, src) {
                (Reg(RAX), Mem(RSP, off)) => dynasm!(ops ; .arch x64 ; imul rax, [rsp - *off]),
                _ => unimplemented!("IMul: {:?} * {:?}", dst, src),
            },
            INeg(dst) => match dst {
                Reg(RAX) => dynasm!(ops ; .arch x64 ; neg rax),
                _ => unimplemented!("INeg: {:?}", dst),
            },
            ISar(dst, src) => match (dst, src) {
                (Reg(RAX), I(n)) => dynasm!(ops ; .arch x64 ; sar rax, *n as i8),
                _ => unimplemented!("ISar: {:?} >> {:?}", dst, src),
            }
            ICmp(dst, src) => match (dst, src) {
                (Reg(RAX), I(n)) => dynasm!(ops ; .arch x64 ; cmp rax, *n as i32),
                (Reg(RAX), Reg(RCX)) => dynasm!(ops ; .arch x64 ; cmp rax, rcx),
                (Reg(RAX), Mem(RSP, off)) => dynasm!(ops ; .arch x64 ; cmp rax, [rsp - *off]),
                _ => unimplemented!("ICmp: {:?} {:?}", dst, src),
            },
            ITest(dst, src) => match (dst, src) {
                (Reg(RAX), I(n)) => dynasm!(ops ; .arch x64 ; test rax, *n as i32),
                (Reg(RAX), Mem(RSP, off)) => dynasm!(ops ; .arch x64 ; test rax, [rsp - *off]),
                (Reg(RCX), I(n)) => dynasm!(ops ; .arch x64 ; test rcx, *n as i32),
                _ => unimplemented!("ITest: {:?} {:?}", dst, src),
            },
            ICmove(dst, src) => match (dst, src) {
                (Reg(RAX), Mem(RSP, off)) => dynasm!(ops ; .arch x64 ; cmove rax, [rsp - *off]),
                _ => unimplemented!("ICmove: {:?} {:?}", dst, src),
            },
            Label(name) => {
                let label = labels.get(name).unwrap();
                dynasm!(ops ; .arch x64 ; =>*label)
            },
            Comment(_) => {},
            ICallErr(code) => {
                let snek_error_addr = snek_error_repl as *const () as i64;
                dynasm!(ops ; .arch x64
                    ; mov rdi, QWORD *code
                    ; mov rax, QWORD snek_error_addr
                    ; call rax
                    ; mov rax, 5
                    ; ret)
            },
            IJmp(dst) => match dst {
                Mem(RSP, _) => dynasm!(ops ; .arch x64 ; jmp QWORD [rsp]),
                S(name) => {
                    let label = labels.get(name).unwrap();
                    dynasm!(ops ; .arch x64 ; jmp =>*label)
                },
                _ => unimplemented!("IJmp: {:?}", dst),
            },
            IJe(name) => {
                let label = labels.get(name).unwrap();
                dynasm!(ops ; .arch x64 ; je =>*label)
            },
            IJne(name) => {
                let label = labels.get(name).unwrap();
                dynasm!(ops ; .arch x64 ; jne =>*label)
            },
            IJg(name) => {
                let label = labels.get(name).unwrap();
                dynasm!(ops ; .arch x64 ; jg =>*label)
            },
            IJl(name) => {
                let label = labels.get(name).unwrap();
                dynasm!(ops ; .arch x64 ; jl =>*label)
            },
            IJge(name) => {
                let label = labels.get(name).unwrap();
                dynasm!(ops ; .arch x64 ; jge =>*label)
            },
            IJle(name) => {
                let label = labels.get(name).unwrap();
                dynasm!(ops ; .arch x64 ; jle =>*label)
            },
            IJo(name) => {
                let label = labels.get(name).unwrap();
                dynasm!(ops ; .arch x64 ; jo =>*label)
            },
            IPrint => {
                let snek_print_addr = snek_print as *const () as i64;
                dynasm!(ops ; .arch x64
                    ; mov rdi, rax
                    ; push rsp
                    ; push rax
                    ; mov rcx, QWORD snek_print_addr
                    ; call rcx
                    ; pop rax
                    ; pop rsp)
            },
        }
    }
}
