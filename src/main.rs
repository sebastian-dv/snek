use std::{
    env,
    mem,
    fs::File,
    io,
    io::prelude::*,
};
use sexp::*;
use dynasmrt::{dynasm, DynasmApi, DynamicLabel, DynasmLabelApi};
use im::HashMap;

pub mod snek;
pub mod ast;
pub mod parser;
pub mod runtime;
pub mod typecheck;
mod codegen;

use codegen::{Context, Instruction, compile_program, compile_fundef, compile_to_instructions, instructions_to_str, instructions_to_asm};
use crate::{
    ast::{ReplEntry, Type},
    parser::{parse_program, parse_repl_entry},
    snek::{SNEK_TRUE, SNEK_FALSE, from_snek},
    typecheck::{typecheck_program},
};

//static EXPECTED_ARGS = "Specify flag:\n-c (in) (out)\n-e (in) (optional_input)\n-g (in) (out) (optional_input)\n-i";

fn compile_aot(in_name: &str, out_name: &str, typecheck: bool) -> std::io::Result<()> {
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    in_contents = format!("({})", in_contents);
    let parsed_file = parse(&in_contents).unwrap();
    let program = match parse_program(&parsed_file) {
        Ok(p) => p,
        Err(e) => panic!("Parse error: {}", e),
    };

    if typecheck {
        match typecheck_program(&program, None) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        }
    }

    let mut context = Context::new();
    let (fun_instrs, main_instrs) = match compile_program(&program, &mut context) {
        Ok(i) => i,
        Err(e) => panic!("Compile error: {}", e),
    };

    let fun_str = instructions_to_str(&fun_instrs);
    let main_str = instructions_to_str(&main_instrs);
    let asm_program = format!(
        "section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        {}
        our_code_starts_here:\n {}\nret\n",
        fun_str, main_str);
    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}

fn compile_jit(mut ops: dynasmrt::x64::Assembler, in_name: &str, input: Option<&String>, typecheck: bool) -> std::io::Result<()> {
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    in_contents = format!("({})", in_contents);
    let parsed_file = match parse(&in_contents) {
        Ok(i) => i,
        Err(_) => panic!("Input parse error"),
    };
    let program = match parse_program(&parsed_file) {
        Ok(p) => p,
        Err(e) => panic!("Parse error: {}", e),
    };

    let mut context = Context::new();
    let (fun_instrs, main_instrs) = match compile_program(&program, &mut context) {
        Ok(i) => i,
        Err(e) => panic!("Compile error: {}", e),
    };

    let mut input_type = Some(Type::Bool);
    let input_val: i64 = match input {
        Some(i) => match i.as_str() {
            "true" => SNEK_TRUE,
            "false" => SNEK_FALSE,
            _ => {
                match i.parse::<i64>() {
                    Ok(num) =>  {
                        input_type = Some(Type::Num);
                        num << 1
                    },
                    Err(_) => SNEK_FALSE,
                }
            }
        },
        None => SNEK_FALSE
    };

    if typecheck {
        match typecheck_program(&program, input_type) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        }
    }

    let start = ops.offset();

    let mut labels: HashMap<String, DynamicLabel> = HashMap::new();
    for instr in &fun_instrs {
        if let Instruction::Label(name) = instr {
            labels.insert(name.clone(), ops.new_dynamic_label());
        }
    }
    for instr in &main_instrs {
        if let Instruction::Label(name) = instr {
            labels.insert(name.clone(), ops.new_dynamic_label());
        }
    }

    let to_main = ops.new_dynamic_label();
    dynasm!(ops ; .arch x64 ; jmp =>to_main);
    instructions_to_asm(&fun_instrs, &mut ops, &labels);
    dynasm!(ops ; .arch x64 ; =>to_main);
    instructions_to_asm(&main_instrs, &mut ops, &labels);
    dynasm!(ops ; .arch x64 ; ret);

    let buf = ops.finalize().unwrap();
    let jitted_fn: extern "C" fn(i64) -> i64 = unsafe { mem::transmute(buf.ptr(start)) };

    let result = jitted_fn(input_val);
    let result_str = from_snek(&result);
    if result_str == "Runtime Error" {
        std::process::exit(1);
    } else{
        println!("{}", result_str);
    };
    Ok(())
}

