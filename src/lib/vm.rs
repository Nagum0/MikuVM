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

#[derive(Debug, Clone, Copy)]
pub enum Inst {
    Push(StackEntry),
    Pop,
    PrintCharDbg,
}

impl Inst {
    /// Takes an Inst and turns it into a vector of bytes.
    /// * First byte is the instruction number the rest are the operands values in bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            Self::Push(operand) => {
                bytes.push(0x00);
                bytes.extend(operand.to_bytes());
            }
            Self::Pop => bytes.push(0x01),
            Self::PrintCharDbg => bytes.push(0x02),
        }

        bytes
    }

    /// Takes a slice of bytes and turns them into an Inst.
    pub fn from_bytes(bytes: &[u8]) -> Inst {
        match bytes[0] {
            0x00 => Inst::Push(StackEntry::from_bytes(&bytes[1..bytes.len()])),
            0x01 => Inst::Pop,
            0x02 => Inst::PrintCharDbg,
            _ => panic!("UNKNOWN INSTRUCTION: {}", bytes[0]),
        }
    }

    pub fn execute(&self, miku: &mut Miku) {
        match self {
            Self::Push(operand) => {
                miku.stack.push(*operand);
            }
            Self::Pop => {
                if miku.stack.is_empty() {
                    panic!("STACK UNDERFLOW");
                }
                miku.stack.pop().unwrap();
            }
            Self::PrintCharDbg => {
                if miku.stack.is_empty() {
                    panic!("STACK UNDERFLOW");
                }

                match miku.stack[miku.stack.len() - 1] {
                    StackEntry::U8(value) => println!("{}", value as char),
                    _ => panic!("TOP VALUE ON STACK IS NOT A U8"),
                }
            }
        }
    }
}

pub struct Miku {
    pub program: Vec<Inst>,
    pub stack: Vec<StackEntry>,
}

impl Miku {
    pub fn new() -> Miku {
        Miku {
            program: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn run_program(&mut self) {
        for i in 0..self.program.len() {
            let inst = self.program[i];
            inst.execute(self);
        }
        self.dump_stack();
    }

    fn dump_stack(&self) {
        println!("Stack ({})", self.stack.len());
        for entry in &self.stack {
            println!("  {:?}", entry);
        }
    }
}
