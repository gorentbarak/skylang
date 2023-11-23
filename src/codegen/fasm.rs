use crate::parse::ast::*;

pub fn fasm_codegen(exprs: &Vec<Expr>, not_a_function: bool) -> String {
    // Define asm_func, used for functions.
    let mut asm_func = String::new();
    // Define asm_data, used for variables.
    let mut asm_data = String::new();
    // Define asm_start, used for the entry point.
    let mut asm_start = String::new();

    // If not_a_function, push necessary headers to the asm_start variable.
    if not_a_function {
	asm_start.push_str("format ELF64 executable 3\n");
	asm_start.push_str("segment readable executable\n");
	asm_start.push_str("entry _start\n");
	asm_start.push_str("_start:\n");
	asm_data.push_str("\nsegment readable writable\n");
    }

    // Iterate over expressions.
    for expr in exprs.iter() {
	// Use patern matching on `expr`.
	match expr {
	    // If the expression is a math expression.
	    Expr::MathExpr(e) => {
		asm_start.push_str(format!("\tmov r10, {}\n", e.left.unwrap()).as_str());
		asm_start.push_str(format!("\tmov r11, {}\n", e.right.unwrap()).as_str());
		match e.operator {
		    // If the operator is addition.
		    MathOperator::OP_ADD  => {
			asm_start.push_str("\tadd r10, r11\n");
			asm_start.push_str("\tmov rax, r10\n");
			// r10 ← r10 + r11; rax ← r10;
			// The sum will now be stored in the %rax register.
		    },
		    // If the operator is multiplication.
		    MathOperator::OP_MULT => {
			asm_start.push_str("\timul r10, r11\n");
			asm_start.push_str("\tmov rax, r10\n");
			// r10 ← r10 * r11; rax ← r10;
			// The product will now be stored in the %rax register.
		    },
		    // If the operator is division.
		    MathOperator::OP_DIV  => {
			asm_start.push_str("\tmov rax, r10\n");
			asm_start.push_str("\txor rdx, rdx\n");
			asm_start.push_str("\tidiv r11\n");
			// rax ← r10; rdx ← 0; rax ← concat(rax, rdx) / r11;
			// The quotient will now be stored in the %rax register.
		    },
		    // If the operators is subtraction.
		    MathOperator::OP_SUB  => {
			asm_start.push_str("\tsub r10, r11\n");
			asm_start.push_str("\tmov rax, r10\n");
			// r10 ← r10 - r11; rax ← r10;
			// The difference will now be stored in the %rax register.
		    },
		    // If the operator is modulo.
		    MathOperator::OP_MOD  => {
			asm_start.push_str("\tmov rax, r10\n");
			asm_start.push_str("\txor rdx, rdx\n");
			asm_start.push_str("\tidiv r11\n");
			asm_start.push_str("\tmov rax, rdx\n");
			// rax ← r10; rdx ← 0; rdx ← concat(rax, rdx) % r11; rax ← rdx;
			// The remainder will now be stored in the %rax register.
		    }
		}
	    },

	    // If the expression is a function call.
	    Expr::FunCall(e) => {
		for (i, p) in e.params.iter().enumerate() {
		    match i {
			0 => {
			    // First parameter. Put in %rdi.←		    asm_start.push_str(format!("\tmov rdi, {}\n", p.unwrap()).as_str());
			    // rdi ← e.params[0];
			},

			1 => {
			    // Second parameter. Put in %rsi.
			    asm_start.push_str(format!("\tmov rsi, {}\n", p.unwrap()).as_str());
			    // rsi ← e.params[1];
			},

			2 => {
			    // Third parameter. Put in %rdx.
			    asm_start.push_str(format!("\tmov rdx, {}\n", p.unwrap()).as_str());
			    // rdx ← e.params[2];
			},

			3 => {
			    // Fourth parameter. Put in %rcx.
			    asm_start.push_str(format!("\tmov rcx, {}\n", p.unwrap()).as_str());
			    // rcx ← e.params[3];
			},

			4 => {
			    // Fifth parameter. Put in %r8.
			    asm_start.push_str(format!("\tmov r8, {}\n", p.unwrap()).as_str());
			    // r8 ← e.params[4];
			},

			5 => {
			    // Sixth parameter. Put in %r9.
			    asm_start.push_str(format!("\tmov r9, {}\n", p.unwrap()).as_str());
			    // r9 ← e.params[5];
			},

			_ => {
			    // Parameters after the sixth parameter are pushed to the stack.
			    asm_start.push_str(format!("\tpush {}\n", p.unwrap()).as_str());
			    // STACK_TOP ← e.params[(6+)];
			}
		    }
		}

		// Call the function.
		asm_start.push_str(format!("\tcall {}\n", e.name).as_str());
	    },

	    // Define a global variable.
	    Expr::VarDefinition(e) => {
		// Define a 64-bit variable.
		asm_data.push_str(format!("\t{} dq {}", e.name, e.value.unwrap()).as_str());
	    },

	    // Breakpoint.
	    Expr::Breakpoint       => {
		// Write the interrupt for a debugger breakpoint.
		asm_start.push_str("\tint3\n");
	    },

	    Expr::Return(e)        => {
		asm_start.push_str(format!("mov [rbp - 8], {}", e.unwrap()).as_str());
	    },

            Expr::FunDefinition(e) => {
                asm_func.push_str(format!("{}:\n", e.name).as_str());
	        asm_func.push_str(fasm_codegen(&e.contents, false).as_str());
                asm_func.push_str("\tret\n");
            },

	    _ => unsafe {
		let mut ptr = 0x00 as *mut u32;
		::std::ptr::write(ptr, "GOREN IS MY NAME AND BREAKING MEMORY IS MY GAME");
	    },
	}
        
    }
    

    if not_a_function {
        asm_start.push_str("\tmov rax, 60    ; 60 is the system call number for exit.\n");
        asm_start.push_str("\txor rdi, rdi   ; 0 is the exit code we want.\n");
        asm_start.push_str("\tsyscall        ; this is the instruction to actually perform the system call.\n");
    }
    let asm = format!("{}{}{}", asm_start, asm_func, asm_data);
    asm
}
