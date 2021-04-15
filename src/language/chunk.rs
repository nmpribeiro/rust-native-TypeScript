use super::common::OpCode;

pub struct Chunk {
    pub count: usize,
    pub code: Vec<OpCode>,
    pub lines: Vec<isize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            count: 0,
            code: Vec::<OpCode>::new(),
            lines: Vec::<isize>::new(),
        }
    }

    pub fn add_op_code(&mut self, op_code: OpCode, line: isize) {
        self.code.push(op_code);
        self.lines.push(line);
        self.count += 1;
    }
}
