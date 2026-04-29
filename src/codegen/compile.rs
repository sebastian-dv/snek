use crate::{
    ast::{Program, Expr, Op1, Op2, FunDef, Type},
    snek::{SNEK_TRUE, SNEK_FALSE, bool_to_snek, num_to_snek},
};
use super::instruction::{Instruction, Instruction::*, Val::*, Reg::*};
use super::context::Context;

use im::HashMap;

pub fn compile_program(prog: &Program, ctx: &Context) -> Result<(Vec<Instruction>, Vec<Instruction>), String>{
    let mut fun_env = HashMap::new();
    for def in &prog.defs {
        if fun_env.contains_key(&def.name) {
            return Err(format!("duplicate function definition: {}", def.name));
        }
        fun_env = fun_env.update(def.name.clone(), def.args.len() as i32);
    }
    let mut new_ctx = ctx.clone();
    new_ctx.with_fun_env(fun_env.clone());

    let mut fun_instrs = Vec::new();
    for def in &prog.defs {
        let def_instrs = compile_fundef(def, &new_ctx.clone())?;
        fun_instrs.extend(def_instrs);
    }

    let mut main_instrs = compile_to_instructions(&prog.main, &new_ctx)?;
    main_instrs.push(Comment("Main expression".to_string()));
    Ok((fun_instrs, main_instrs))
}

pub fn compile_fundef(def: &FunDef, ctx: &Context) -> Result<Vec<Instruction>, String> {
    let mut instrs = Vec::new();

    let fun_label = format!("fun_{}", def.name);
    instrs.push(Label(fun_label.clone()));
    instrs.push(Comment(format!("Function: {} with {} args", def.name, def.args.len())));

    let mut new_ctx = ctx.clone();
    for arg in def.args.iter() {
        new_ctx.insert_stack(arg.clone(), new_ctx.curr_offset());
        new_ctx.increment_si_by(1);
    }

    let body_instrs = compile_to_instructions(&def.body, &new_ctx)?;
    instrs.extend(body_instrs);

    instrs.push(Comment("Return from function".to_string()));
    instrs.push(IJmp(Mem(RSP, 0)));

    Ok(instrs)
}

