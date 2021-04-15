use super::chunk::Chunk;
use super::common::{intern, to_str, OpCode, StrId};
#[cfg(feature = "log_level_debug")]
use super::debug::Debug;
use super::value::Value;
use crate::Compiler;

pub enum Failure {
    CompileError,
    RuntimeError,
}
type Res = Result<(), Failure>;

pub struct VM {
    ip: usize,
    pub chunk: Option<Chunk>,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Some(Chunk::new()),
            ip: 0,
            stack: Vec::<Value>::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Res {
        let mut compiler: Compiler = Compiler::new(source);
        compiler.compile(self.instruction_chunk());
        // let cls = compiler.compile().ok_or(Failure::CompileError)?;

        #[cfg(feature = "log_level_debug")]
        {
            print!("          ");
            for value in self.stack.iter() {
                print!("[ {:04} ]", value);
            }
            println!("");
            self.instruction_chunk()
                .disassemble(&Some(intern("vm_code")));
            // disassemble_chunk(&self.instruction_chunk(), &Some(intern("to be changed")));
            // disassemble_instruction(&self.instruction_chunk(), &current_instruction, 0);
        }

        // self.define_natives();
        // let cls = cls.to_obj();
        // self.stack.push(Value::Closure(cls.clone()));

        self.run()
    }

    fn instruction_chunk(&mut self) -> &mut Chunk {
        self.chunk.as_mut().unwrap()
    }

    fn run(&mut self) -> Res {
        println!();
        if self.instruction_chunk().code.is_empty() {
            return Ok(());
        }

        loop {
            let ip: usize = self.ip;
            let current_instruction = self.instruction_chunk().code[ip];
            self.ip += 1;

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
                        // let line = &self.instruction_chunk().code[(frame.ip) as usize].line;
                        // let line = &self.instruction_chunk().lines.pop().unwrap();
                        self.print_error("Binary operation had invalid operands!");
                        break;
                    }
                }
                OpCode::NEGATE | OpCode::NOT => {
                    let result = self.unary_instruction(&current_instruction);
                    if let Some(result) = result {
                        self.stack.push(result)
                    } else {
                        // let line = current_func.chunk.code[(frame.ip) as usize].line;
                        // let line = &self.instruction_chunk().lines.pop().unwrap();
                        self.print_error("Unary operation had an invalid operand!");
                        break;
                    }
                }
                OpCode::PRINT => println!("{}", self.pop()),
                OpCode::RETURN => {
                    if self.stack.len() > 0 {
                        println!("[vm][OpCode::RETURN] {}", self.pop());
                    } else {
                        println!("[vm][OpCode::RETURN] end prog");
                    }
                    return Ok(());
                }
            }
        }
        // All terminations of this loop are to be interpreted as an error,
        // return will return directly and prevent hitting this
        Err(Failure::RuntimeError)
    }

    fn print_error(&mut self, message: &str) {
        let mut lines = self.chunk.as_ref().unwrap().lines.clone();
        let line = lines.pop().unwrap();
        println!("[Line {}] Runtime error: {}", line, message);
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        match self.stack.pop() {
            Some(value) => return value,
            _ => {
                println!("VM tried to get value from empty stack");
                Value::ConstString(intern("unknown"))
            }
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
