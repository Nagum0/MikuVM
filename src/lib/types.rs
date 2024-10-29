use crate::{error::MikuError, tools};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MikuType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

macro_rules! impl_arith_trait {
    ($operation: ident, $method: ident) => {
        impl $operation for MikuType {
            type Output = Result<MikuType, MikuError>;
            
            fn $method(self, rhs: Self) -> Self::Output {
                impl_arith_operations!(self, $method, rhs)
            }
        }
    }
}

macro_rules! impl_arith_operations {
    ($self: ident, $method: ident, $rhs: ident) => {
        match ($self, $rhs) {
            (MikuType::U8(a), MikuType::U8(b))   => Ok(MikuType::U8(a.$method(b))),
            (MikuType::U16(a), MikuType::U16(b)) => Ok(MikuType::U16(a.$method(b))),
            (MikuType::U32(a), MikuType::U32(b)) => Ok(MikuType::U32(a.$method(b))),
            (MikuType::U64(a), MikuType::U64(b)) => Ok(MikuType::U64(a.$method(b))),
            (MikuType::I8(a), MikuType::I8(b))   => Ok(MikuType::I8(a.$method(b))),
            (MikuType::I16(a), MikuType::I16(b)) => Ok(MikuType::I16(a.$method(b))),
            (MikuType::I32(a), MikuType::I32(b)) => Ok(MikuType::I32(a.$method(b))),
            (MikuType::I64(a), MikuType::I64(b)) => Ok(MikuType::I64(a.$method(b))),
            (MikuType::F32(a), MikuType::F32(b)) => Ok(MikuType::F32(a.$method(b))),
            (MikuType::F64(a), MikuType::F64(b)) => Ok(MikuType::F64(a.$method(b))),
            _ => Err(MikuError::UndefinedOperationBetweenTypesError(format!("{}({:?}, {:?})", stringify!(method), $self, $rhs))),
        }
    }
}

/// Macro for implementing to_bytes for MikuType.
macro_rules! match_to_bytes {
    ($self: expr, { $ ( $variant: ident => $tag: expr), * }) => {{
        let mut bytes = Vec::new();
        
        match $self {
            $(
                MikuType::$variant(value) => {
                    bytes.push($tag);
                    bytes.extend(value.to_le_bytes());
                }
            )*
        }
        
        bytes
    }};
}

/// Macro for implementing from_bytes for MikuType.
macro_rules! match_from_bytes {
    ($type_identifier_byte: expr, $le_bytes: expr, { $ ($tag: expr => $variant: ident | $type: ident), * }) => {
        match $type_identifier_byte {
            $(
                $tag => Ok(Self::$variant($type::from_le_bytes(tools::convert_bytes($le_bytes)?))),
            )*
            _ => Err(MikuError::UnknownTypeError($type_identifier_byte)),
        }
    };
}

/// Takes a MikuType and turns it into a vector of bytes.
/// * First byte is the type and the rest are the value.
impl From<MikuType> for Vec<u8> {
	fn from(value: MikuType) -> Self {
		match_to_bytes!(
            value, { U8 => 0x00, U16 => 0x01, U32 => 0x02, U64 => 0x03,
                     I8 => 0x04, I16 => 0x05, I32 => 0x06, I64 => 0x07,
                     F32 => 0x08, F64 => 0x09 }
        )
	}
}

/// Takes a slice of bytes and turns them into a MikuType.
impl TryFrom<&[u8]> for MikuType {
	type Error = MikuError;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		let type_identifier_byte = value[0];
        let le_bytes = &value[1..];

        match_from_bytes!(
            type_identifier_byte, le_bytes, 
            { 0x00 => U8 | u8, 0x01 => U16 | u16, 0x02 => U32 | u32, 0x03 => U64 | u64,
              0x04 => I8 | i8, 0x05 => I16 | i16, 0x06 => I32 | i32, 0x07 => I64 | i64,
              0x08 => F32 | f32, 0x09 => F64 | f64
        })
	}
}

impl_arith_trait!(Add, add);
impl_arith_trait!(Sub, sub);
impl_arith_trait!(Mul, mul);

impl Div for MikuType {
    type Output = Result<MikuType, MikuError>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == MikuType::U8(0) || rhs == MikuType::U16(0) || rhs == MikuType::U32(0) || rhs == MikuType::U64(0) ||
            rhs == MikuType::I8(0) || rhs == MikuType::I16(0) || rhs == MikuType::I32(0) || rhs == MikuType::I64(0) ||
            rhs == MikuType::F32(0.0) || rhs == MikuType::F64(0.0)
        {
            return Err(MikuError::DivisionByZeroError);
        }

        impl_arith_operations!(self, div, rhs)
    }
}

impl MikuType {
    /// Takes a slice of string slices and turns them into a MikuType.
    /// ! Temporary
    /// ### Panics
    pub fn from_strs(strs: &[&str]) -> MikuType {
        if strs.len() != 2 {
            panic!("EXPECTED: <type> <value>\nRECEIVED: {:?}", strs);
        }

        match strs[0] {
            "u8"  => MikuType::U8(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u16" => MikuType::U16(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u32" => MikuType::U32(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u64" => MikuType::U64(strs[1].parse().expect("EXPECTED A NUMBER")),
            "i8"  => MikuType::I8(strs[1].parse().expect("EXPECTED A NUMBER")),
            "i16" => MikuType::I16(strs[1].parse().expect("EXPECTED A NUMBER")),
            "i32" => MikuType::I32(strs[1].parse().expect("EXPECTED A NUMBER")),
            "i64" => MikuType::I64(strs[1].parse().expect("EXPECTED A NUMBER")),
            _ => panic!("UNKNOWN TYPE: {}", strs[0]),
        }
    }
}