pub fn compile_to_instructions(e: &Expr, ctx: &Context) -> Result<Vec<Instruction>, String> {
    let instructions = match e {
        Expr::Num(n) => Ok(vec![IMov(Reg(RAX), I(num_to_snek(n)))]),
        Expr::Bool(b) => Ok(vec![IMov(Reg(RAX), I(bool_to_snek(*b)))]),
        Expr::Input => Ok(vec![IMov(Reg(RAX), Reg(RDI))]),
        Expr::Id(s) => {
            match ctx.stack_env.get(s) {
                Some(offset) => Ok(vec![IMov(Reg(RAX), Mem(RSP, *offset as i32))]),
                None => {
                    match ctx.define_env.get(s) {
                        Some(value) => {
                            Ok(vec![IMov(Reg(RAX), I(*value)), 
                                    IMov(Reg(RAX), Mem(RAX, 0))])
                        },
                        None => {
                            match ctx.fun_env.get(s) {
                                Some(_) => {
                                    compile_call(s, &Vec::new(), ctx)
                                },
                                None => Err(format!("unbound variable: {s}")),
                            }
                        }
                    }
                }
            }
        },
        Expr::Let(bindings, subexpr) => {
            let mut instrs = Vec::new();
            let mut new_ctx = ctx.clone();
            let mut curr_bindings = Vec::new();

            for (id, expr) in bindings {
                if curr_bindings.contains(&id) {
                    return Err("duplicate binding".to_string());
                }
                curr_bindings.push(id);

                let mut bind_instrs = compile_to_instructions(expr, &new_ctx)?;
                let offset = new_ctx.curr_offset();
                instrs.append(&mut bind_instrs);
                instrs.push(IMov(Mem(RSP, offset), Reg(RAX)));
                new_ctx.insert_stack(id.to_string(), offset);
                new_ctx.increment_si_by(1);
            }
            let sub_instrs = compile_to_instructions(subexpr, &new_ctx)?;
            Ok([instrs, sub_instrs].concat())
        }
        Expr::UnOp(op, subexpr) => Ok([compile_to_instructions(subexpr, ctx)?, compile_unop(op, ctx)].concat()),
        Expr::BinOp(op, subexpr1, subexpr2) => compile_binop(op, subexpr1, subexpr2, ctx),
        Expr::If(cond, then, els) => {
            let label_else = create_label("else");
            let label_end = create_label("end");

            let i_cond = compile_to_instructions(cond, ctx)?;
            let i_check_bool = create_check(Check::Bool);
            let i_jmp_else = vec![ICmp(Reg(RAX), I(SNEK_TRUE)),
                                IJne(label_else.clone())];
            let cond_instrs = [i_cond, i_check_bool, i_jmp_else].concat();


            let mut then_ctx = ctx.clone();
            then_ctx.increment_si_by(1);
            let i_then = compile_to_instructions(then, &then_ctx)?;
            let i_jmp_end = vec![IJmp(S(label_end.clone()))];
            let then_instrs = [i_then, i_jmp_end].concat();

            let mut else_ctx = ctx.clone();
            else_ctx.increment_si_by(2);
            let i_else_label = vec![Label(label_else.clone())];
            let i_else = compile_to_instructions(els, &else_ctx)?;
            let i_end_label = vec![Label(label_end.clone())];
            let else_instrs = [i_else_label, i_else, i_end_label].concat();

            Ok([cond_instrs, then_instrs, else_instrs].concat())
        },
        Expr::Loop(body) => {
            let label_start = create_label("loop_start");
            let label_end = create_label("loop_end");

            let mut body_ctx = ctx.clone();
            body_ctx.with_break_label(label_end.clone());

            let i_start_label = vec![Label(label_start.clone())];
            let i_body = compile_to_instructions(body, &body_ctx)?;
            let i_jmp_start = vec![IJmp(S(label_start.clone()))];
            let i_end_label = vec![Label(label_end.clone())];

            let loop_instrs = [i_start_label, i_body, i_jmp_start, i_end_label].concat();
            Ok(loop_instrs)
        },
        Expr::Break(subexpr) => {
            let mut break_ctx = ctx.clone();
            break_ctx.clear_break_label();
            let i_ret = compile_to_instructions(subexpr, &break_ctx)?;
            if let Some(label_loop_end) = &ctx.break_label {
                let i_jmp_loop_end = vec![IJmp(S(label_loop_end.to_string()))];
                Ok([i_ret, i_jmp_loop_end].concat())
            } else {
                Err("break used outside of loop".to_string())
            }
        },
        Expr::Set(id, subexpr) => {
            let instrs = compile_to_instructions(subexpr, ctx)?;
            match ctx.stack_env.get(id) {
                Some(offset) => {
                    let set_instrs = vec![IMov(Mem(RSP, *offset), Reg(RAX))];
                    Ok([instrs, set_instrs].concat())
                },
                None => match ctx.define_env.get(id) {
                    Some(ptr) => {
                        let set_define_instrs = vec![IMov(Reg(RCX), I(*ptr)),
                                                        IMov(Mem(RCX, 0), Reg(RAX))];
                        Ok([instrs, set_define_instrs].concat())
                    },
                    None => Err(format!("unbound variable: {id}")),
                }
            }
        },
        Expr::Block(subexprs) => {
            let mut instrs = Vec::new();
            let mut block_ctx = ctx.clone();
            for subexpr in subexprs {
                let mut instr = compile_to_instructions(subexpr, &block_ctx)?;
                instrs.append(&mut instr);
                block_ctx.increment_si_by(1);
            }
            Ok(instrs)
        },
        Expr::Call(name, args) => {
            compile_call(&name, args, ctx)
        },
        Expr::Cast(ty, e) => {
            let expr_instrs = compile_to_instructions(e, ctx)?;
            let check_instrs = create_cast_check(ty);
            Ok([expr_instrs, check_instrs].concat())
        },
    };
    instructions
}

