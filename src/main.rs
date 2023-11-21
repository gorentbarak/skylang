#![allow(warnings)]

pub mod lex;
pub mod codegen;
use crate::codegen::fasm::*;
use crate::parse::ast::*;
pub mod parse;

fn main() {
    let fc = fasm_codegen(&
                          vec![
                              Expr::VarDefinition(VarDefinition {name: "goren", value: Value::Number(10)}),
                              Expr::MathExpr(Math {
                                  left: &Value::Var(VarReference { name: "goren"}),
                                  right: &Value::Number(17),
                                  operator: MathOperator::OP_MULT
                              }
                              ),
                              Expr::FunDefinition(FunDefinition {
                                  name: "adder", contents: vec![
                                      Expr::Return(Value::Math(
                                          Math {
                                              left: &Value::Param(ParamReference {param_number: 0}),
                                              right: &Value::Param(ParamReference {param_number: 1}),
                                              operator: MathOperator::OP_ADD
                                          }
                                      ))
                                  ]
                              }),
                              Expr::Breakpoint],
                          true
    );
    println!("{}", fc);
}