fn compile_repl(mut ops: dynasmrt::x64::Assembler, typecheck: bool) -> std::io::Result<()> {
    let mut context = Context::new();
    let mut labels: HashMap<String, DynamicLabel> = HashMap::new();
    let mut fun_ops = dynasmrt::x64::Assembler::new().unwrap();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input == "exit" || input == "quit" {
            break;
        }

        let parsed_input = match parse(&input) {
            Ok(i) => i,
            Err(_) => {
                eprintln!("Input parse error");
                continue;
            },
        };
        let repl_entry = match parse_repl_entry(&parsed_input) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Parse error: {}", e);
                continue;
            },
        };

        match repl_entry {
            ReplEntry::Define(name, expr) => {
                if context.define_env.contains_key(&name) {
                    eprintln!("Error: duplicate binding");
                    continue;
                }
                let instrs = match compile_to_instructions(&expr, &context) {
                    Ok(instrs) => instrs,
                    Err(e) => {
                        eprintln!("Compile error: {}", e);
                        continue;
                    }
                };
                for instr in &instrs {
                    if let Instruction::Label(name) = instr {
                        labels.insert(name.clone(), ops.new_dynamic_label());
                    }
                }

                let start = ops.offset();

                instructions_to_asm(&instrs, &mut ops, &labels);
                dynasm!(ops ; .arch x64 ; ret);

                ops.commit().unwrap();
                let reader = ops.reader();
                let buf = reader.lock();
                let jitted_fn: extern "C" fn() -> i64 = unsafe { mem::transmute(buf.ptr(start)) };
                let result = jitted_fn();

                let boxed_res = Box::new(result);
                let ptr = Box::into_raw(boxed_res) as i64;
                context.insert_define(name, ptr);
            },
            ReplEntry::FunDef(f) => {
                if context.fun_env.contains_key(&f.name) {
                    eprintln!("Error: duplicate function name");
                    continue;
                }
                let instrs = match compile_fundef(&f, &context) {
                    Ok(instrs) => instrs,
                    Err(e) => {
                        eprintln!("Compile error: {}", e);
                        continue;
                    }
                };
                for instr in &instrs {
                    if let Instruction::Label(name) = instr {
                        labels.insert(name.clone(), ops.new_dynamic_label());
                    }
                }
                instructions_to_asm(&instrs, &mut ops, &labels);
                dynasm!(fun_ops ; .arch x64 ; ret);

                fun_ops.commit().unwrap();
                context.insert_fun(f.name.clone(), f.args.len() as i32);
            },
            ReplEntry::Expr(expr) => {
                let instrs = match compile_to_instructions(&expr, &context) {
                    Ok(instrs) => instrs,
                    Err(e) => {
                        eprintln!("Compile error: {}", e);
                        continue;
                    }
                };
                for instr in &instrs {
                    if let Instruction::Label(name) = instr {
                        labels.insert(name.clone(), ops.new_dynamic_label());
                    }
                }

                let start = ops.offset();
                instructions_to_asm(&instrs, &mut ops, &labels);
                dynasm!(ops ; .arch x64 ; ret);

                ops.commit().unwrap();
                let reader = ops.reader();
                let buf = reader.lock();
                let jitted_fn: extern "C" fn() -> i64 = unsafe { mem::transmute(buf.ptr(start)) };
                let result = jitted_fn();
                println!("{}", from_snek(&result));
            }
        };
        context.clear_stack();
    }
    Ok(())
}

fn typecheck_only(in_name: &str) -> std::io::Result<()> {
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    in_contents = format!("({})", in_contents);
    let parsed_file = match parse(&in_contents) {
        Ok(i) => i,
        Err(_) => panic!("Input parse error"),
    };
    let program = match parse_program(&parsed_file) {
        Ok(p) => p,
        Err(e) => panic!("Parse error: {}", e),
    };

    match typecheck_program(&program, None) {
        Ok(ty) => println!("{}", ty.to_string()),
        Err(e) => panic!("{}", e),
    }
    Ok(())
}

fn invalid_arg() -> ! {
    eprintln!("Error: missing required argument input name");
    eprintln!("-c/-tc (in) (out)\n-e/-te (in) (optional_input)\n-g/-tg (in) (out) (optional_input)\n-i/-ti\n-t (in)");
    std::process::exit(1);
}

fn collect_args(args: &Vec<String>, flag: &String) -> (String, Option<String>, Option<String>) {
    if flag == "-c" || flag == "-tc" {
        let in_name = match args.get(2) {
            Some(name) => name.clone(),
            None => invalid_arg(),
        };
        let out_name = match args.get(3) {
            Some(name) => name.clone(),
            None => invalid_arg(),
        };

        (in_name, Some(out_name), None)

    } else if flag == "-e" || flag == "-te"{
        let in_name = match args.get(2) {
            Some(name) => name.clone(),
            None => invalid_arg(),
        };
        let optional_arg = args.get(3);

        (in_name, None, optional_arg.cloned())

    } else if flag == "-g" || flag =="-tg"{
        let in_name = match args.get(2) {
            Some(name) => name.clone(),
            None => invalid_arg(),
        };
        let out_name = match args.get(3) {
            Some(name) => name.clone(),
            None => invalid_arg(),
        };
        let optional_arg = args.get(4);

        (in_name, Some(out_name), optional_arg.cloned())

    } else if flag == "-t" {
        let in_name = match args.get(2) {
            Some(name) => name.clone(),
            None => invalid_arg(),
        };

        (in_name, None, None)
    } else {
        (String::new(), None, None)
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let valid_flags = vec!["-c", "-e", "-g", "-i", "-tc", "-te", "-tg", "-ti", "-t"];
    let flag = match args.get(1) {
        Some(f) => f,
        None => {
            eprintln!("Specify flag:\n-c (in) (out)\n-e (in) (optional_input)\n-g (in) (out) (optional_input)\n-i");
            return Ok(());
        }
    };
    if !valid_flags.contains(&flag.as_str()) {
        eprintln!("Specify flag:\n-c (in) (out)\n-e (in) (optional_input)\n-g (in) (out) (optional_input)\n-i");
        return Ok(());
    }

    let ops = dynasmrt::x64::Assembler::new().unwrap();

    if flag == "-c" || flag == "-tc" {
        let (in_name, out_name, _) = collect_args(&args, flag);
        compile_aot(&in_name, &out_name.unwrap(), flag =="-tc")?;

    } else if flag == "-e" || flag == "-te" {
        let (in_name, _, optional_arg) = collect_args(&args, flag);
        compile_jit(ops, &in_name, optional_arg.as_ref(), flag == "-te")?;

    } else if flag == "-g" || flag =="-tg" {
        let (in_name, out_name, optional_arg) = collect_args(&args, flag);
        compile_aot(&in_name, &out_name.unwrap(), flag == "-tg")?;
        compile_jit(ops, &in_name, optional_arg.as_ref(), flag == "-tg")?;

    } else if flag == "-i" || flag == "-ti" {
        compile_repl(ops, flag == "-ti")?;

    } else if flag == "-t" {
        let (in_name, _, _) = collect_args(&args, flag);
        typecheck_only(&in_name)?;
    }

    Ok(())
}