fn compile_unop(o: &Op1, ctx: &Context) -> Vec<Instruction> {
    let stack_offset = ctx.curr_offset();
    let instructions = match o {
        Op1::Add1 => vec![IAdd(Reg(RAX), I(num_to_snek(&1)))],
        Op1::Sub1 => vec![ISub(Reg(RAX), I(num_to_snek(&1)))],
        Op1::Negate => vec![INeg(Reg(RAX))],
        Op1::IsNum => vec![ITest(Reg(RAX), I(SNEK_FALSE)),
                        IMov(Reg(RAX), I(SNEK_FALSE)),
                        IMov(Mem(RSP, stack_offset), I(SNEK_TRUE)),
                        ICmove(Reg(RAX), Mem(RSP, stack_offset))],
        Op1::IsBool => vec![ITest(Reg(RAX), I(SNEK_FALSE)),
                        IMov(Reg(RAX), I(SNEK_TRUE)),
                        IMov(Mem(RSP, stack_offset), I(SNEK_FALSE)),
                        ICmove(Reg(RAX), Mem(RSP, stack_offset))],
        Op1::Print => vec![IPrint],
    };
    instructions
}

fn compile_binop(o: &Op2, e1: &Expr, e2: &Expr, ctx: &Context) -> Result<Vec<Instruction>, String> {
    let stack_offset = ctx.curr_offset();

    let comment = format!("{:?}", o);
    let comment_instrs = vec![Comment(comment)];

    let check_num1_instrs = create_check(Check::Num);
    let check_num2_instrs = create_check(Check::Num);

    let v1_ctx = ctx.clone();
    let val1_instrs = compile_to_instructions(e1, &v1_ctx)?;
    let val1_to_stack_instrs = vec![IMov(Mem(RSP, stack_offset), Reg(RAX))];

    let mut v2_ctx = ctx.clone();
    v2_ctx.increment_si_by(1);
    let val2_instrs = compile_to_instructions(e2, &v2_ctx)?;

    let op_instrs = match o {
        Op2::Plus => vec![IAdd(Reg(RAX), Mem(RSP, stack_offset))],
        Op2::Minus => vec![ISub(Reg(RAX), Mem(RSP, stack_offset))],
        Op2::Times => vec![IMul(Reg(RAX), Mem(RSP, stack_offset))],
        Op2::Equal => {
            let label_eq = create_label("eq");
            let label_end = create_label("end");
            vec![ICmp(Reg(RAX), Mem(RSP, stack_offset)),
                IJe(label_eq.clone()),
                IMov(Reg(RAX), I(SNEK_FALSE)),
                IJmp(S(label_end.clone())),
                Label(label_eq.clone()),
                IMov(Reg(RAX), I(SNEK_TRUE)),
                Label(label_end.clone())]
        },
        Op2::Greater => {
            let label_greater = create_label("greater");
            let label_end = create_label("end");
            vec![ICmp(Reg(RAX), Mem(RSP, stack_offset)),
                IJg(label_greater.clone()),
                IMov(Reg(RAX), I(SNEK_FALSE)),
                IJmp(S(label_end.clone())),
                Label(label_greater.clone()),
                IMov(Reg(RAX), I(SNEK_TRUE)),
                Label(label_end.clone())]
        },
        Op2::GreaterEqual => {
            let label_ge = create_label("greater_equal");
            let label_end = create_label("end");
            vec![ICmp(Reg(RAX), Mem(RSP, stack_offset)),
                IJge(label_ge.clone()),
                IMov(Reg(RAX), I(SNEK_FALSE)),
                IJmp(S(label_end.clone())),
                Label(label_ge.clone()),
                IMov(Reg(RAX), I(SNEK_TRUE)),
                Label(label_end.clone())]
        },
        Op2::Less => {
            let label_less = create_label("less");
            let label_end = create_label("end");
            vec![ICmp(Reg(RAX), Mem(RSP, stack_offset)),
                IJl(label_less.clone()),
                IMov(Reg(RAX), I(SNEK_FALSE)),
                IJmp(S(label_end.clone())),
                Label(label_less.clone()),
                IMov(Reg(RAX), I(SNEK_TRUE)),
                Label(label_end.clone())]
        },
        Op2::LessEqual => {
            let label_le = create_label("less_equal");
            let label_end = create_label("end");
            vec![ICmp(Reg(RAX), Mem(RSP, stack_offset)),
                IJle(label_le.clone()),
                IMov(Reg(RAX), I(SNEK_FALSE)),
                IJmp(S(label_end.clone())),
                Label(label_le.clone()),
                IMov(Reg(RAX), I(SNEK_TRUE)),
                Label(label_end.clone())]
        },
    };
    match o {
        Op2::Plus | Op2::Minus => {
            let check_overflow_instrs = create_check(Check::Overflow);
            Ok([comment_instrs, val1_instrs, check_num1_instrs, val1_to_stack_instrs,
                val2_instrs, check_num2_instrs, op_instrs, check_overflow_instrs].concat())
        }
        Op2::Times => {
            let check_overflow_instrs = create_check(Check::Overflow);
            let val1_shift_instrs = vec![ISar(Reg(RAX), I(1))];
            Ok([comment_instrs, val1_instrs, check_num1_instrs, val1_shift_instrs, val1_to_stack_instrs,
                val2_instrs, check_num2_instrs, op_instrs, check_overflow_instrs].concat())
        }
        Op2::Equal => {
            let val1_to_rcx_instrs = vec![IMov(Reg(RCX), Reg(RAX))];
            let check_type_instrs = create_check(Check::SameType);
            Ok([comment_instrs, val1_instrs, val1_to_rcx_instrs, val1_to_stack_instrs, 
                val2_instrs, check_type_instrs, op_instrs].concat())
        }
        Op2::Greater | Op2::GreaterEqual | Op2::Less | Op2::LessEqual => {
            Ok([comment_instrs, val1_instrs, check_num1_instrs, val1_to_stack_instrs, 
                val2_instrs, check_num2_instrs, op_instrs].concat())
        },
    }
}

