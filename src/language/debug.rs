pub use super::chunk::Chunk;
pub use super::common::{to_str, OpCode, StrId};
use smol_str::SmolStr;

#[cfg(feature = "log_level_debug")]
pub trait Debug {
    fn disassemble(&self, name: &Option<StrId>);
}

#[cfg(feature = "log_level_debug")]
impl Debug for Chunk {
    // fn disassemble(&self) {
    //     println!("== {} ==", name);

    //     for i in 0..self.code.len() {
    //         match self.code.get(i) {
    //             Some(op_code) => Chunk::disassemble_instruction(i, op_code),
    //             _ => println!("Error debugging OpCode on at {}", i),
    //         }
    //     }
    // }

    fn disassemble(&self, name: &Option<StrId>) {
        if cfg!(debug_assertions) {
            println!(
                "== {} ==",
                name.map(to_str)
                    .unwrap_or_else(|| SmolStr::new_inline("SCRIPT"))
            );
            for i in 0..self.code.len() {
                match self.code.get(i) {
                    Some(instruction) => disassemble_instruction(i, instruction, self),
                    _ => println!("Error debugging OpCode on at {}", i),
                }
            }
        }
    }
}

fn disassemble_instruction(index: usize, instruction: &OpCode, chunk: &Chunk) {
    print!("{:04} ", index);
    if index > 0 && chunk.lines[index] == chunk.lines[index - 1] {
        print!("   | ");
    } else {
        print!("{:04} ", chunk.lines[index]);
    }
    match instruction {
        OpCode::STRING(value) => print!("{:03}\n", instruction),
        _ => print!("{:03} \n", instruction),
    }
}

// #[cfg(feature = "log_level_debug")]
// pub fn disassemble_instruction(chunk: &Chunk, op_code: &OpCode, offset: usize) {
//     print!("{:04} ", offset);

//     if offset > 0 && offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
//         print!("   | ");
//     } else {
//         print!("{:04} ", chunk.lines[offset]);
//     }

//     match op_code {
//         OpCode::ConstBool(val) => constant_instruction_bool(chunk, val),
//         OpCode::ConstNumber(val) => constant_instruction_number(chunk, val),
//         OpCode::ConstString(val) => constant_instruction_string(chunk, val),
//         // OpCode::Constant(index) => constant_instruction(chunk, *index),
//         OpCode::Negate => simple_instruction(OpCode::Negate),
//         OpCode::Add => simple_instruction(OpCode::Add),
//         OpCode::Subtract => simple_instruction(OpCode::Subtract),
//         OpCode::Multiply => simple_instruction(OpCode::Multiply),
//         OpCode::Divide => simple_instruction(OpCode::Divide),
//         OpCode::Return => simple_instruction(OpCode::Return),
//         _ => println!("Unknown opcode {:04}\n", op_code),
//     }
// }

// #[cfg(feature = "log_level_debug")]
// fn simple_instruction(op_code: OpCode) {
//     println!("{}", op_code);
// }

// #[cfg(feature = "log_level_debug")]
// fn constant_instruction(chunk: &Chunk, index: usize) {
//     let value = chunk.constants[index];
//     println!(
//         "{:<16} {:04} '{:.prec$}'",
//         "CONSTANT",
//         index,
//         value,
//         prec = 2
//     );
// }

// #[cfg(feature = "log_level_debug")]
// fn constant_instruction_bool(chunk: &Chunk, value: &bool) {
//     println!("{:<16} {:04} '{:.prec$}'", "CONST_BOOL", value, prec = 2);
// }

// #[cfg(feature = "log_level_debug")]
// fn constant_instruction_number(chunk: &Chunk, value: &f64) {
//     println!("{:<16} {:04} '{:.prec$}'", "CONST_BOOL", value, prec = 2);
// }

// #[cfg(feature = "log_level_debug")]
// fn constant_instruction_string(chunk: &Chunk, value: &StrId) {
//     println!(
//         "{:<16} {:04} '{:.prec$}'",
//         "CONST_BOOL",
//         to_str(*value),
//         prec = 2
//     );
// }
