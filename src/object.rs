use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Void,
    Integer(i64),
    Float(f64),
    Bool(bool),
    Operator(Op),
    Lambda(Vec<String>, Vec<Object>),
    Condition,
    List(Vec<Object>),
    Keyword(String),
    Name(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Float(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Operator(o) => write!(f, "{}", o),
            Object::Lambda(vs, vo) => {
                write!(f, "lambda [")?;
                for s in vs {
                    write!(f, " {}", s)?;
                }
                write!(f, " ] (")?;
                for o in vo {
                    write!(f, " {}", o)?;
                }
                write!(f, " )")
            }
            Object::Condition => write!(f, "If"),
            Object::List(l) => {
                write!(f, "(")?;
                for o in l {
                    write!(f, " {}", o)?;
                }
                write!(f, " )")
            }
            Object::Keyword(s) => write!(f, "{}", s),
            Object::Name(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Greater,
    Smaller,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Eq => write!(f, "=="),
            Op::NotEq => write!(f, "!="),
            Op::Greater => write!(f, ">"),
            Op::Smaller => write!(f, "<"),
        }
    }
}
