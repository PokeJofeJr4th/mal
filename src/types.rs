use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
pub enum MalObject {
    List(Vec<MalObject>),
    Int(i32),
    Symbol(String),
    Function(Rc<dyn Fn(Vec<MalObject>) -> MalObject>),
}

impl Display for MalObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Symbol(s) => write!(f, "{s}"),
            Self::List(l) => {
                write!(f, "(")?;
                for (i, x) in l.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{x}")?;
                    } else {
                        write!(f, " {x}")?;
                    }
                }
                write!(f, ")")
            }
            Self::Function(_) => f.debug_struct("Function").finish_non_exhaustive(),
        }
    }
}
