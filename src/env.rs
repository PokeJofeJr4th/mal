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

pub fn new_env(env: Env) -> Env {
    Rc::new(RefCell::new(EnvData {
        outer: Some(env),
        data: HashMap::new(),
    }))
}

pub fn initial_env() -> Env {
    let mut data = HashMap::new();
    data.insert("+".to_string(), MalObject::Function(Rc::new(builtins::add)));
    data.insert("-".to_string(), MalObject::Function(Rc::new(builtins::sub)));
    data.insert("*".to_string(), MalObject::Function(Rc::new(builtins::mul)));
    data.insert("=".to_string(), MalObject::Function(Rc::new(builtins::eq)));
    data.insert("<".to_string(), MalObject::Function(Rc::new(builtins::lt)));
    data.insert(">".to_string(), MalObject::Function(Rc::new(builtins::gt)));
    data.insert("<=".to_string(), MalObject::Function(Rc::new(builtins::le)));
    data.insert(">=".to_string(), MalObject::Function(Rc::new(builtins::ge)));
    data.insert(
        "list".to_string(),
        MalObject::Function(Rc::new(MalObject::List)),
    );
    data.insert(
        "count".to_string(),
        MalObject::Function(Rc::new(builtins::count)),
    );
    data.insert(
        "list?".to_string(),
        MalObject::Function(Rc::new(builtins::is_list)),
    );
    data.insert(
        "empty?".to_string(),
        MalObject::Function(Rc::new(builtins::is_empty)),
    );

    Rc::new(RefCell::new(EnvData { outer: None, data }))
}
