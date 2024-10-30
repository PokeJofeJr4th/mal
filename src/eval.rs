use crate::{
    env::{sub_env, Env},
    types::MalObject,
};

pub fn eval(o: MalObject, env: Env) -> MalObject {
    match o {
        MalObject::List(vec) if vec.first().is_some_and(|v| v.is_symbol("def!")) => {
            let [_, MalObject::Symbol(symbol), value] = &vec[..] else {
                panic!()
            };
            let value = eval(value.clone(), env.clone());
            env.borrow_mut().set(symbol.clone(), value.clone());
            value
        }
        MalObject::List(vec) if vec.first().is_some_and(|v| v.is_symbol("let*")) => {
            let [_, MalObject::List(lets), body] = &vec[..] else {
                panic!()
            };
            let inner_env = sub_env(env);
            for i in 0..(lets.len() / 2) {
                let MalObject::Symbol(var) = lets[i * 2].clone() else {
                    panic!()
                };
                let value = lets[i * 2 + 1].clone();
                let value = eval(value, inner_env.clone());
                inner_env.borrow_mut().set(var, value);
            }
            eval(body.clone(), inner_env)
        }
        MalObject::List(vec) => {
            let mut args: Vec<_> = vec.into_iter().map(|v| eval(v, env.clone())).collect();
            let MalObject::Function(func) = args.remove(0) else {
                panic!()
            };
            func(args)
        }
        o @ (MalObject::Int(_) | MalObject::Function(_)) => o,
        MalObject::Symbol(sym) if &*sym == "true" || &*sym == "false" || &*sym == "nil" => {
            MalObject::Symbol(sym)
        }
        MalObject::Symbol(s) => (*env.borrow()).get(&s).unwrap(),
    }
}
