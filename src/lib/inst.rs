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
    /// # Returns
    /// - `Ok(())` if the execution was successful.
    /// - [`MikuError`] if something goes wrong during the execution of the instruction.
    fn execute(&self, vm: &mut MikuVM) -> Result<(), MikuError>;

    /// Encodes the instruction into a [`Vec`] of bytes. 
    /// This is used for bytecode compilation.
    /// 
    /// # The preferred encoding format
    ///
    /// - The first byte is the opcode.
    /// - The rest of the bytes are the operands.
    fn encode(&self) -> Vec<u8>;
    
    /// Decodes the instruction from a slice of bytes.
    /// This is used when running already compiled code.
    ///
    /// # Expected encoding format is the same as at [`Inst::encode`]
    ///
    /// # Returns
    /// - `Ok(Self)` ([`Inst`]) if the decoding was successful.
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
#[derive(Debug, PartialEq)]
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
        vm.stack_push(self.operand)
    }
    
    /// # Example
    /// ``` rust
    /// let push = Push::new(MikuType::U8(69)); 
    /// let encoded_push = push.encode();
    /// assert_eq!(vec![0x00, 0x00, 0x45], encoded_push);
    /// ```
    fn encode(&self) -> Vec<u8> {
        let operand_bytes: Vec<u8> = Vec::from(self.operand);
        let opcode_byte: u8 = 0x00;
        let mut encoded_instruction = vec![opcode_byte];
        encoded_instruction.extend(operand_bytes);
        encoded_instruction
    }
    
    /// # Example
    /// ``` rust
    /// let encoded_push = vec![0x00, 0x00, 0x45];
    /// let decoded_push = Push::decode(&encoded_push).unwrap(); 
    /// assert_eq!(Push::new(MikuType::U8(69)), decoded_push);
    /// ```
    fn decode(bytes: &[u8]) -> Result<Self, MikuError> where Self: Sized {
        let operand = MikuType::try_from(&bytes[1..bytes.len()])?;
        Ok(Push::new(operand))
    }
}

 
/// # Pop instruction.
///
/// Pops and entry off the stack.
/// 
/// ## Information
/// - Opcode 1
/// - Operands:
///   - None
#[derive(Debug, PartialEq)]
pub struct Pop { }

impl Pop {
    pub fn new() -> Self {
        Self { }
    }
}

impl Inst for Pop {
    fn execute(&self, vm: &mut MikuVM) -> Result<(), MikuError> {
        vm.inc_pc();
        vm.stack_pop()
    }
    
    /// # Example
    ///
    /// ``` rust
    /// let pop = Pop::new();
    /// let encoded_pop = pop.encode();
    /// assert_eq!(vec![0x01], encoded_pop);
    /// ```
    fn encode(&self) -> Vec<u8> {
        let opcode_byte: u8 = 0x01;
        vec![opcode_byte]
    }
    
    /// # Example
    ///
    /// ``` rust
    /// assert_eq!(Pop::new(), Pop::decode(&vec![0x01]).unwrap());
    /// ```
    fn decode(bytes: &[u8]) -> Result<Self, MikuError> where Self: Sized {
        if bytes.len() != 1 {
            return Err(MikuError::BytesConversionError);
        }
        Ok(Pop::new())
    }
}
