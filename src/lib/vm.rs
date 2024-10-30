//! # The Miku Virtual Machine
//! This crate holds all the modules required for MikuVM and the MikuVM struct itself.
//! 
//! ## Instructions
//! * The instructions are impemented in the [`inst`] module.
//! 
//! | name | opcode | operand 1 | operand 2 | operand 3 |
//! | ---- | ------ | --------- | --------- | --------- |
//! | push | 0      | [`types::MikuType`] | - | - |
//! | pop  | 1      | - | - | - |
//! | def  | 2      | [`types::MikuType`] | address | - |

use std::usize;
 
pub const MEMORY_SIZE: usize = 1024;
/// The stack segment is 30% of the full memory size.
pub const STACK_START: usize = 0;
pub const STACK_END: usize = (MEMORY_SIZE / 100) * 30;
/// The .data segment is 30% of the full memory size.
pub const DATA_START: usize = STACK_END + 1;
pub const DATA_END: usize = DATA_START + (MEMORY_SIZE / 100) * 30;
/// The heap segment is 40% of the full memory size.
pub const HEAP_START: usize = DATA_END + 1;
pub const HEAP_END: usize = MEMORY_SIZE;

pub mod error;
pub mod tools;
pub mod inst;
pub mod miku;
pub mod types;

#[cfg(test)]
mod tests;
