use std::rc::Rc;

use crate::{
    env::{new_env, Env},
    types::MalObject,
};

pub fn eval(mut ast: MalObject, env: Env) -> MalObject {
    loop {
        match ast {
            MalObject::List(vec) if vec.first().is_some_and(|v| v.is_symbol("def!")) => {
                let [_, MalObject::Symbol(symbol), value] = &vec[..] else {
                    panic!()
                };
                let value = eval(value.clone(), env.clone());
                env.borrow_mut().set(symbol.clone(), value.clone());
                return value;
            }
            MalObject::List(vec) if vec.first().is_some_and(|v| v.is_symbol("let*")) => {
                let [_, MalObject::List(lets), body] = &vec[..] else {
                    panic!()
                };
                let inner_env = new_env(env);
                for i in 0..(lets.len() / 2) {
                    let MalObject::Symbol(var) = lets[i * 2].clone() else {
                        panic!()
                    };
                    let value = lets[i * 2 + 1].clone();
                    let value = eval(value, inner_env.clone());
                    inner_env.borrow_mut().set(var, value);
                }
                return eval(body.clone(), inner_env);
            }
            MalObject::List(mut vec) if vec.first().is_some_and(|v| v.is_symbol("do")) => {
                if vec.len() == 1 {
                    return MalObject::nil();
                }
                let last_eval = vec.pop().unwrap();
                for expr in vec.into_iter().skip(1) {
                    eval(expr, env.clone());
                }
                ast = last_eval;
            }
            MalObject::List(vec) if vec.first().is_some_and(|v| v.is_symbol("if")) => {
                let start = eval(vec[1].clone(), env.clone());
                if start.is_symbol("nil") || start.is_symbol("false") {
                    if let Some(else_block) = vec.get(3) {
                        ast = else_block.clone();
                    } else {
                        return MalObject::nil();
                    }
                } else {
                    ast = vec[2].clone();
                }
            }
            MalObject::List(vec) if vec.first().is_some_and(|v| v.is_symbol("fn*")) => {
                let [_, MalObject::List(params), body] = &vec[..] else {
                    panic!()
                };
                let params: Vec<_> = params
                    .iter()
                    .map(|p| {
                        let MalObject::Symbol(s) = p else { panic!() };
                        s.clone()
                    })
                    .collect();
                let body = body.clone();
                return MalObject::Function(Rc::new(move |args, env| {
                    let inner_env = new_env(env);
                    assert!(args.len() == params.len());
                    for (arg, param) in args.into_iter().zip(params.iter().cloned()) {
                        inner_env.borrow_mut().set(param, arg);
                    }
                    eval(body.clone(), inner_env)
                }));
            }
            MalObject::List(vec) => {
                let mut args: Vec<_> = vec.into_iter().map(|v| eval(v, env.clone())).collect();
                let MalObject::Function(func) = args.remove(0) else {
                    panic!()
                };
                return func(args, env);
            }
            o @ (MalObject::Int(_) | MalObject::Function(_)) => return o,
            MalObject::Symbol(sym) if &*sym == "true" || &*sym == "false" || &*sym == "nil" => {
                return MalObject::Symbol(sym);
            }
            MalObject::Symbol(s) => return (*env.borrow()).get(&s).unwrap(),
        }
    }
}
