#[derive(Debug)]
pub enum Expr<'a> {
    MathExpr(Math),
    FunCallExpr(FunCall<'a>),
}

// MATH EXPRESSION

#[derive(Debug)]
pub struct Math {
    pub left: i64,
    pub right: i64,
    pub operator: MathOperator
}

#[derive(Debug)]
pub enum MathOperator {
    OP_ADD,	// Addition
    OP_SUB,	// Subtraction
    OP_DIV,	// Division
    OP_MULT,	// Multiplication
    OP_MOD,	// Modulo
}

// FUNCTIONS

#[derive(Debug)]
pub struct FunCall<'a> {
    name: &'a str,
    params: Vec<FunParam<'a>>,
}

#[derive(Debug)]
pub struct FunDefention<'a> {
    name: &'a str,
    params: Vec<FunParam<'a>>,
    contents: Vec<Expr<'a>>,
    return_value: Expr<'a>,
}

#[derive(Debug)]
pub struct FunParam<'a> {
    name: &'a str,
}
