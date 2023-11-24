use crate::parse::ast::*;

#[macro_export]
macro_rules! fasm_codegen {
    // Macro to make calling fasm_codegen function easier.
    ($exprs:expr) => {
	fasm_codegen(&$exprs, true)
    };

    (fun: $exprs:expr) => {
	fasm_codegen($exprs, false)
    }
}

pub fn temp(counter: u64) -> String {
    format!("tmp{:?}", counter)
}

pub fn fasm_codegen(exprs: &Vec<Expr>, not_a_function: bool) -> String {
    // A counter for how many temporary variables have been created. This is used to create new ones. The new ones will be called tmp1, tmp2, etc.
    let mut tmp_counter: u64 = 0;
    
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
		asm_start.push_str("\txor rax, rax\n");
	    },

	    // Return something from a function.
	    Expr::Return(e)        => {
		// Do the operation that should later be returned.
		asm_start.push_str(fasm_codegen!(fun: &e).as_str());
		// Move the return value to rbp + 8.
		asm_start.push_str("mov [rbp + 8], rax");
		// 8(%rbp) ← return_value
	    },

	    // A function defenition.
            Expr::FunDefinition(e) => {
		// In x86-64 assembly, a function is defined as <function_name>:. Push this to the `asm_func`.
                asm_func.push_str(format!("{}:\n", e.name).as_str());
		// Call the function itself specifying that you are defining a function, and push the returned value to `asm_func`.
	        asm_func.push_str(fasm_codegen!(fun: &e.contents).as_str());
		// Use the ret instruction to return from the procedure.
                asm_func.push_str("\tret\n");
            },

	    Expr::If(e) => {
		// Increment the temporary variable/function counter.
		tmp_counter += 1;
		// Compare the left and right value.
		asm_start.push_str(format!("\tcmp {}, {}\n", e.left.unwrap(), e.right.unwrap()).as_str());
		// Check what the condition is.
		match e.cond {
		    COND_OP::EQ => {
			// If the compared values are equal to each other jump to the temporary function.
			asm_start.push_str(format!("je .{}", temp(tmp_counter)).as_str());
		    },

		    COND_OP::NE => {
			// If the compared values are not equal to eachother jump to the temporary function.
			asm_start.push_str(format!("jne .{}", temp(tmp_counter)).as_str());
		    }
		}

		// Create the temporary function.
                asm_func.push_str(format!(".{}:\n", temp(tmp_counter)).as_str());
	        asm_func.push_str(fasm_codegen!(fun: &e.action).as_str());
                asm_func.push_str("\tret\n");

	    }
	    
	    _ => unsafe {
		// Write some data I randomly typed to your memory because don't going around playing with something that I haven't implemented yet.
		let mut ptr = 0x00 as *mut f64;
		::std::ptr::write(ptr, 124010240120401240.12410240124120401240);
	    },
	}

	
    }
    

    if not_a_function {
	// Use the exit syscall to leave the program. If you don't do this, you will get a segmentation fault.
        asm_start.push_str("\tmov rax, 60    ; 60 is the system call number for exit.\n");
        asm_start.push_str("\txor rdi, rdi   ; 0 is the exit code we want.\n");
        asm_start.push_str("\tsyscall        ; this is the instruction to actually perform the system call.\n");
    }
    // Get the final `asm` string derived from all of the other strings that we have manipulated (finally!).
    let asm = format!("{}{}{}", asm_start, asm_func, asm_data);
    // Return the final `asm` string.
    
    asm
}
