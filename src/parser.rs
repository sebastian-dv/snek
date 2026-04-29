use crate::{
    ast::{Program, Expr, Op1, Op2, FunDef, ReplEntry, Type},
};
use sexp::*;
use sexp::Atom::*;

pub fn parse_program(s: &Sexp) -> Result<Program, String> {
    match s {
        Sexp::List(vec) => {
            match &vec[..] {
                [body] => {
                    Ok(Program { defs: Vec::new(), main: Box::new(parse_expr(body)?) })
                }
                [funcs @ .., body] => {
                    let mut seen_names = Vec::new();
                    let mut fundefs = Vec::new();
                    for func in funcs {
                        let fundef = parse_def(func)?;
                        if seen_names.contains(&fundef.name) {
                            return Err("duplicate function name".to_string());
                        }
                        seen_names.push(fundef.name.clone());
                        fundefs.push(fundef);
                    }
                    Ok(Program { defs: fundefs, main: Box::new(parse_expr(body)?) })
                },
                _ => Err("missing program".to_string()),
            }
        }
        _ => Err("invalid program".to_string()),
    }
}

fn parse_def(def: &Sexp) -> Result<FunDef, String> {
    match def {
        Sexp::List(list) => match &list[..] {
            [Sexp::Atom(S(fun)), Sexp::List(sign), Sexp::Atom(S(arrow)), ret, body]
                if fun == "fun" && arrow == "->" => {
                let (fun_name, fun_args, fun_arg_types) = parse_annotated_signature(sign)?;
                let ret_type = parse_type(ret)?;
                let fun_body = Box::new(parse_expr(body)?);
                if contains_input(&fun_body) {
                    return Err("cannot use input in function body".to_string());
                }
                Ok(FunDef {
                    name: fun_name,
                    args: fun_args,
                    arg_types: fun_arg_types,
                    ret_type: Some(ret_type),
                    body: fun_body
                })
            },
            [Sexp::Atom(S(fun)), Sexp::List(sign), body] if fun == "fun" => {
                let (fun_name, fun_args) = parse_signature(sign)?;
                let fun_body = Box::new(parse_expr(body)?);
                if contains_input(&fun_body) {
                    return Err("cannot use input in function body".to_string());
                }
                Ok(FunDef {
                    name: fun_name,
                    args: fun_args,
                    arg_types: Vec::new(),
                    ret_type: None,
                    body: fun_body
                })
            },
            _ => Err("expected function: (fun (name params..) body)".to_string()),
        },
        _ => Err("invalid function definition".to_string()),
    }
}

fn parse_type(s: &Sexp) -> Result<Type, String> {
    match s {
        Sexp::Atom(S(t)) => match t.as_str() {
            "Num" => Ok(Type::Num),
            "Bool" => Ok(Type::Bool),
            "Nothing" => Ok(Type::Nothing),
            "Any" => Ok(Type::Any),
            _ => Err(format!("invalid type: {}", t)),
        },
        _ => Err("invalid type".to_string()),
    }
}
fn parse_annotated_signature(signature: &Vec<Sexp>) -> Result<(String, Vec<String>, Vec<Type>), String> {
    match &signature[..] {
        [Sexp::Atom(S(fun_name)), args @ ..] => {
            if !valid_id(&fun_name) {
                return Err("invalid function name".to_string());
            }
            let mut fun_args = Vec::new();
            let mut fun_arg_types = Vec::new();
            for arg in args {
                match arg {
                    Sexp::List(parts) => match &parts[..] {
                        [Sexp::Atom(S(name)), Sexp::Atom(S(colon)), type_sexp] if colon == ":" => {
                            if !valid_id(&name) {
                                return Err("invalid function parameter".to_string());
                            }
                            if fun_args.contains(name) {
                                return Err("duplicate function parameter".to_string());
                            }
                            let ty = parse_type(type_sexp)?;
                            fun_args.push(name.to_string());
                            fun_arg_types.push(ty);
                        },
                        _ => return Err("invalid function argument annotation".to_string()),
                    },
                    _ => return Err("invalid function argument".to_string()),
                }
            }
            Ok((fun_name.clone(), fun_args, fun_arg_types))
        },
        _ => Err("invalid function signature".to_string()),
    }
}

