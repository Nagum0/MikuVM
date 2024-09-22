use crate::inst::Inst;
use crate::stack::StackEntry;

pub struct Miku {
    pub program: Vec<Inst>,
    pub ins_ptr: usize,

    pub stack: Vec<StackEntry>,
    pub stack_base: usize,
    pub stack_top: usize,
}

impl Miku {
    pub fn new() -> Miku {
        Miku {
            program: Vec::new(),
            ins_ptr: 0,

            stack: Vec::new(),
            stack_base: 0,
            stack_top: 0,
        }
    }

    pub fn run_program(&mut self) {
        while self.ins_ptr < self.program.len() {
            let inst = self.program[self.ins_ptr];
            inst.execute(self);
            self.dump_stack();
        }
    }

    fn dump_stack(&self) {
        println!("Stack ({}):", self.stack_top);
        for i in 0..self.stack.len() {
            println!("  [{:<3}]  {:?}", i, self.stack[i]);
        }
    }
}
