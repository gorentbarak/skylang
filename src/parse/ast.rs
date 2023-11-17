#[derive(Debug)]
pub enum Expr<'a> {
    MathExpr(Math),
    FunCall(FunCall<'a>),
    FunDefenition(FunDefenition<'a>),
    VarDefenition(VarDefenition<'a>),
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
    pub name: &'a str,
    pub params: Vec<FunParamCall>,
}

#[derive(Debug)]
pub struct FunDefenition<'a> {
    name: &'a str,
    params: Vec<FunParamDef<'a>>,
    contents: Vec<Expr<'a>>,
    return_value: &'a Expr<'a>,
}

#[derive(Debug)]
pub struct FunParamDef<'a> {
    name: &'a str,
}

#[derive(Debug)]
pub struct FunParamCall {
    // Everything is a u64 for now.
    pub value: u64,
}

#[derive(Debug)]
pub struct VarDefenition<'a> {
    pub name: &'a str,
    pub value: u64,
}
