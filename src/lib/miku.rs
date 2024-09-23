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
            println!("Executing {:?}:", inst);
            inst.execute(self);

            if self.stack_top != 0 {
                self.dump_stack();
            } else {
                println!("  [Empty]")
            }
        }
    }

    fn dump_stack(&self) {
        println!("  Stack ({}) -> ({}):", self.stack_base, self.stack_top);
        for i in 0..self.stack_top {
            println!("    [{:<2}]  {:?}", i, self.stack[i]);
        }
    }
}
