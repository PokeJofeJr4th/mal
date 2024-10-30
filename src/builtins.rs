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
