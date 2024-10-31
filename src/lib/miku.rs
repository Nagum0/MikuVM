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
    error::MikuError, inst::*, types::MikuType, DATA_END, DATA_START, HEAP_END, HEAP_START, MEMORY_SIZE, STACK_END, STACK_START};
use std::{fmt::Display, usize};

/// The main structure of the virtual machine.
#[derive(Debug)]
pub struct MikuVM<'a> {
    /// Points to the top of the current stackframe.
    stack_top: usize,
    /// Points to the base of the current stackframe.
    stack_base: usize,
    
    /// The RAM.
    /// An array of [`MikuType`].
    /// First 40% of it is the .data section reserved for constants.
    /// The rest of the memory is the heap.
    memory: [MikuType; MEMORY_SIZE],
    /// This points to the largest address where a non NULL value is stored in the .data section of
    /// the memory.
    largest_data_address: usize,
    /// This points to the largest address where a non NULL value is stored in the heap section of
    /// the memory.
    largest_heap_address: usize,

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
            stack_top: STACK_START, 
            stack_base: STACK_START,
            memory: [MikuType::NULL; MEMORY_SIZE],
            largest_data_address: DATA_START,
            largest_heap_address: HEAP_START,
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
    
    /// Write data in the .data section of the RAM.
    /// 
    /// # Returns
    /// - `Ok(())` if the data was successfully stored.
    /// - [`MikuError::UsedDataSpace`] if the .data section isn't [`MikuType::NULL`] at the given
    /// address.
    /// - [`MikuError::SegmentationFault`] if the given address is outside of the .data section's
    /// bounds or if the address is larger than the [`MEMORY_SIZE`].
    pub fn define_data(&mut self, data: MikuType, address: usize) -> Result<(), MikuError> {
        if address < DATA_START || address > DATA_END || address >= MEMORY_SIZE {
            return Err(MikuError::SegmentationFault);
        }
        
        match self.memory[address] {
            MikuType::NULL => self.memory[address] = data,
            _ => return Err(MikuError::UsedDataSpace),
        }

        if address > self.largest_data_address {
            self.largest_data_address = address
        }

        Ok(())
    }

    /// Dereference a the given address.
    /// Reads the contents of the RAM at the given address and returns a copy 
    /// of the read data.
    ///
    /// # Returns
    /// - `Ok(MikuType)` on successful read.
    /// - [`MikuError::SegmentationFault`] if the address is out of bounds or the pointer is [`MikuType::NULL`].
    /// - [`MikuError::InvalidPointerType`] if the pointer isn't a [`MikuType::U64`].
    pub fn deref_ptr(&self, ptr: MikuType) -> Result<MikuType, MikuError> {
        let address: usize = match ptr {
            MikuType::U64(address) => address as usize,
            MikuType::NULL => return Err(MikuError::SegmentationFault),
            _ => return Err(MikuError::InvalidPointerType(ptr)),
        };

        if address >= MEMORY_SIZE {
            return Err(MikuError::SegmentationFault);
        }

        Ok(self.memory[address])
    }
    
    /// Push a [`MikuType`] onto the stack.
    ///
    /// # Returns
    /// - `Ok(())` on successful push.
    /// - [`MikuError::StackOverflow`] if the stack is out of space.
    pub fn stack_push(&mut self, stack_entry: MikuType) -> Result<(), MikuError> {
        if self.stack_top == STACK_END {
            return Err(MikuError::StackOverflow);           
        }

        self.memory[self.stack_top] = stack_entry;
        self.stack_top += 1;
        Ok(())
    }
    
    /// Pops the top entry off the stack. 
    /// # Returns
    /// - `Ok(())` on successful pop.
    /// - [`MikuError::StackUnderflow`] if the stack is empty. 
    pub fn stack_pop(&mut self) -> Result<(), MikuError> {
        if self.stack_base == self.stack_top {
            return Err(MikuError::StackUnderflow);
        }

        self.stack_top -= 1;
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

    /// The stack memory.
    /// Returns a clone of the stack segment of the memory as a [`Vec`] of [`MikuType`].
    pub fn stack(&self) -> Vec<MikuType> {
        self.memory[STACK_START..STACK_END].to_vec()
    }
    
    /// The data memory.
    /// The data memory holds the constants of the program.
    /// Returns a clone of the data segment of the memory as a [`Vec`] of [`MikuType`].
    pub fn data_mem(&self) -> Vec<MikuType> {
        self.memory[DATA_START..DATA_END].to_vec()
    }
    
    /// The heap memory.
    /// The heap memroy holds the dynamically allocated data of the program.
    /// Returns a clone of the heap segment of the memory as a [`Vec`] of [`MikuType`].
    pub fn heap_mem(&self) -> Vec<MikuType> {
        self.memory[HEAP_START..HEAP_END].to_vec()
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

impl Display for MikuVM<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "----------- VM -----------\n  
            Program: {:?}\n  
            Stack: {:?}\n  
            Data: {:?}\n  
            Heap: {:?}", 
            self.program, 
            &self.memory[STACK_START..self.stack_top], 
            &self.memory[DATA_START..self.largest_data_address + 1], 
            &self.memory[HEAP_START..self.largest_heap_address])
    }
}
