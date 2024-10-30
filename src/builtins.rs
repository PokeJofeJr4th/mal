#![allow(clippy::needless_pass_by_value)]
use crate::types::MalObject;

pub fn add(values: Vec<MalObject>) -> MalObject {
    let mut sum = 0;
    for v in values {
        let MalObject::Int(i) = v else { todo!() };
        sum += i;
    }
    MalObject::Int(sum)
}

pub fn sub(values: Vec<MalObject>) -> MalObject {
    let mut sub = 0;
    for v in values {
        let MalObject::Int(i) = v else { todo!() };
        sub = -i - sub;
    }
    MalObject::Int(sub)
}

pub fn mul(values: Vec<MalObject>) -> MalObject {
    let mut mul = 1;
    for v in values {
        let MalObject::Int(i) = v else { todo!() };
        mul *= i;
    }
    MalObject::Int(mul)
}

pub fn count(values: Vec<MalObject>) -> MalObject {
    match values.first() {
        None => MalObject::Int(0),
        Some(MalObject::List(l)) => MalObject::Int(l.len() as i32),
        Some(_) => MalObject::Int(1),
    }
}

pub fn is_list(values: Vec<MalObject>) -> MalObject {
    match values.first() {
        Some(MalObject::List(_)) => MalObject::Symbol("true".to_string()),
        _ => MalObject::Symbol("false".to_string()),
    }
}

pub fn is_empty(values: Vec<MalObject>) -> MalObject {
    match values.first() {
        Some(MalObject::List(l)) if l.is_empty() => MalObject::Symbol("true".to_string()),
        _ => MalObject::Symbol("false".to_string()),
    }
}

pub fn eq(values: Vec<MalObject>) -> MalObject {
    if values.is_empty() {
        return MalObject::Symbol("true".to_string());
    }
    for v in &values[1..] {
        if v != &values[0] {
            return MalObject::Symbol("false".to_string());
        }
    }
    MalObject::Symbol("true".to_string())
}

pub fn lt(values: Vec<MalObject>) -> MalObject {
    let [MalObject::Int(a), MalObject::Int(b)] = &values[..] else {
        return MalObject::Symbol("false".to_string());
    };
    if *a < *b {
        MalObject::Symbol("true".to_string())
    } else {
        MalObject::Symbol("false".to_string())
    }
}

pub fn gt(values: Vec<MalObject>) -> MalObject {
    let [MalObject::Int(a), MalObject::Int(b)] = &values[..] else {
        return MalObject::Symbol("false".to_string());
    };
    if *a > *b {
        MalObject::Symbol("true".to_string())
    } else {
        MalObject::Symbol("false".to_string())
    }
}

pub fn le(values: Vec<MalObject>) -> MalObject {
    let [MalObject::Int(a), MalObject::Int(b)] = &values[..] else {
        return MalObject::Symbol("false".to_string());
    };
    if *a <= *b {
        MalObject::Symbol("true".to_string())
    } else {
        MalObject::Symbol("false".to_string())
    }
}

pub fn ge(values: Vec<MalObject>) -> MalObject {
    let [MalObject::Int(a), MalObject::Int(b)] = &values[..] else {
        return MalObject::Symbol("false".to_string());
    };
    if *a >= *b {
        MalObject::Symbol("true".to_string())
    } else {
        MalObject::Symbol("false".to_string())
    }
}
