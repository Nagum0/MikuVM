use crate::{error::MikuError, tools, traits::AsBytes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MikuType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl AsBytes for MikuType {
    /// Takes a MikuType and turns it into a vector of bytes.
    /// * First byte is the type and the rest are the value.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            Self::U8(value) => {
                bytes.push(0x00);
                bytes.extend(value.to_le_bytes());
            }
            Self::U16(value) => {
                bytes.push(0x01);
                bytes.extend(value.to_le_bytes());
            }
            Self::U32(value) => {
                bytes.push(0x02);
                bytes.extend(value.to_le_bytes());
            }
            Self::U64(value) => {
                bytes.push(0x03);
                bytes.extend(value.to_le_bytes());
            }
        }

        bytes
    }

    /// Takes a slice of bytes and turns them into a MikuType.
    fn from_bytes(bytes: &[u8]) -> Result<Self, crate::error::MikuError> 
    where 
        Self: Sized 
    {
        let type_identifier_byte = bytes[0];
        let le_bytes = &bytes[1..bytes.len()];

        match type_identifier_byte {
            0x00 => Ok(Self::U8(u8::from_le_bytes(tools::convert_bytes(le_bytes)?))),
            0x01 => Ok(Self::U16(u16::from_le_bytes(tools::convert_bytes(le_bytes)?))),
            0x03 => Ok(Self::U32(u32::from_le_bytes(tools::convert_bytes(le_bytes)?))),
            0x02 => Ok(Self::U64(u64::from_le_bytes(tools::convert_bytes(le_bytes)?))),
            _ => Err(MikuError::UnknownTypeError(type_identifier_byte)),
        }
    }
}

/// -- FIX THE REPETITIVE CODE:
impl MikuType {
    fn to_u64(&self) -> u64 {
        todo!()
    }

    fn from_u64(value: u64) -> MikuType {
        todo!()
    }

    fn operation<F, U>(a: MikuType, b: MikuType, op: F) -> MikuType 
    where 
        F: Fn(u64, u64) -> u64
    {
        if std::mem::discriminant(&a) != std::mem::discriminant(&b) {
            panic!("TYPES DO NOT MATCH");
        }

        let result = op(a.to_u64(), b.to_u64());
        MikuType::from_u64(result)
    }

    /// Adds 2 stack entry values together and returns the result.
    pub fn add(a: MikuType, b: MikuType) -> Result<MikuType, MikuError> {
        match (a, b) {
            (Self::U8(a), Self::U8(b))     => Ok(Self::U8(a + b)),
            (Self::U16(a), Self::U16(b)) => Ok(Self::U16(a + b)),
            (Self::U32(a), Self::U32(b)) => Ok(Self::U32(a + b)),
            (Self::U64(a), Self::U64(b)) => Ok(Self::U64(a + b)),
            _ => Err(MikuError::UndefinedOperationBetweenTypesError(format!("Cannot add {:?} to {:?}", a, b))),
        }
    }

    /// Subtracts 2 stack entry values and returns the result.
    pub fn subtract(a: MikuType, b: MikuType) -> Result<MikuType, MikuError> {
        match (a, b) {
            (Self::U8(a), Self::U8(b))     => Ok(Self::U8(a - b)),
            (Self::U16(a), Self::U16(b)) => Ok(Self::U16(a - b)),
            (Self::U32(a), Self::U32(b)) => Ok(Self::U32(a - b)),
            (Self::U64(a), Self::U64(b)) => Ok(Self::U64(a - b)),
            _ => Err(MikuError::UndefinedOperationBetweenTypesError(format!("Cannot subtract {:?} from {:?}", a, b))),
        }
    }

    /// Multiplies 2 stack entries and returns the result.
    pub fn multiply(a: MikuType, b: MikuType) -> Result<MikuType, MikuError> {
        match (a, b) {
            (Self::U8(a), Self::U8(b))     => Ok(Self::U8(a * b)),
            (Self::U16(a), Self::U16(b)) => Ok(Self::U16(a * b)),
            (Self::U32(a), Self::U32(b)) => Ok(Self::U32(a * b)),
            (Self::U64(a), Self::U64(b)) => Ok(Self::U64(a * b)),
            _ => Err(MikuError::UndefinedOperationBetweenTypesError(format!("Cannot multiply {:?} with {:?}", a, b))),
        }
    }

    /// Divides 2 stack entries and returns the result. Panics if b is 0.
    pub fn divide(a: MikuType, b: MikuType) -> Result<MikuType, MikuError> {
        if Self::is_zero(b) {
            return Err(MikuError::DivisionByZeroError);
        }
        
        match (a, b) {
            (Self::U8(a), Self::U8(b))     => Ok(Self::U8(a / b)),
            (Self::U16(a), Self::U16(b)) => Ok(Self::U16(a / b)),
            (Self::U32(a), Self::U32(b)) => Ok(Self::U32(a / b)),
            (Self::U64(a), Self::U64(b)) => Ok(Self::U64(a / b)),
            _ => Err(MikuError::UndefinedOperationBetweenTypesError(format!("Cannot divide {:?} by {:?}", a, b))),
        }
    }

    pub fn eq(a: MikuType, b: MikuType) -> Result<bool, MikuError> {
        match (a, b) {
            (Self::U8(a), Self::U8(b))     => Ok(a == b),
            (Self::U16(a), Self::U16(b)) => Ok(a == b),
            (Self::U32(a), Self::U32(b)) => Ok(a == b),
            (Self::U64(a), Self::U64(b)) => Ok(a == b),
            _ => Err(MikuError::UndefinedOperationBetweenTypesError(format!("Cannot check equality between {:?} and {:?}", a, b))),
        }
    }

    /// Takes a slice of string slices and turns them into a MikuType.
    /// ! Temporary
    /// ### Panics
    pub fn from_strs(strs: &[&str]) -> MikuType {
        if strs.len() != 2 {
            panic!("EXPECTED: <type> <value>\nRECEIVED: {:?}", strs);
        }

        match strs[0] {
            "u8" => MikuType::U8(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u16" => MikuType::U16(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u32" => MikuType::U32(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u64" => MikuType::U64(strs[1].parse().expect("EXPECTED A NUMBER")),
            _ => panic!("UNKNOWN TYPE: {}", strs[0]),
        }
    }

    /// Checks whether a is 0.
    fn is_zero(a: MikuType) -> bool {
        match a {
            MikuType::U8(a)   => a == 0,
            MikuType::U16(a) => a == 0,
            MikuType::U32(a) => a == 0,
            MikuType::U64(a) => a == 0,
        }
    }
}
