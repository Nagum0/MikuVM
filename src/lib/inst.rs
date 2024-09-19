use crate::miku::Miku;
use crate::stack::StackEntry;

#[derive(Debug, Clone, Copy)]
pub enum Inst {
    Push(StackEntry),
    Pop,
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
        }

        bytes
    }

    /// Takes a slice of bytes and turns them into an Inst.
    pub fn from_bytes(bytes: &[u8]) -> Inst {
        match bytes[0] {
            0x00 => Inst::Push(StackEntry::from_bytes(&bytes[1..bytes.len()])),
            0x01 => Inst::Pop,
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
        }
    }
}