fn compile_call(name: &String, args: &Vec<Expr>, ctx: &Context) -> Result<Vec<Instruction>, String> {
    match ctx.fun_env.get(name) {
        Some(expected_args) => {
            let num_args = args.len() as i32;
            if num_args != *expected_args {
                return Err(format!("function {} expects {} arguments, got {}",
                                name, expected_args, num_args
                ));
            }

            let mut new_ctx = ctx.clone();
            let mut instrs = Vec::new();
            instrs.push(Comment(format!("Call: {} with {} args", name, num_args)));

            let fun_offset = new_ctx.curr_offset();
            let after_call_label = create_label(&format!("after_call_{}", name));
            instrs.push(Comment("Save return address".to_string()));
            instrs.push(ILea(Reg(RCX), after_call_label.clone()));
            instrs.push(IMov(Mem(RSP, fun_offset), Reg(RCX)));

            instrs.push(Comment("Evaluate and store arguments".to_string()));
            for arg in args.iter() {
                new_ctx.increment_si_by(1);
                let arg_instrs = compile_to_instructions(arg, &new_ctx)?;
                instrs.extend(arg_instrs);
                instrs.push(IMov(Mem(RSP, new_ctx.curr_offset()), Reg(RAX)));
            }
            instrs.push(Comment(format!("Adjust RSP")));
            instrs.push(ISub(Reg(RSP), I(fun_offset as i64)));
            let fun_label = format!("fun_{}", name);
            instrs.push(Comment(format!("Jump to function {}", name)));
            instrs.push(IJmp(S(fun_label)));
            instrs.push(Label(after_call_label.clone()));
            instrs.push(Comment("Returned from function".to_string()));
            instrs.push(IAdd(Reg(RSP), I(fun_offset as i64)));
            Ok(instrs)
        },
        None => Err(format!("unknown function: {}", name)),
    }
}

