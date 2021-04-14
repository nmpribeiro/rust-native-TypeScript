use {
    super::common::{ConstantIndex, OpCode},
    super::value::NumberValueType,
};

pub struct Chunk {
    pub count: usize,
    // pub capacity: usize,
    pub code: Vec<OpCode>,
    pub lines: Vec<isize>,
    pub constants: Vec<NumberValueType>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            count: 0,
            code: Vec::<OpCode>::new(),
            constants: Vec::<NumberValueType>::new(),
            lines: Vec::<isize>::new(),
        }
    }

    pub fn add_op_code(&mut self, op_code: OpCode, line: isize) {
        self.code.push(op_code);
        self.lines.push(line);
        self.count = self.count + 1;
    }

    pub fn add_constant(&mut self, value: NumberValueType) -> ConstantIndex {
        self.constants.push(value);
        return self.constants.len() - 1;
    }
}
