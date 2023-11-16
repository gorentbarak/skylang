use crate::parse::ast::*;

pub fn fasm_codegen(expr: Expr) -> String {
    let mut asm = String::new();

    asm.push_str("format ELF64 executable 3\n");
    asm.push_str("segment readable executable\n");
    asm.push_str("entry _start\n");
    asm.push_str("_start:\n");
    match expr {
	Expr::MathExpr(e) => {
	    asm.push_str(format!("\tmov r10, {:?}\n", e.left).as_str());
	    asm.push_str(format!("\tmov r11, {:?}\n", e.right).as_str());
	    match e.operator {
		// If the operator is addition.
		MathOperator::OP_ADD => {
		    asm.push_str("\tadd r10, r11\n");
		    asm.push_str("\tmov rax, r10\n");
		},
		// If the operator is multiplication.
		MathOperator::OP_MULT => {
		    asm.push_str("\timul r10, r11\n");
		    asm.push_str("\tmov rax, r10\n");
		},
		// If the operator is division.
		MathOperator::OP_DIV => {
		    asm.push_str("\tmov rax, r10\n");
		    asm.push_str("\tmov rdx, r11\n");
		    asm.push_str("\tidiv r10, r11\n");
		    asm.push_str("\tmov rax, r10\n");
		},
		// If the operators is subtraction.
		MathOperator::OP_SUB => {
		    asm.push_str("\tsub r10, r11\n");
		    asm.push_str("\tmov rax, r10\n");
		},
		// If the operator is modulo.
		MathOperator::OP_MOD => {
		    asm.push_str("\tmov rax, r10\n");
		    asm.push_str("\tmov rdx, r11\n");
		    asm.push_str("\tidiv r10, r11\n");
		    asm.push_str("\tmov rax, rdx\n");

		},
		_ => unimplemented!("sorry unimplemented"),
	    }
	},
	_ => unimplemented!("sorry unimplemented"),
    }

    println!("{}", asm);
    asm
}
