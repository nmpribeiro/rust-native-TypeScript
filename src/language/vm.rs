use super::chunk::Chunk;
use super::common::{intern, OpCode, StrId};
#[cfg(feature = "log_level_debug")]
use super::debug::{disassemble_chunk, Debug};
use super::value::Value;
use crate::Compiler;

pub enum Failure {
    CompileError,
    RuntimeError,
}
type Res = Result<(), Failure>;

pub enum InterpreterResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::<Value>::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Res {
        let mut compiler = Compiler::new(source);
        let chunk = Chunk::new();
        compiler.compile(chunk);
        // let cls = compiler.compile().ok_or(Failure::CompileError)?;

        // self.define_natives();
        // let cls = cls.to_obj();
        // self.stack.push(Value::Closure(cls.clone()));

        self.run()
    }

    fn run(&mut self) -> Res {
        if self.chunk.code.is_empty() {
            return Ok(());
        }

        loop {
            let current_instruction = self.chunk.code[self.ip];
            self.ip = self.ip + 1;
            #[cfg(feature = "log_level_debug")]
            {
                print!("          ");
                for value in self.stack.iter() {
                    print!("[ {:04} ]", value);
                }
                println!("");
                let name: StrId = intern("to be changed");
                disassemble_chunk(&self.chunk, &Some(name));
                // disassemble_instruction(&self.chunk, &current_instruction, 0);
            }

            match current_instruction {
                OpCode::NULL => self.stack.push(Value::ValNull),
                OpCode::BOOL(val) => self.stack.push(Value::ValBool(val)),
                OpCode::NUMBER(val) => self.stack.push(Value::ValNumber(val)),
                OpCode::STRING(str) => self.stack.push(Value::ConstString(str)),

                OpCode::POP => {
                    self.pop();
                }

                OpCode::TRUE => self.stack.push(Value::ValBool(true)),
                OpCode::FALSE => self.stack.push(Value::ValBool(false)),
                OpCode::ADD
                | OpCode::SUBTRACT
                | OpCode::MULTIPLY
                | OpCode::DIVIDE
                | OpCode::EQUAL
                | OpCode::GREATER
                | OpCode::LESS => {
                    let result = self.binary_operation_values(&current_instruction);
                    if let Some(result) = result {
                        self.stack.push(result)
                    } else {
                        // let line = &self.chunk.code[(frame.ip) as usize].line;
                        let line = &self.chunk.lines.pop().unwrap();
                        self.print_error(line, "Binary operation had invalid operands!");
                        break;
                    }
                }
                OpCode::NEGATE | OpCode::NOT => {
                    let result = self.unary_instruction(&current_instruction);
                    if let Some(result) = result {
                        self.stack.push(result)
                    } else {
                        // let line = current_func.chunk.code[(frame.ip) as usize].line;
                        let line = &self.chunk.lines.pop().unwrap();
                        self.print_error(line, "Unary operation had an invalid operand!");
                        break;
                    }
                }
                OpCode::PRINT => {
                    println!("{}", self.pop());
                    break;
                }
                OpCode::RETURN => {
                    println!("{}", self.pop());
                    return Ok(());
                }
            }
        }
        // All terminations of this loop are to be interpreted as an error,
        // return will return directly and prevent hitting this
        Err(Failure::RuntimeError)
    }

    fn print_error(&mut self, line: &isize, message: &str) {
        println!("[Line {}] Runtime error: {}", line, message);
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        match self.stack.pop() {
            Some(value) => return value,
            _ => panic!("VM tried to get value from empty stack"),
        }
    }

    // fn peek(&mut self, distance: i32) -> Value {
    // return self.stackTop[-1 - distance];
    // }

    fn unary_instruction(&mut self, opcode: &OpCode) -> Option<Value> {
        match opcode {
            OpCode::NEGATE => {
                let operand = self.pop();
                if !operand.same_type_as(&Value::ValNumber(0.1)) {
                    return operand.neg();
                } else {
                    panic!("Operand must be a number."); // runtime error!
                }
            }
            OpCode::NOT => Some(self.pop().not()),
            _ => panic!("unknown opcode"),
        }
    }

    fn binary_operation_values(&mut self, operation: &OpCode) -> Option<Value> {
        let a = self.pop();
        let b = self.pop();
        match operation {
            OpCode::ADD => Some(a.add(b)),
            OpCode::SUBTRACT => a.sub(b),
            OpCode::DIVIDE => a.div(b),
            OpCode::MULTIPLY => a.mul(b),
            OpCode::EQUAL => Some(Value::ValBool(a == b)),
            OpCode::GREATER => a.greater(b),
            OpCode::LESS => a.less(b),
            _ => panic!("unknown opcode"),
        }
    }
}
