pub mod lex;
pub mod codegen;
use crate::codegen::fasm::*;
use crate::parse::ast::*;
pub mod parse;

fn main() {
    fasm_codegen(Expr::MathExpr(Math {left: 1, right: 2, operator: MathOperator::OP_DIV}));
}
