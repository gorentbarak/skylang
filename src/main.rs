#![allow(warnings)]

pub mod lex;
pub mod codegen;
use crate::codegen::fasm::*;
use crate::parse::ast::*;
pub mod parse;

fn main() {
    fasm_codegen(vec![Expr::VarDefenition(VarDefenition {name: "goren", value: Value::Number(10)}), Expr::MathExpr(Math { left: Value::Var(VarReference {name: "goren"}), right: Value::Number(4), operator: MathOperator::OP_DIV})]);
}
