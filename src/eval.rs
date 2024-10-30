use std::collections::HashMap;

use crate::types::MalObject;

pub type Env = HashMap<String, MalObject>;

pub fn initial_env() -> Env {
    let mut hm = HashMap::new();
    hm
}

pub fn eval(o: MalObject, env: &mut Env) -> MalObject {
    match o {
        MalObject::List(vec) => todo!(),
        o @ MalObject::Int(_) => o,
        MalObject::Symbol(s) => env.get(&s).unwrap().clone(),
    }
}
