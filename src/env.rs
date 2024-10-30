#![allow(clippy::module_name_repetitions)]
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{builtins, types::MalObject};

pub struct EnvData {
    outer: Option<Env>,
    data: HashMap<String, MalObject>,
}

impl EnvData {
    pub fn get(&self, k: &str) -> Option<MalObject> {
        if let Some(x) = self.data.get(k) {
            Some(x.clone())
        } else {
            self.outer.as_ref()?.borrow().get(k)
        }
    }

    pub fn set(&mut self, k: String, v: MalObject) {
        self.data.insert(k, v);
    }
}

pub type Env = Rc<RefCell<EnvData>>;

pub fn new_env(data: HashMap<String, MalObject>) -> Env {
    Rc::new(RefCell::new(EnvData { outer: None, data }))
}

pub fn sub_env(env: Env) -> Env {
    Rc::new(RefCell::new(EnvData {
        outer: Some(env),
        data: HashMap::new(),
    }))
}

pub fn initial_env() -> Env {
    let mut hm = HashMap::new();
    hm.insert("+".to_string(), MalObject::Function(Rc::new(builtins::add)));
    hm.insert("-".to_string(), MalObject::Function(Rc::new(builtins::sub)));
    hm.insert("*".to_string(), MalObject::Function(Rc::new(builtins::mul)));
    new_env(hm)
}
