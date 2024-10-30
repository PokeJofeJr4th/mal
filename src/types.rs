use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
pub enum MalObject {
    List(Vec<MalObject>),
    Int(i32),
    Symbol(String),
    Function(Rc<dyn Fn(Vec<MalObject>) -> MalObject>),
}

impl MalObject {
    pub fn is_symbol(&self, s: &str) -> bool {
        let Self::Symbol(s2) = self else { return false };
        s == s2
    }

    pub fn nil() -> Self {
        Self::Symbol("nil".to_string())
    }
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

impl PartialEq for MalObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => *a == *b,
            (Self::Symbol(a), Self::Symbol(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            _ => false,
        }
    }
}
