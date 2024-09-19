use crate::inst::Inst;
use crate::stack::StackEntry;

pub struct Miku {
    pub program: Vec<Inst>,
    pub stack: Vec<StackEntry>,
}

impl Miku {
    pub fn new() -> Miku {
        Miku {
            program: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn run_program(&mut self) {
        for i in 0..self.program.len() {
            let inst = self.program[i];
            inst.execute(self);
        }
        self.dump_stack();
    }

    fn dump_stack(&self) {
        println!("Stack ({})", self.stack.len());
        for entry in &self.stack {
            println!("  {:?}", entry);
        }
    }
}
