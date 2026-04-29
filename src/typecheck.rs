use crate::ast::{Program, Expr, Op1, Op2, FunDef, Type};
use im::HashMap;

#[derive(Clone, Debug)]
pub struct TypeEnv {
    vars: HashMap<String, Type>,
    funs: HashMap<String, (Vec<Type>, Type)>,
}

impl TypeEnv {
    pub fn new() -> Self {
        TypeEnv {
            vars: HashMap::new(),
            funs: HashMap::new(),
        }
    }

    pub fn with_var(&self, name: String, ty: Type) -> Self {
        TypeEnv {
            vars: self.vars.update(name, ty),
            funs: self.funs.clone(),
        }
    }

    pub fn with_fun(&self, name: String, arg_types: Vec<Type>, ret_type: Type) -> Self {
        TypeEnv {
            vars: self.vars.clone(),
            funs: self.funs.update(name, (arg_types, ret_type)),
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&Type> {
        self.vars.get(name)
    }

    pub fn get_fun(&self, name: &str) -> Option<&(Vec<Type>, Type)> {
        self.funs.get(name)
    }
}

pub fn is_subtype(t1: &Type, t2: &Type) -> bool {
    match (t1, t2) {
        (_, Type::Any) => true,
        (Type::Nothing, _) => true,
        (Type::Num, Type::Num) => true,
        (Type::Bool, Type::Bool) => true,
        _ => false,
    }
}

pub fn union(t1: &Type, t2: &Type) -> Type {
    match (t1, t2) {
        (Type::Any, _) | (_, Type::Any) => Type::Any,
        (Type::Nothing, t) | (t, Type::Nothing) => t.clone(),
        (Type::Num, Type::Num) => Type::Num,
        (Type::Bool, Type::Bool) => Type::Bool,
        (Type::Num, Type::Bool) | (Type::Bool, Type::Num) => Type::Any,
    }
}

pub fn typecheck_program(prog: &Program, input_type: Option<Type>) -> Result<Type, String> {
    let mut env = TypeEnv::new();
    for def in &prog.defs {
        let (arg_types, ret_type) = get_function_signature(def);
        env = env.with_fun(def.name.clone(), arg_types, ret_type);
    }

    for def in &prog.defs {
        typecheck_fundef(def, &env)?;
    }

    let main_env = if let Some(ty) = input_type {
        env.with_var("input".to_string(), ty)
    } else {
        env
    };
    typecheck_expr(&prog.main, &main_env)
}

fn get_function_signature(def: &FunDef) -> (Vec<Type>, Type) {
    match &def.ret_type {
        Some(ret_ty) => {
            let arg_types = def.arg_types.iter().map(|t| t.clone()).collect();
            (arg_types, ret_ty.clone())
        }
        None => {
            let arg_types = vec![Type::Any; def.args.len()];
            (arg_types, Type::Any)
        }
    }
}

fn typecheck_fundef(def: &FunDef, env: &TypeEnv) -> Result<(), String> {
    let mut body_env = env.clone();
    match &def.ret_type {
        Some(expected_ret) => {
            for (arg_name, arg_type) in def.args.iter().zip(def.arg_types.iter()) {
                body_env = body_env.with_var(arg_name.clone(), arg_type.clone());
            }
            let body_type = typecheck_expr(&def.body, &body_env)?;
            if !is_subtype(&body_type, expected_ret) {
                return Err(format!("Type error: function {} body has type {} but expected {}",
                    def.name, body_type.to_string(), expected_ret.to_string()
                ));
            }
        }
        None => {
            for arg_name in &def.args {
                body_env = body_env.with_var(arg_name.clone(), Type::Any);
            }
            let body_type = typecheck_expr(&def.body, &body_env)?;
            if !is_subtype(&body_type, &Type::Any) {
                return Err(format!("Type error: unannotated function {} body has type {}",
                    def.name, body_type.to_string()
                ));
            }
        }
    }
    Ok(())
}

pub fn typecheck_expr(e: &Expr, env: &TypeEnv) -> Result<Type, String> {
    match e {
        Expr::Num(_) => Ok(Type::Num),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Input => {
            match env.get_var("input") {
                Some(ty) => Ok(ty.clone()),
                None => Ok(Type::Any),
            }
        }
        Expr::Id(name) => {
            match env.get_var(name) {
                Some(ty) => Ok(ty.clone()),
                None => Err(format!("Type error: unbound variable {}", name)),
            }
        }
        Expr::Let(bindings, body) => {
            let mut curr_env = env.clone();
            for (name, expr) in bindings {
                let expr_type = typecheck_expr(expr, &curr_env)?;
                curr_env = curr_env.with_var(name.clone(), expr_type);
            }
            typecheck_expr(body, &curr_env)
        }
        Expr::UnOp(op, e) => {
            let e_type = typecheck_expr(e, env)?;
            match op {
                Op1::Add1 | Op1::Sub1 | Op1::Negate => {
                    if !is_subtype(&e_type, &Type::Num) {
                        return Err(format!("Type error: {:?} expects Num but got {}",
                            op, e_type.to_string()
                        ));
                    }
                    Ok(Type::Num)
                }
                Op1::IsNum | Op1::IsBool => {
                    Ok(Type::Bool)
                }
                Op1::Print => {
                    Ok(e_type)
                }
            }
        }
        Expr::BinOp(op, e1, e2) => {
            let t1 = typecheck_expr(e1, env)?;
            let t2 = typecheck_expr(e2, env)?;

            match op {
                Op2::Plus | Op2::Minus | Op2::Times => {
                    if !is_subtype(&t1, &Type::Num) {
                        return Err(format!("Type error: {:?} expects Num for first operand but got {}",
                            op, t1.to_string()
                        ));
                    }
                    if !is_subtype(&t2, &Type::Num) {
                        return Err(format!("Type error: {:?} expects Num for second operand but got {}",
                            op, t2.to_string()
                        ));
                    }
                    Ok(Type::Num)
                }
                Op2::Less | Op2::Greater | Op2::LessEqual | Op2::GreaterEqual => {
                    if !is_subtype(&t1, &Type::Num) {
                        return Err(format!("Type error: {:?} expects Num for first operand but got {}",
                            op, t1.to_string()
                        ));
                    }
                    if !is_subtype(&t2, &Type::Num) {
                        return Err(format!("Type error: {:?} expects Num for second operand but got {}",
                            op, t2.to_string()
                        ));
                    }
                    Ok(Type::Bool)
                }
                Op2::Equal => {
                    if is_subtype(&t1, &Type::Num) && is_subtype(&t2, &Type::Num) {
                        Ok(Type::Bool)
                    } else if is_subtype(&t1, &Type::Bool) && is_subtype(&t2, &Type::Bool) {
                        Ok(Type::Bool)
                    } else {
                        Err(format!("Type error: = expects both operands to be Bools or Nums, got {} and {}",
                            t1.to_string(), t2.to_string()
                        ))
                    }
                }
            }
        }
        Expr::If(cond, then_e, else_e) => {
            let cond_type = typecheck_expr(cond, env)?;
            if !is_subtype(&cond_type, &Type::Bool) {
                return Err(format!("Type error: if condition expects Bool but got {}",
                    cond_type.to_string()
                ));
            }
            let then_type = typecheck_expr(then_e, env)?;
            let else_type = typecheck_expr(else_e, env)?;
            Ok(union(&then_type, &else_type))
        }
        Expr::Loop(body) => {
            let break_types = collect_break_types(body, env)?;
            if break_types.is_empty() {
                Ok(Type::Nothing)
            } else {
                Ok(break_types.into_iter().reduce(|acc, t| union(&acc, &t)).unwrap())
            }
        }
        Expr::Break(e) => {
            let _ = typecheck_expr(e, env)?;
            Ok(Type::Nothing)
        }
        Expr::Set(name, e) => {
            let expr_type = typecheck_expr(e, env)?;
            match env.get_var(name) {
                Some(var_type) => {
                    if !is_subtype(&expr_type, var_type) {
                        return Err(format!("Type error: cannot assign {} to variable {} of type {}",
                            expr_type.to_string(), name, var_type.to_string()
                        ));
                    }
                    Ok(expr_type)
                }
                None => Err(format!("Type error: unbound variable {}", name)),
            }
        }
        Expr::Block(exprs) => {
            if exprs.is_empty() {
                return Err("Type error: empty block".to_string());
            }
            let mut last_type = Type::Nothing;
            for expr in exprs {
                last_type = typecheck_expr(expr, env)?;
            }
            Ok(last_type)
        }
        Expr::Call(name, args) => {
            match env.get_fun(name) {
                Some((arg_types, ret_type)) => {
                    if args.len() != arg_types.len() {
                        return Err(format!("Type error: function {} expects {} arguments but got {}",
                            name, arg_types.len(), args.len()
                        ));
                    }
                    for (i, (arg, expected_type)) in args.iter().zip(arg_types.iter()).enumerate() {
                        let arg_type = typecheck_expr(arg, env)?;
                        if !is_subtype(&arg_type, expected_type) {
                            return Err(format!("Type error: argument {} to function {} has type {} but expected {}",
                                i, name, arg_type.to_string(), expected_type.to_string()
                            ));
                        }
                    }
                    Ok(ret_type.clone())
                }
                None => Err(format!("Type error: unknown function {}", name)),
            }
        }
        Expr::Cast(target_type, e) => {
            let _ = typecheck_expr(e, env)?;
            Ok(target_type.clone())
        }
    }
}

fn collect_break_types(e: &Expr, env: &TypeEnv) -> Result<Vec<Type>, String> {
    match e {
        Expr::Break(inner) => {
            let t = typecheck_expr(inner, env)?;
            Ok(vec![t])
        }
        Expr::Let(bindings, body) => {
            let mut curr_env = env.clone();
            let mut types = Vec::new();
            for (name, expr) in bindings {
                types.extend(collect_break_types(expr, &curr_env)?);
                let expr_type = typecheck_expr(expr, &curr_env)?;
                curr_env = curr_env.with_var(name.clone(), expr_type);
            }
            types.extend(collect_break_types(body, &curr_env)?);
            Ok(types)
        }
        Expr::UnOp(_, e) => collect_break_types(e, env),
        Expr::BinOp(_, e1, e2) => {
            let mut types = collect_break_types(e1, env)?;
            types.extend(collect_break_types(e2, env)?);
            Ok(types)
        }
        Expr::If(cond, then_e, else_e) => {
            let mut types = collect_break_types(cond, env)?;
            types.extend(collect_break_types(then_e, env)?);
            types.extend(collect_break_types(else_e, env)?);
            Ok(types)
        }
        Expr::Loop(_) => {
            Ok(Vec::new())
        }
        Expr::Set(_, e) => collect_break_types(e, env),
        Expr::Block(exprs) => {
            let mut types = Vec::new();
            for expr in exprs {
                types.extend(collect_break_types(expr, env)?);
            }
            Ok(types)
        }
        Expr::Call(_, args) => {
            let mut types = Vec::new();
            for arg in args {
                types.extend(collect_break_types(arg, env)?);
            }
            Ok(types)
        }
        Expr::Cast(_, e) => collect_break_types(e, env),
        _ => Ok(Vec::new()),
    }
}
