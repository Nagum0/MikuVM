//! # The Miku Virtual Machine
//! This crate holds all the modules required for MikuVM and the MikuVM struct itself.

pub mod error;
pub mod tools;
pub mod inst;
pub mod miku;
pub mod types;

#[cfg(test)]
mod tests;
