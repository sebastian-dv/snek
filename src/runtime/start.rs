#![allow(dead_code)]

use std::env;

#[link(name = "our_code")]
extern "C" {
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    let err = match errcode {
        1 => "invalid argument",
        2 => "overflow",
        3 => "bad cast",
        _ => "unknown error",
    };
    eprintln!("ERROR code {}: {}", errcode, err);
    std::process::exit(1);
}

pub extern "C" fn snek_error_repl(errcode: i64) {
    let err = match errcode {
        1 => "invalid argument",
        2 => "overflow",
        3 => "bad cast",
        _ => "unknown error",
    };
    eprintln!("ERROR code {}: {}", errcode, err);
}

#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(n: i64) {
    let out = if n == 3 {
        "true".to_string()
    } else if n == 1 {
        "false".to_string()
    } else if n == 5 {
        "Runtime Error".to_string()
    } else {
        let num = n / 2;
        (num).to_string()
    };
    println!("{}", out);
}

fn parse_input(input: &str) -> u64 {
    match input {
        "true" => 3,
        "false" => 1,
        _ => {
            match input.parse::<u64>() {
                Ok(num) => num << 1,
                Err(_) => 1, // return false otherwise
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: u64 = unsafe { our_code_starts_here(input) };
    snek_print(i as i64);
}
