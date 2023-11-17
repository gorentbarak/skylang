use crate::parse::ast::*;

pub fn fasm_codegen(expr: Expr) -> String {
    let mut asm_text = String::new();
    let mut asm_data = String::new();
    
    asm_text.push_str("format ELF64 executable 3\n");
    asm_text.push_str("segment readable executable\n");
    asm_text.push_str("entry _start\n");
    asm_text.push_str("_start:\n");
    asm_data.push_str("segment readable writeable\n");
    match expr {
	Expr::MathExpr(e) => {
	    asm_text.push_str(format!("\tmov r10, {:?}\n", e.left).as_str());
	    asm_text.push_str(format!("\tmov r11, {:?}\n", e.right).as_str());
	    match e.operator {
		// If the operator is addition.
		MathOperator::OP_ADD => {
		    asm_text.push_str("\tadd r10, r11\n");
		    asm_text.push_str("\tmov rax, r10\n");
		},
		// If the operator is multiplication.
		MathOperator::OP_MULT => {
		    asm_text.push_str("\timul r10, r11\n");
		    asm_text.push_str("\tmov rax, r10\n");
		},
		// If the operator is division.
		MathOperator::OP_DIV => {
		    asm_text.push_str("\tmov rax, r10\n");
		    asm_text.push_str("\tmov rdx, r11\n");
		    asm_text.push_str("\tidiv r10, r11\n");
		    asm_text.push_str("\tmov rax, r10\n");
		},
		// If the operators is subtraction.
		MathOperator::OP_SUB => {
		    asm_text.push_str("\tsub r10, r11\n");
		    asm_text.push_str("\tmov rax, r10\n");
		},
		// If the operator is modulo.
		MathOperator::OP_MOD => {
		    asm_text.push_str("\tmov rax, r10\n");
		    asm_text.push_str("\tmov rdx, r11\n");
		    asm_text.push_str("\tidiv r10, r11\n");
		    asm_text.push_str("\tmov rax, rdx\n");

		}
	    }
	},

	Expr::FunCall(e) => {
	    for (i, p) in e.params.iter().enumerate() {
		match i {
		    0 => {
			// First parameter. Put in %rdi.
			asm_text.push_str(format!("\tmov rdi, {:?}\n", p.value).as_str());
		    },

		    1 => {
			// Second parameter. Put in %rsi.
			asm_text.push_str(format!("\tmov rsi, {:?}\n", p.value).as_str());
		    },

		    2 => {
			// Third parameter. Put in %rdx.
			asm_text.push_str(format!("\tmov rdx, {:?}\n", p.value).as_str());
		    },

		    3 => {
			// Fourth parameter. Put in %rcx.
			asm_text.push_str(format!("\tmov rcx, {:?}\n", p.value).as_str());
		    },

		    4 => {
			// Fifth parameter. Put in %r8.
			asm_text.push_str(format!("\tmov r8, {:?}\n", p.value).as_str());
		    },
		    
		    5 => {
			// Sixth parameter. Put in %r9.
			asm_text.push_str(format!("\tmov r9, {:?}\n", p.value).as_str());
		    },

		    _ => {
			// Parameters after the sixth parameter are pushed to the stack.
			asm_text.push_str(format!("\tpush {:?}\n", p.value).as_str());
		    }
		}
	    }

	    asm_text.push_str(format!("call {:?}", e.name).as_str());
	},

	Expr::VarDefenition(e) => {
	    asm_data.push_str(format!("{} dq {:?}", e.name, e.value).as_str());
	},
	
	_ => unimplemented!("sorry unimplemented"),
    }
    let asm = format!("{}{}", asm_text, asm_data);
    println!("{}", asm);
    asm
}
