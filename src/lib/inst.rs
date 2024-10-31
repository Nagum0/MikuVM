//! # Instructions for the virtual machine.
//! 
//! This is achieved in a more OOP style unlike [`MikuType`]. 
//! Each instruction implements the [`Inst`] trait and the VM itself
//! holds a vector of elements that implement this trait. This is achieved with
//! dynamic dispatching.

use std::{fmt::Debug, usize};
use either::Either;
use crate::{
    error::MikuError, miku::MikuVM, tools, types::MikuType
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

/// # Def instruction.
///
/// It takes a [`MikuType`] and writes it to the .data section of the memory at the given address.
///
/// ## Information
/// - Opcode: 2
/// - Operands:
///   - [`MikuType`]
///   - address ([`prim@usize`])
#[derive(Debug, PartialEq)]
pub struct Def {
    operand_1: MikuType,
    opreand_2: usize,
}

impl Def {
    pub fn new(operand_1: MikuType, opreand_2: usize) -> Self {
        Self { operand_1, opreand_2 }
    }
}

impl Inst for Def {
    fn execute(&self, vm: &mut MikuVM) -> Result<(), MikuError> {
        vm.inc_pc();
        vm.define_data(self.operand_1, self.opreand_2)
    }
    
    /// # Example
    ///
    /// ``` rust
    /// let def = Def::new(MikuType::U8(69), 1);
    /// let encoded_def = def.encode();
    /// assert_eq!(
    ///    vec![0x02, 0x00, 0x45, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///    encoded_def
    /// );
    /// ```
    fn encode(&self) -> Vec<u8> {
        let opcode: u8 = 0x02;
        let operand_1_bytes = Vec::from(self.operand_1);
        let operand_2_bytes = self.opreand_2.to_le_bytes();
        let mut encoded_def = vec![opcode];
        encoded_def.extend(operand_1_bytes);
        encoded_def.extend(operand_2_bytes);
        encoded_def
    }
    
    /// # Example
    ///
    /// ``` rust
    /// assert_eq!(Def::new(MikuType::U8(69)), Def::decode(&vec![0x02, 0x00, 0x45, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]));
    /// ```
    fn decode(bytes: &[u8]) -> Result<Self, MikuError> where Self: Sized {
        let operand_1_length: usize = MikuType::get_bytes_length(bytes[1])?;
        let operand_1 = MikuType::try_from(&bytes[1..operand_1_length + 1])?;
        let opreand_2 = usize::from_le_bytes(tools::convert_bytes(&bytes[operand_1_length + 1..bytes.len()])?);
        Ok(Def::new(operand_1, opreand_2))
    }
}

/// # Set instruction.
///
/// Set the value of a register. The first operand is a register identifier as a [`prim@usize`].
/// The the second operand is either a [`MikuType`] or a [`prim@usize`] for a register identifier.
///
/// ## Flags
/// - op2_is_deref: Indicates that the second operand is a pointer to a memory address and requires
/// derefencing.
#[derive(Debug, PartialEq)]
pub struct Set {
    operand_1: usize,
    operand_2: Either<MikuType, usize>,
    op2_is_deref: bool,
}

impl Set {
    pub fn new(operand_1: usize, operand_2: Either<MikuType, usize>, op2_is_deref: bool) -> Self {
        Self { operand_1, operand_2, op2_is_deref }
    }
}

impl Inst for Set {
    /// !!!!!!!!!!!!! THIS IS UGLY FIX LATER 
    fn execute(&self, vm: &mut MikuVM) -> Result<(), MikuError> {
        vm.inc_pc();

        if self.operand_2.is_left() {
            if self.op2_is_deref {
                vm.set_register(self.operand_1, vm.deref_ptr(self.operand_2.left().unwrap())?)?;
            }
            else {
                vm.set_register(self.operand_1, self.operand_2.left().unwrap())?;
            }
        }
        else {
            if self.op2_is_deref {
                vm.set_register(self.operand_1, vm.deref_ptr(vm.read_register(self.operand_2.right().unwrap()).unwrap())?)?;    
            }
            else {
                vm.set_register(self.operand_1, vm.read_register(self.operand_2.right().unwrap()).unwrap())?;
            }
        }

        Ok(())
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn decode(bytes: &[u8]) -> Result<Self, MikuError> where Self: Sized {
        todo!()
    }
}