// Helpers

static mut LABEL_COUNTER: i64 = 0;
fn create_label(s: &str) -> String {
    unsafe {
        let label = format!("{}_{}", s, LABEL_COUNTER);
        LABEL_COUNTER += 1;
        label
    }
}

enum ErrorCode {
    InvalidArg = 1,
    Overflow = 2,
    BadCast = 3,
}

enum Check {
    Bool,
    Num,
    Overflow,
    SameType,
}

fn create_check(c: Check) -> Vec<Instruction> {
    let comment;
    let label_ok = create_label("ok");
    let check_instrs = match c {
        Check::Bool => {
            comment = format!("Check: Bool");
            vec![ICmp(Reg(RAX), I(SNEK_FALSE)),
                IJe(label_ok.clone()),
                ICmp(Reg(RAX), I(SNEK_TRUE)),
                IJe(label_ok.clone()),
                ICallErr(ErrorCode::InvalidArg as i64),
                Label(label_ok.clone())]
        },
        Check::Num => {
            comment = format!("Check: Num");
            vec![ITest(Reg(RAX), I(1)), // check LSB
                IJe(label_ok.clone()), // je = jz, check zero
                ICallErr(ErrorCode::InvalidArg as i64),
                Label(label_ok.clone())]
        },
        Check::Overflow => {
            let label_err = create_label("err");
            comment = format!("Check: Overflow");
            vec![IJo(label_err.clone()),
                IJmp(S(label_ok.clone())),
                Label(label_err.clone()),
                ICallErr(ErrorCode::Overflow as i64),
                Label(label_ok.clone())]
        },
        Check::SameType => { // Checks RAX and RCX
            let label_is_bool = create_label("is_bool");
            let label_is_num = create_label("is_num");
            let label_err = create_label("err");
            comment = format!("Check: SameType");
            vec![ITest(Reg(RAX), I(1)),
                IJne(label_is_bool.clone()), // RAX is bool, jne = jnz
                Label(label_is_num.clone()), //RAX is num
                ITest(Reg(RCX), I(1)),
                IJne(label_err.clone()), // RCX is bool, error
                IJmp(S(label_ok.clone())), // RCX is num, )ok
                Label(label_is_bool.clone()),
                ITest(Reg(RCX), I(1)),
                IJne(label_ok.clone()), // RCX is bool, ok
                Label(label_err.clone()),
                ICallErr(ErrorCode::InvalidArg as i64),
                Label(label_ok.clone())]
        },
    };
    let comment_instrs = vec![Comment(comment)];
    [comment_instrs, check_instrs].concat()
}

fn create_cast_check(ty: &Type) -> Vec<Instruction> {
    match ty {
        Type::Num => {
            let label_ok = create_label("cast_ok");
            vec![
                Comment("Check: Cast to Num".to_string()),
                ITest(Reg(RAX), I(1)), // Check LSB (0 = num, 1 = bool)
                IJe(label_ok.clone()), // If 0 (num), ok
                ICallErr(ErrorCode::BadCast as i64), // If 1 (bool), error
                Label(label_ok),
            ]
        },
        Type::Bool => {
            let label_ok = create_label("cast_ok");
            vec![
                Comment("Check: Cast to Bool".to_string()),
                ITest(Reg(RAX), I(1)), // Check LSB (0 = num, 1 = bool)
                IJne(label_ok.clone()), // If 1 (bool), ok
                ICallErr(ErrorCode::BadCast as i64), // If 0 (num), error
                Label(label_ok),
            ]
        },
        Type::Nothing => {
            vec![
                Comment("Check: Cast to Nothing (always fails)".to_string()),
                ICallErr(ErrorCode::BadCast as i64)
            ]
        },
        Type::Any => {
            vec![Comment("Check: Cast to Any (always succeeds)".to_string())]
        },
    }
}