fn parse_signature(signature: &Vec<Sexp>) -> Result<(String, Vec<String>), String> {
    match &signature[..] {
        [Sexp::Atom(S(fun_name)), args @ ..] => {
            if !valid_id(&fun_name) {
                return Err("invalid function name".to_string());
            }
            let mut fun_args = Vec::new();
            for arg in args {
                match arg {
                    Sexp::Atom(S(a)) => {
                        if !valid_id(&a) {
                            return Err("invalid function parameter".to_string());
                        }
                        if fun_args.contains(a) {
                            return Err("duplicate function parameter".to_string());
                        }
                        fun_args.push(a.to_string());
                    },
                    _ => return Err("invalid function argument".to_string()),
                }
            }
            Ok((fun_name.clone(), fun_args))
        },
        _ => Err("invalid function signature".to_string()),
    }
}

fn parse_expr(s: &Sexp) -> Result<Expr, String> {
    match s {
        Sexp::Atom(I(n)) => Ok(Expr::Num(i64::try_from(*n).unwrap())),
        Sexp::Atom(S(s)) => {
            if s == "true" {
                Ok(Expr::Bool(true))
            } else if s == "false" {
                Ok(Expr::Bool(false))
            } else if s == "input" {
                Ok(Expr::Input)
            } else if valid_id(&s) {
                Ok(Expr::Id(s.to_string()))
            } else {
                Err(format!("invalid ID: can't use reserved word(s): {}", s))
            }
        },
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(I(n))] => Ok(Expr::Num(i64::try_from(*n).unwrap())),
                [Sexp::Atom(S(s))] => {
                    if s == "true" {
                        Ok(Expr::Bool(true))
                    } else if s == "false" {
                        Ok(Expr::Bool(false))
                    } else if s == "input" {
                        Ok(Expr::Input)
                    } else if valid_id(&s) {
                        Ok(Expr::Id(s.to_string()))
                    } else {
                        Err(format!("invalid ID: an't use reserved word(s): {}", s))
                    }
                },
                [Sexp::Atom(S(op)), e] if op == "add1" => Ok(Expr::UnOp(Op1::Add1, Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Ok(Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), e] if op == "negate" => Ok(Expr::UnOp(Op1::Negate, Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), e] if op == "isnum" => Ok(Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), e] if op == "isbool" => Ok(Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), e] if op == "print" => Ok(Expr::UnOp(Op1::Print, Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), e1, e2] if op == "+" => Ok(Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)?), Box::new(parse_expr(e2)?))),
                [Sexp::Atom(S(op)), e1, e2] if op == "-" => Ok(Expr::BinOp(Op2::Minus, Box::new(parse_expr(e2)?), Box::new(parse_expr(e1)?))),
                [Sexp::Atom(S(op)), e1, e2] if op == "*" => Ok(Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)?), Box::new(parse_expr(e2)?))),
                [Sexp::Atom(S(op)), e1, e2] if op == "<" => Ok(Expr::BinOp(Op2::Less, Box::new(parse_expr(e2)?), Box::new(parse_expr(e1)?))),
                [Sexp::Atom(S(op)), e1, e2] if op == ">" => Ok(Expr::BinOp(Op2::Greater, Box::new(parse_expr(e2)?), Box::new(parse_expr(e1)?))),
                [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Ok(Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(e2)?), Box::new(parse_expr(e1)?))),
                [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Ok(Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(e2)?), Box::new(parse_expr(e1)?))),
                [Sexp::Atom(S(op)), e1, e2] if op == "=" => Ok(Expr::BinOp(Op2::Equal, Box::new(parse_expr(e2)?), Box::new(parse_expr(e1)?))),
                [Sexp::Atom(S(op)), Sexp::List(bindings), body] if op == "let" => {
                    let mut parsed_binds: Vec<(String, Expr)> = Vec::new();
                    for bind in bindings {
                        match parse_bind(bind) {
                            Ok(b) => parsed_binds.push(b),
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(Expr::Let(parsed_binds, Box::new(parse_expr(body)?)))
                },
                [Sexp::Atom(S(op)), e1, e2, e3] if op == "if" => Ok(Expr::If(Box::new(parse_expr(e1)?), Box::new(parse_expr(e2)?), Box::new(parse_expr(e3)?))),
                [Sexp::Atom(S(op)), e] if op == "loop" => Ok(Expr::Loop(Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), e] if op == "break" => Ok(Expr::Break(Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), Sexp::Atom(S(id)), e] if op == "set!" => Ok(Expr::Set(id.to_string(), Box::new(parse_expr(e)?))),
                [Sexp::Atom(S(op)), rest @ ..] if op == "block" => {
                    let exprs = rest
                        .iter()
                        .map(|e| parse_expr(e))
                        .collect::<Result<Vec<Expr>, String>>()?;
                    Ok(Expr::Block(exprs))
                },
                [Sexp::Atom(S(name)), args @ ..] if valid_id(name) => {
                    let parsed_args = args
                        .iter()
                        .map(|e| parse_expr(e))
                        .collect::<Result<Vec<Expr>, String>>()?;
                    Ok(Expr::Call(name.clone(), parsed_args))
                },
                [Sexp::Atom(S(fun)), Sexp::List(_), _] if fun == "fun" => {
                    Err("missing body".to_string())
                },
                [Sexp::Atom(S(op)), type_sexp, e] if op == "cast" => {
                    let ty = parse_type(type_sexp)?;
                    Ok(Expr::Cast(ty, Box::new(parse_expr(e)?)))
                },
                _ => Err("invalid expression".to_string()),
            }
        },
        _ => Err("invalid expression".to_string()),
    }
}

