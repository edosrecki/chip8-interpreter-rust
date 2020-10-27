const OPCODE_SIZE: usize = 2;

pub struct ProgramCounter {
    pc: usize,
}

impl ProgramCounter {
    pub fn new(pc: usize) -> Self {
        ProgramCounter {
            pc,
        }
    }
    
    pub fn get_current(&self) -> usize {
        self.pc
    }

    pub fn get_next(&self) -> usize {
        self.pc + OPCODE_SIZE
    }

    pub fn goto_next(&mut self) {
        self.pc += OPCODE_SIZE;
    }

    pub fn skip_next(&mut self) {
        self.pc += 2 * OPCODE_SIZE;
    }

    pub fn jump(&mut self, address: usize) {
        self.pc = address;
    }
}

impl From<ProgramCounter> for usize {
    fn from(program_counter: ProgramCounter) -> Self {
        program_counter.pc
    }
}
