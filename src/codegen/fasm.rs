use crate::parse::ast::*;

pub fn fasm_codegen(exprs: &Vec<Expr>, not_a_function: bool) -> String {
    let mut asm_text = String::new();
    let mut asm_data = String::new();
    let mut asm_start = String::new();
    if not_a_function {
	asm_start.push_str("format ELF64 executable 3\n");
	asm_start.push_str("segment readable executable\n");
	asm_start.push_str("entry _start\n");
	asm_start.push_str("_start:\n");
	asm_data.push_str("\nsegment readable writable\n");
    }
    
    let topush = if not_a_function { &mut asm_start } else { &mut asm_text };

    for mut expr in exprs.iter() {
	match &mut expr {
	    Expr::MathExpr(e) => {
		topush.push_str(format!("\tmov r10, {}\n", e.left.unwrap()).as_str());
		topush.push_str(format!("\tmov r11, {}\n", e.right.unwrap()).as_str());
		match e.operator {
		    // If the operator is addition.
		    MathOperator::OP_ADD  => {
			topush.push_str("\tadd r10, r11\n");
			topush.push_str("\tmov rax, r10\n");
		    },
		    // If the operator is multiplication.
		    MathOperator::OP_MULT => {
			topush.push_str("\timul r10, r11\n");
			topush.push_str("\tmov rax, r10\n");
		    },
		    // If the operator is division.
		    MathOperator::OP_DIV  => {
			topush.push_str("\tmov rax, r10\n");
			topush.push_str("\txor rdx, rdx\n");
			topush.push_str("\tidiv r11\n");
			// The quotient is now stored in %rax.
		    },
		    // If the operators is subtraction.
		    MathOperator::OP_SUB  => {
			topush.push_str("\tsub r10, r11\n");
			topush.push_str("\tmov rax, r10\n");
		    },
		    // If the operator is modulo.
		    MathOperator::OP_MOD  => {
			topush.push_str("\tmov rax, r10\n");
			topush.push_str("\txor rdx, rdx\n");
			topush.push_str("\tidiv r11\n");
			topush.push_str("\tmov rax, rdx\n");
			// The remainder will now be stored in the %rax register.
		    }
		}
	    },

	    Expr::FunCall(e) => {
		for (i, p) in e.params.iter().enumerate() {
		    match i {
			0 => {
			    // First parameter. Put in %rdi.
			    topush.push_str(format!("\tmov rdi, {}\n", p.value.unwrap()).as_str());
			},

			1 => {
			    // Second parameter. Put in %rsi.
			    topush.push_str(format!("\tmov rsi, {}\n", p.value.unwrap()).as_str());
			},

			2 => {
			    // Third parameter. Put in %rdx.
			    topush.push_str(format!("\tmov rdx, {}\n", p.value.unwrap()).as_str());
			},

			3 => {
			    // Fourth parameter. Put in %rcx.
			    topush.push_str(format!("\tmov rcx, {}\n", p.value.unwrap()).as_str());
			},

			4 => {
			    // Fifth parameter. Put in %r8.
			    topush.push_str(format!("\tmov r8, {}\n", p.value.unwrap()).as_str());
			},

			5 => {
			    // Sixth parameter. Put in %r9.
			    topush.push_str(format!("\tmov r9, {}\n", p.value.unwrap()).as_str());
			},

			_ => {
			    // Parameters after the sixth parameter are pushed to the stack.
			    topush.push_str(format!("\tpush {}\n", p.value.unwrap()).as_str());
			}
		    }
		}

		topush.push_str(format!("call {}", e.name).as_str());
	    },

	    Expr::VarDefinition(e) => {
		asm_data.push_str(format!("\t{} dq {}", e.name, e.value.unwrap()).as_str());
	    },

	    Expr::FunDefinition(e) => {
		topush.push_str(format!("{}:\n", e.name).as_str());
		topush.push_str(fasm_codegen(&e.contents, false).as_str());
	    },

	    Expr::Return(e)        => {
		topush.push_str(format!("mov rax, {}", e.unwrap()).as_str());
	    },

	    Expr::Breakpoint       => {
		topush.push_str("\tint3\n");
	    },

	    _ => unimplemented!("sorry unimplemented"),
	}
    }

    if not_a_function {
	asm_start.push_str("\tmov rax, 60    ; 60 is the system call number for exit.\n");
	asm_start.push_str("\txor rdi, rdi   ; 0 is the exit code we want.\n");
	asm_start.push_str("\tsyscall        ; this is the instruction to actually perform the system call.");
    }
    
    let asm = format!("{}{}{}", asm_start, asm_text, asm_data);
    asm
}