fn contains_input(e: &Expr) -> bool {
    match e {
        Expr::Input => true,
        Expr::Let(bindings, body) => {
            bindings.iter().any(|(_, expr)| contains_input(expr)) || contains_input(body)
        },
        Expr::UnOp(_, e) => contains_input(e),
        Expr::BinOp(_, e1, e2) => contains_input(e1) || contains_input(e2),
        Expr::If(cond, then, els) => {
            contains_input(cond) || contains_input(then) || contains_input(els)
        },
        Expr::Loop(e) => contains_input(e),
        Expr::Break(e) => contains_input(e),
        Expr::Set(_, e) => contains_input(e),
        Expr::Block(exprs) => exprs.iter().any(|e| contains_input(e)),
        Expr::Call(_, args) => args.iter().any(|e| contains_input(e)),
        _ => false,
    }
}

pub fn parse_repl_entry(s: &Sexp) -> Result<ReplEntry, String> {
    match s {
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), Sexp::Atom(S(id)), expr] if op == "define" => {
                    let parsed_expr = parse_expr(expr)?;
                    if valid_id(id) {
                        Ok(ReplEntry::Define(id.to_string(), Box::new(parsed_expr)))
                    } else {
                        Err("invalid ID: can't use reserved words".to_string())
                    }
                },
                [Sexp::Atom(S(fun)), Sexp::List(sign), body] if fun == "fun" => {
                    let (fun_name, fun_args) = parse_signature(sign)?;
                    let fun_body = Box::new(parse_expr(body)?);
                    if contains_input(&fun_body) {
                        return Err("cannot use input in function body".to_string());
                    }
                    Ok(ReplEntry::FunDef(FunDef {
                        name: fun_name,
                        args: fun_args,
                        arg_types: Vec::new(),
                        ret_type: None,
                        body: fun_body 
                    }))
                },
                [Sexp::Atom(S(fun)), Sexp::List(sign), Sexp::Atom(S(arrow)), ret, body] 
                    if fun == "fun" && arrow == "->" => {
                    let (fun_name, fun_args, fun_arg_types) = parse_annotated_signature(sign)?;
                    let ret_type = parse_type(ret)?;
                    let fun_body = Box::new(parse_expr(body)?);
                    if contains_input(&fun_body) {
                        return Err("cannot use input in function body".to_string());
                    }
                    Ok(ReplEntry::FunDef(FunDef {
                        name: fun_name,
                        args: fun_args,
                        arg_types: fun_arg_types,
                        ret_type: Some(ret_type),
                        body: fun_body
                    }))
                },
                _ => {
                    Ok(ReplEntry::Expr(Box::new(parse_expr(s)?)))
                }
            }
        },
        _ => {
            Ok(ReplEntry::Expr(Box::new(parse_expr(s)?)))
        }
    }
}

fn parse_bind(s: &Sexp) -> Result<(String, Expr), String> {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(id)), e] if valid_id(&id) => Ok((id.to_string(), parse_expr(e)?)),
            _ => Err("parse error: invalid binding".to_string()),
        },
        _ => Err("parse error: invalid binding".to_string()),
    }
}

fn valid_id(id: &str) -> bool {
    let reserved = ["let", "isnum", "isbool", "add1", "sub1", "+", "-", "*", "<", ">", "define",
        "<=", ">=", "=" , "true", "false", "input", "set!", "if", "block", "loop", "break",
        "fun", "print", "cast"];
    !reserved.contains(&id)
}
