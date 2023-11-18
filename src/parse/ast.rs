#[derive(Debug)]
pub enum Expr<'a> {
    MathExpr(Math<'a>),
    FunCall(FunCall<'a>),
    FunDefinition(FunDefinition<'a>),
    VarDefinition(VarDefinition<'a>),
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
pub struct FunDefinition<'a> {
    name: &'a str,
    params: Vec<FunParamDef<'a>>,
    contents: Vec<Expr<'a>>,
    return_value: Value<'a>,
}

#[derive(Debug)]
pub struct FunParamDef<'a> {
    name: &'a str,
    number: u64,
}

#[derive(Debug)]
pub struct FunParamCall<'a> {
    pub value: Value<'a>,
}

// VARIABLES

#[derive(Debug)]
pub struct VarDefinition<'a> {
    pub name: &'a str,
    pub value: Value<'a>,
}

#[derive(Debug, Copy, Clone)]
pub struct VarReference<'a> {
    pub name: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub struct ParamReference<'a> {
    pub param_number: u64,
}

#[derive(Debug, Copy, Clone)]
pub enum Value<'a> {
    Var(VarReference<'a>),
    Param(ParamReference<'a>),
    Number(u64),
}

impl<'a> Value<'a> {
    pub fn unwrap(&self) -> String {
	match self {
	    Value::Param(e) => {
		
	    },
	    
	    Value::Number(e) => {
		return e.to_string();
	    },

	    Value::Var(e) => {
		return format!("[{}]", e.name.to_string());
	    }
	}
    }
}
