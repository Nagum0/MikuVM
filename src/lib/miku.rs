//! # The VM.
//!
//! This module holds all the main functionality of the virtual machine 
//! and the virtual machine struct itself.
//! 
//! ## Examples
//! ``` rust
//! use vm::{
//!     miku::MikuVM,
//!     inst::*,
//!     types::MikuType,
//! };
//!
//! let mut vm = MikuVM::new();
//! let inst1: Box<dyn Inst> = Box::new(Push::new(MikuType::U8(69)));
//! vm.push_inst(&inst1);
//! let _ = vm.run_program();
//! ```

use crate::{
    error::MikuError, inst::*, types::MikuType, STACK_MAX_SIZE};

/// The main structure of the virtual machine.
#[derive(Debug)]
pub struct MikuVM<'a> {
    /// The stack.
    /// It's represented as a [`Vec`] of [`MikuType`].
    stack: Vec<MikuType>,
    /// Points to the top of the current stackframe.
    stack_top: usize,
    /// Points to the base of the current stackframe.
    stack_base: usize,
    
    /// The loaded program.
    /// A [`Vec`] of `&'a Box<dyn Inst>` (a reference with lifetime a to a pointer that points to an object that implements the [Inst] trait)
    program: Vec<&'a Box<dyn Inst>>,
    /// The program counter.
    /// Points to the next instruciton to be executed.
    pc: usize,
}

impl<'a> MikuVM<'a> {
    /// Creates a new empty vm.
    pub fn new() -> Self {
        Self { 
            stack: Vec::new(), 
            stack_top: 0, 
            stack_base: 0, 
            program: Vec::new(), 
            pc: 0 
        }
    }
    
    /// Runs until the program terminates. Executes each instruction stored in program.
    /// # Returns
    /// - `Ok(())` if the execution doesn't hit an error.
    /// - [`MikuError`] if something goes wrong during execution.
    pub fn run_program(&mut self) -> Result<(), MikuError> {
        while self.pc != self.program.len() {
            let inst = self.program[self.pc];
            inst.execute(self)?;
        }

        Ok(())
    }
    
    /// Push a [`MikuType`] onto the stack.
    /// # Returns
    /// - `Ok(())` on successful push.
    /// - [`MikuError::StackOverflow`] if the stack is out of space.
    pub fn stack_push(&mut self, stack_entry: MikuType) -> Result<(), MikuError> {
        if self.stack.len() == STACK_MAX_SIZE {
            return Err(MikuError::StackOverflow);           
        }

        if self.stack_top == self.stack.len() {
            self.stack.push(stack_entry);
        }
        else {
            self.stack[self.stack_top] = stack_entry;
        }

        self.stack_top += 1;

        Ok(())
    }
    
    /// Pops the top entry off the stack. 
    /// # Returns
    /// - `Ok(())` on successful pop.
    /// - [`MikuError::StackUnderflow`] if the stack is empty. 
    pub fn stack_pop(&mut self) -> Result<(), MikuError> {
        match self.stack.pop() {
            Some(_) => self.stack_top -= 1,
            None => return Err(MikuError::StackUnderflow)
        }
        Ok(())
    }
    
    /// Increment the program counter by 1.
    pub fn inc_pc(&mut self) {
        self.pc += 1;
    }
    
    /// Pushes an instruciton into the program.
    pub fn push_inst(&mut self, inst: &'a Box<dyn Inst>) {
        self.program.push(inst);
    }

    /// The stack.
    /// It's represented as a [`Vec`] of [`MikuType`].
    /// Returns a clone of the vm's stack.
    pub fn stack(&self) -> Vec<MikuType> {
        self.stack.clone()
    }

    /// The program counter.
    /// Points to the next instruciton to be executed.
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// Returns a pointer to the top of the current stackframe.
    pub fn stack_top(&self) -> usize {
        self.stack_top
    }

    /// Returns a pointer to the base of the current stackframe.
    pub fn stack_base(&self) -> usize {
        self.stack_base
    }
}
