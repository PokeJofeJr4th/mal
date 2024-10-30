use std::{collections::HashMap, rc::Rc};

use crate::{builtins, types::MalObject};

pub type Env = HashMap<String, MalObject>;

pub fn initial_env() -> Env {
    let mut hm = HashMap::new();
    hm.insert("+".to_string(), MalObject::Function(Rc::new(builtins::add)));
    hm.insert("-".to_string(), MalObject::Function(Rc::new(builtins::sub)));
    hm.insert("*".to_string(), MalObject::Function(Rc::new(builtins::mul)));
    hm
}

pub fn eval(o: MalObject, env: &mut Env) -> MalObject {
    match o {
        MalObject::List(vec) => {
            let mut args: Vec<_> = vec.into_iter().map(|v| eval(v, env)).collect();
            let MalObject::Function(func) = args.remove(0) else {
                panic!()
            };
            func(args)
        }
        o @ (MalObject::Int(_) | MalObject::Function(_)) => o,
        MalObject::Symbol(s) => env.get(&s).unwrap().clone(),
    }
}
