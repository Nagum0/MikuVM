//! # Instructions for the virtual machine.
//! 
//! This is achieved in a more OOP style unlike [`MikuType`]. 
//! Each instruction implements the [`Inst`] trait and the VM itself
//! holds a vector of elements that implement this trait. This is achieved with
//! dynamic dispatching.

use std::fmt::Debug;

use crate::{
    error::MikuError, miku::MikuVM, types::MikuType
};

/// # The instruction trait.
///
/// This trait needs to be implemented by anything that wants to be executed by [`MikuVM`].
pub trait Inst: Debug {
    /// This method gets called by [`MikuVM::run_program()`] when the instruction needs to be
    /// executed.
    /// ### Results in
    /// - () 
    /// - [`MikuError`] if something goes wrong during the execution of the instruction.
    fn execute(&self, vm: &mut MikuVM) -> Result<(), MikuError>;
    /// Encodes the instruction into a [`Vec`] of bytes. 
    /// This is used for bytecode compilation.
    fn encode(&self) -> Vec<u8>;
    /// Decodes the instruction from a slice of bytes.
    /// This is used when running already compiled code.
    /// ### Result in
    /// - Self ([`Inst`])
    /// - [`MikuError`] if something goes wrong during decoding.
    fn decode(bytes: &[u8]) -> Result<Self, MikuError> where Self: Sized;
}

/// # Push instruction.
///
/// Pushes a [`MikuType`] onto the stack.
///
/// ## Information
/// - Opcode: 0
/// - Operands: 
///   - [`MikuType`]
#[derive(Debug)]
pub struct Push {
    operand: MikuType,
}

impl Push {
    pub fn new(operand: MikuType) -> Self {
        Self { operand }
    }
}

impl Inst for Push {
    fn execute(&self, vm: &mut MikuVM) -> Result<(), MikuError> {
        vm.inc_pc();
        vm.stack_push(self.operand);
        Ok(())
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn decode(bytes: &[u8]) -> Result<Self, MikuError> where Self: Sized {
        todo!()
    }
}
