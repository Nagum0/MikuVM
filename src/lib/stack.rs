#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackEntry {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl StackEntry {
    /// Adds 2 stack entry values together and returns the result.
    pub fn add(a: StackEntry, b: StackEntry) -> StackEntry {
        match (a, b) {
            (Self::U8(a), Self::U8(b)) => Self::U8(a + b),
            (Self::U16(a), Self::U16(b)) => Self::U16(a + b),
            (Self::U32(a), Self::U32(b)) => Self::U32(a + b),
            (Self::U64(a), Self::U64(b)) => Self::U64(a + b),
            _ => panic!("ADDITION BETWEEN SEPARATE TYPES: {:?} + {:?}", a, b),
        }
    }

    /// Subtracts 2 stack entry values and returns the result.
    pub fn subtract(a: StackEntry, b: StackEntry) -> StackEntry {
        match (a, b) {
            (Self::U8(a), Self::U8(b)) => Self::U8(a - b),
            (Self::U16(a), Self::U16(b)) => Self::U16(a - b),
            (Self::U32(a), Self::U32(b)) => Self::U32(a - b),
            (Self::U64(a), Self::U64(b)) => Self::U64(a - b),
            _ => panic!("SUBTRACTION BETWEEN SEPARATE TYPES: {:?} - {:?}", a, b),
        }
    }

    /// Multiplies 2 stack entries and returns the result.
    pub fn multiply(a: StackEntry, b: StackEntry) -> StackEntry {
        match (a, b) {
            (Self::U8(a), Self::U8(b)) => Self::U8(a * b),
            (Self::U16(a), Self::U16(b)) => Self::U16(a * b),
            (Self::U32(a), Self::U32(b)) => Self::U32(a * b),
            (Self::U64(a), Self::U64(b)) => Self::U64(a * b),
            _ => panic!("MULTIPLICATION BETWEEN SEPARATE TYPES: {:?} * {:?}", a, b),
        }
    }

    /// Divides 2 stack entries and returns the result. Panics if b is 0.
    pub fn divide(a: StackEntry, b: StackEntry) -> StackEntry {
        match (a, b) {
            (Self::U8(a), Self::U8(b)) => {
                if b == 0 {
                    panic!("DIVISION BY ZERO");
                }
                Self::U8(a / b)
            }
            (Self::U16(a), Self::U16(b)) => {
                if b == 0 {
                    panic!("DIVISION BY ZERO");
                }
                Self::U16(a / b)
            }
            (Self::U32(a), Self::U32(b)) => {
                if b == 0 {
                    panic!("DIVISION BY ZERO");
                }
                Self::U32(a / b)
            }
            (Self::U64(a), Self::U64(b)) => {
                if b == 0 {
                    panic!("DIVISION BY ZERO");
                }
                Self::U64(a / b)
            }
            _ => panic!("DIVISION BETWEEN SEPARATE TYPES: {:?} / {:?}", a, b),
        }
    }

    pub fn eq(a: StackEntry, b: StackEntry) -> bool {
        match (a, b) {
            (Self::U8(a), Self::U8(b)) => a == b,
            (Self::U16(a), Self::U16(b)) => a == b,
            (Self::U32(a), Self::U32(b)) => a == b,
            (Self::U64(a), Self::U64(b)) => a == b,
            _ => panic!(
                "EQUALITY CHECKING BETWEEN SEPARATE TYPES: {:?} == {:?}",
                a, b
            ),
        }
    }

    /// Takes a StackEntry and turns it into a vector of bytes.
    /// * First byte is the type and the rest are the value.
    pub fn to_bytes(&self) -> Vec<u8> {
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

    /// Takes a slice of bytes and turns them into a StackEntry.
    pub fn from_bytes(bytes: &[u8]) -> StackEntry {
        match bytes[0] {
            0x00 => Self::U8(u8::from_le_bytes(bytes[1..bytes.len()].try_into().unwrap())),
            0x01 => Self::U16(u16::from_le_bytes(
                bytes[1..bytes.len()].try_into().unwrap(),
            )),
            0x02 => Self::U32(u32::from_le_bytes(
                bytes[1..bytes.len()].try_into().unwrap(),
            )),
            0x03 => Self::U64(u64::from_le_bytes(
                bytes[1..bytes.len()].try_into().unwrap(),
            )),
            _ => panic!("UNEXPECTED TYPE"),
        }
    }

    /// Takes a slice of string slices and turns them into a StackEntry.
    pub fn from_strs(strs: &[&str]) -> StackEntry {
        if strs.len() != 2 {
            panic!("EXPECTED: <type> <value>\nRECEIVED: {:?}", strs);
        }

        match strs[0] {
            "u8" => StackEntry::U8(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u16" => StackEntry::U16(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u32" => StackEntry::U32(strs[1].parse().expect("EXPECTED A NUMBER")),
            "u64" => StackEntry::U64(strs[1].parse().expect("EXPECTED A NUMBER")),
            _ => panic!("UNKNOWN TYPE: {}", strs[0]),
        }
    }
}
