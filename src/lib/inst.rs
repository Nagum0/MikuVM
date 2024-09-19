use crate::miku::Miku;
use crate::stack::StackEntry;

#[derive(Debug, Clone, Copy)]
pub enum Inst {
    // Stack operations
    Push(StackEntry),
    Pop,
    Dup(usize),
    Swap,
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
            Self::Dup(operand) => {
                bytes.push(0x02);
                bytes.extend(operand.to_le_bytes());
            }
            Self::Swap => bytes.push(0x03),
        }

        bytes
    }

    /// Takes a slice of bytes and turns them into an Inst.
    pub fn from_bytes(bytes: &[u8]) -> Inst {
        match bytes[0] {
            0x00 => Inst::Push(StackEntry::from_bytes(&bytes[1..bytes.len()])),
            0x01 => Inst::Pop,
            0x02 => Inst::Dup(usize::from_le_bytes(
                bytes[1..bytes.len()]
                    .try_into()
                    .expect("COULD NOT CONVERT OPERAND AT DUP"),
            )),
            0x03 => Inst::Swap,
            _ => panic!("UNKNOWN INSTRUCTION: {}", bytes[0]),
        }
    }

    pub fn execute(&self, miku: &mut Miku) {
        match self {
            Self::Push(operand) => {
                miku.stack.push(*operand);
                miku.stack_top += 1;
            }
            Self::Pop => {
                if miku.stack.is_empty() {
                    panic!("STACK UNDERFLOW");
                }

                miku.stack.pop().unwrap();
                miku.stack_top -= 1;
            }
            Self::Dup(operand) => {
                if miku.stack.is_empty() {
                    panic!("STACK UNDERFLOW");
                }

                if *operand >= miku.stack.len() {
                    panic!("STACK OVERFLOW");
                }

                miku.stack.push(miku.stack[*operand]);
            }
            Self::Swap => {
                if miku.stack.len() < 2 {
                    panic!("STACK UNDERFLOW");
                }

                miku.stack.swap(miku.stack_top - 1, miku.stack_top - 2);
            }
        }
    }
}
