
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Id(String),
    Bool(bool),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
    // maybe add more expressions later like field access, array indexing, function calls, etc.
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    And, Or, Eq, Neq, Lt, Le, Gt, Ge, Add, Sub, Mul, Div, Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Not, Neg,
}