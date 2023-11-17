#[derive(Debug)]
pub enum Expr<'a> {
    MathExpr(Math<'a>),
    FunCall(FunCall<'a>),
    FunDefenition(FunDefenition<'a>),
    VarDefenition(VarDefenition<'a>),
    VarReference(VarReference<'a>),
    Breakpoint
}

// MATH EXPRESSION

#[derive(Debug)]
pub struct Math<'a> {
    pub left: Value<'a>,
    pub right: Value<'a>,
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
    pub params: Vec<FunParamCall<'a>>,
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
pub struct FunParamCall<'a> {
    pub value: Value<'a>,
}

// VARIABLES

#[derive(Debug)]
pub struct VarDefenition<'a> {
    pub name: &'a str,
    pub value: Value<'a>,
}

#[derive(Debug, Copy, Clone)]
pub struct VarReference<'a> {
    pub name: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub enum Value<'a> {
    Var(VarReference<'a>),
    Number(u64),
}

impl<'a> Value<'a> {
    pub fn unwrap(&self) -> String {
	match self {
	    Value::Number(e) => {
		return e.to_string();
	    },

	    Value::Var(e) => {
		return format!("[{}]", e.name.to_string());
	    }
	}
    }
}
