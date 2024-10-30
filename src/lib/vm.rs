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

use std::usize;
 
/// Constants
pub const STACK_MAX_SIZE: usize = 1024;
pub const MEMORY_SIZE: usize = 1024;
pub const DATA_START: usize = 0;
pub const DATA_END: usize = (MEMORY_SIZE / 100) * 40;
pub const HEAP_START: usize = DATA_END + 1;
pub const HEAP_END: usize = MEMORY_SIZE;

pub mod error;
pub mod tools;
pub mod inst;
pub mod miku;
pub mod types;

#[cfg(test)]
mod tests;
