#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
