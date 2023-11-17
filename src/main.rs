pub mod lex;
pub mod codegen;
use crate::codegen::fasm::*;
use crate::parse::ast::*;
pub mod parse;

fn main() {
    fasm_codegen(Expr::VarDefenition(VarDefenition {
	name: "hi",
	value: 100,
    }));
}
