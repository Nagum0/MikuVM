#[derive(Debug, Clone, Copy)]
pub enum StackEntry {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl StackEntry {
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
