//! # The Miku Virtual Machine
//! This crate holds all the modules required for MikuVM and the MikuVM struct itself.
//!
//! ## Memory
//! - The memory is an array of [`types::MikuType`] with the size of [`MEMORY_SIZE`]. This
//! size is set to `1024` but can be changed manually in the source code.
//! - The memory is separated into 3 segments. See below.
//!
//! ### Stack
//! - The stack can be used to hold temporary data.
//! - The stack grows upwards from a lower address to a higher one.
//! - The stack is **30%** of the entire memory and the first segment of it.
//! - Lowest address: [`STACK_START`]
//! - Highest address: [`STACK_END`]
//!
//! ### Data
//! - This part of the memory is used for constants and static data for the program.
//! - It takes up **30%** of the entire memory and this comes after the stack.
//! - Lowest address: [`STACK_START`]
//! - Highest address: [`STACK_END`]
//!
//! ### Heap
//! - [ ] TODO!
//! 
//! ## Registers
//! - Refer to the [`register`] module for the implementation.
//! - The VM has 6 general purpose registers.
//!   - A
//!   - B
//!   - C
//!   - D
//!   - E
//!   - F
//! - Each register has it's own identifier which is represented as a [`u8`] and a value which is a
//! [`types::MikuType`].
//!
//! ## Instructions
//! - The instructions are impemented in the [`inst`] module.
//! 
//! | name | opcode | operand 1 | operand 2 | operand 3 |
//! | ---- | ------ | --------- | --------- | --------- |
//! | push | 0      | [`types::MikuType`] | - | - |
//! | pop  | 1      | - | - | - |
//! | def  | 2      | [`types::MikuType`] | address | - |
//! | set  | 3      | register identifier ([`prim@usize`]) | [`types::MikuType`] or register identifier ([`prim@usize`]) | - |

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
pub const REGISTER_COUNT: usize = 6;

pub mod error;
pub mod tools;
pub mod inst;
pub mod miku;
pub mod types;
pub mod register;

#[cfg(test)]
mod tests;
