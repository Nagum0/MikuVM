//! # Registers
//! 
//! The MikuVM registers work exaclty like other registers.
//! They hold temporary data during you program.
//! Each register has it's own identifier ([`Register::ident`]).
//! Each register holds a single [`MikuType`] as it's value.

use crate::types::MikuType;

/// # Register
///
/// Each register has it's own identifier ([`Register::ident`]).
/// Each register holds a single [`MikuType`] as it's value.
#[derive(Debug, Clone, Copy)]
pub struct Register {
    ident: u8,
    value: MikuType,
}

impl Register {
    pub fn new(ident: u8, value: MikuType) -> Self {
        Self { ident, value }
    }
    
    /// Returns the identifier of the register.
    pub fn ident(&self) -> u8 {
        self.ident
    }

    /// Returns the value that the register holds.
    pub fn value(&self) -> MikuType {
        self.value
    }
    
    /// Set the value of the register.
    pub fn set(&mut self, value: MikuType) {
        self.value = value;
    }
}
