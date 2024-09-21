use crate::miku::Miku;
use crate::stack::StackEntry;

#[derive(Debug, Clone, Copy)]
pub enum Inst {
    // Stack operations
    Push(StackEntry),
    Pop,
    Dup(usize),
    Swap,
    Plus,
    Minus,
    Mult,
    Div,
    Eq,
    Jmp(usize),
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
            Self::Plus => bytes.push(0x04),
            Self::Minus => bytes.push(0x05),
            Self::Mult => bytes.push(0x06),
            Self::Div => bytes.push(0x07),
            Self::Eq => bytes.push(0x08),
            Self::Jmp(operand) => {
                bytes.push(0x09);
                bytes.extend(operand.to_le_bytes());
            }
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
            0x04 => Inst::Plus,
            0x05 => Inst::Minus,
            0x06 => Inst::Mult,
            0x07 => Inst::Div,
            0x08 => Inst::Eq,
            0x09 => Inst::Jmp(usize::from_le_bytes(
                bytes[1..bytes.len()].try_into().unwrap(),
            )),
            _ => panic!("UNKNOWN INSTRUCTION: {}", bytes[0]),
        }
    }

    pub fn execute(&self, miku: &mut Miku) {
        match self {
            Self::Push(operand) => {
                if miku.stack_top == miku.stack.len() {
                    miku.stack.push(*operand);
                } else {
                    miku.stack[miku.stack_top] = *operand;
                }

                miku.stack_top += 1;
                miku.ins_ptr += 1;
            }
            Self::Pop => {
                if miku.stack_top == miku.stack_base {
                    panic!("STACK UNDERFLOW");
                }

                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Dup(operand) => {
                if miku.stack_top == miku.stack_base {
                    panic!("STACK UNDERFLOW");
                }

                if *operand >= miku.stack_top {
                    panic!("STACK OVERFLOW");
                }

                let offset = miku.stack_top - 1 - *operand;

                if miku.stack_top == miku.stack.len() {
                    miku.stack.push(miku.stack[offset]);
                } else {
                    miku.stack[miku.stack_top] = miku.stack[offset];
                }

                miku.stack_top += 1;
                miku.ins_ptr += 1;
            }
            Self::Swap => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                miku.stack.swap(miku.stack_top - 1, miku.stack_top - 2);
                miku.ins_ptr += 1;
            }
            Self::Plus => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();
                miku.stack.push(StackEntry::add(a, b));
                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Minus => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();
                miku.stack.push(StackEntry::subtract(a, b));
                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Mult => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();
                miku.stack.push(StackEntry::multiply(a, b));
                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Div => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();
                miku.stack.push(StackEntry::divide(a, b));
                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Eq => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();

                if StackEntry::eq(a, b) {
                    miku.stack.push(StackEntry::U8(0));
                } else {
                    miku.stack.push(StackEntry::U8(1));
                }

                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Jmp(operand) => {
                if *operand >= miku.program.len() {
                    panic!("JUMP OUT OF BOUNDS: {}", operand);
                }

                miku.ins_ptr = *operand;
            }
        }
    }
}
