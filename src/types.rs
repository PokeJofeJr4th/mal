use std::fmt::Display;

#[derive(Clone)]
pub enum MalObject {
    List(Vec<MalObject>),
    Int(i32),
    Symbol(String),
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
        }
    }
}
