#[derive(Debug)]
pub enum Expr<'a> {
    MathExpr(Math<'a>),
    FunCall(FunCall<'a>),
    FunDefinition(FunDefinition<'a>),
    VarDefinition(VarDefinition<'a>),
    Return(Value<'a>),
    Breakpoint
}

// MATH EXPRESSION

#[derive(Debug, Copy, Clone)]
pub struct Math<'a> {
    pub left: &'a Value<'a>,
    pub right: &'a Value<'a>,
    pub operator: MathOperator
}

#[derive(Debug, Copy, Clone)]
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
    pub name: &'a str,
    pub contents: Vec<Expr<'a>>,
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
pub struct ParamReference {
    pub param_number: u64,
}

#[derive(Debug, Copy, Clone)]
pub enum Value<'a> {
    Var(VarReference<'a>),
    Param(ParamReference),
    Number(u64),
    Math(Math<'a>),
}

impl<'a> Value<'a> {
    pub fn unwrap(&self) -> String {
	match self {
	    Value::Param(e) => {
		match e.param_number {
		    0 => { return "rdi".to_string(); },
		    1 => { return "rsi".to_string(); },
		    2 => { return "rdx".to_string(); },
		    3 => { return "rcx".to_string(); },
		    4 => { return  "r8".to_string(); },
		    5 => { return  "r9".to_string(); },
		    _ => { unimplemented!() }
		}
	    },
	    
	    Value::Number(e) => {
		return e.to_string();
	    },

	    Value::Var(e) => {
		return format!("[{}]", e.name.to_string());
	    },

            Value::Math(e) => {
                return String::from("rax");
            }
	}
    }
}
