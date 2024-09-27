use crate::miku::Miku;
use crate::stack::MikuType;

#[derive(Debug, Clone, Copy)]
pub enum Inst {
    // Stack operations
    Push(MikuType),
    Pop,
    DupT(usize),
    Swap,
    Plus,
    Minus,
    Mult,
    Div,
    Eq,
    Jmp(usize),
    JmpZ(usize),
    JmpNZ(usize),
    DupB(usize),

    // Functions
    Call(usize), 
    Ret,
    RetV,
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
            Self::DupT(operand) => {
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
            Self::JmpZ(operand) => {
                bytes.push(0x0A);
                bytes.extend(operand.to_le_bytes());
            }
            Self::JmpNZ(operand) => {
                bytes.push(0x0B);
                bytes.extend(operand.to_le_bytes());
            }
            Self::DupB(operand) => {
                bytes.push(0x0C);
                bytes.extend(operand.to_le_bytes());
            }
            Self::Call(operand) => {
                bytes.push(0x0D);
                bytes.extend(operand.to_le_bytes());
            }
            Self::Ret => bytes.push(0x0E),
            Self::RetV => bytes.push(0x0F),
        }

        bytes
    }

    /// Takes a slice of bytes and turns them into an Inst.
    pub fn from_bytes(bytes: &[u8]) -> Inst {
        match bytes[0] {
            0x00 => Inst::Push(MikuType::from_bytes(&bytes[1..bytes.len()])),
            0x01 => Inst::Pop,
            0x02 => Inst::DupT(usize::from_le_bytes(
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
                bytes[1..bytes.len()]
                    .try_into()
                    .expect("COULD NOT CONVERT OPERAND AT JMP"),
            )),
            0x0A => Inst::JmpZ(usize::from_le_bytes(
                bytes[1..bytes.len()]
                    .try_into()
                    .expect("COULD NOT CONVERT OPERAND AT JMPZ"),
            )),
            0x0B => Inst::JmpNZ(usize::from_le_bytes(
                bytes[1..bytes.len()]
                    .try_into()
                    .expect("COULD NOT CONVERT OPERAND AT JMPNZ"),
            )),
            0x0C => Inst::DupB(usize::from_le_bytes(
                bytes[1..bytes.len()]
                    .try_into()
                    .expect("COULD NOT CONVERT OPERAND AT DUPB"),
            )),
            0x0D => Inst::Call(usize::from_le_bytes(
                bytes[1..bytes.len()]
                    .try_into()
                    .expect("COULD NOT CONVERT OPERAND AT CALL"),
            )),
            0x0E => Inst::Ret,
            0x0F => Inst::RetV,
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
            Self::DupT(operand) => {
                if miku.stack.is_empty() {
                    panic!("STACK UNDERFLOW");
                }

                if *operand >= miku.stack_top {
                    panic!("DUP INDEX OUT OF BOUNDS");
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
                miku.stack.push(MikuType::add(a, b));
                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Minus => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();
                miku.stack.push(MikuType::subtract(a, b));
                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Mult => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();
                miku.stack.push(MikuType::multiply(a, b));
                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Div => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();
                miku.stack.push(MikuType::divide(a, b));
                miku.stack_top -= 1;
                miku.ins_ptr += 1;
            }
            Self::Eq => {
                if miku.stack_top < 2 {
                    panic!("STACK UNDERFLOW");
                }

                let a = miku.stack.pop().unwrap();
                let b = miku.stack.pop().unwrap();

                if MikuType::eq(a, b) {
                    miku.stack.push(MikuType::U8(0));
                } else {
                    miku.stack.push(MikuType::U8(1));
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
            Self::JmpZ(operand) => {
                if *operand >= miku.program.len() {
                    panic!("JUMP OUT OF BOUNDS: {}", operand);
                }

                let top = miku.stack.pop().unwrap();
                miku.stack_top -= 1;

                if MikuType::eq(top, MikuType::U8(0)) {
                    miku.ins_ptr = *operand;
                } else {
                    miku.ins_ptr += 1;
                }
            }
            Self::JmpNZ(operand) => {
                if *operand >= miku.program.len() {
                    panic!("JUMP OUT OF BOUNDS: {}", operand);
                }

                let top = miku.stack.pop().unwrap();
                miku.stack_top -= 1;

                if !MikuType::eq(top, MikuType::U8(0)) {
                    miku.ins_ptr = *operand;
                } else {
                    miku.ins_ptr += 1;
                }
            }
            Self::DupB(operand) => {
                if miku.stack.is_empty() {
                    panic!("STACK UNDERFLOW");
                }

                let index = miku.stack_base + *operand;

                if index >= miku.stack_top {
                    panic!("DUPB INDEX OUT OF BOUNDS");
                }

                if miku.stack.len() == miku.stack_top {
                    miku.stack.push(miku.stack[index]);
                } else {
                    miku.stack[miku.stack_top] = miku.stack[index];
                }

                miku.stack_top += 1;
                miku.ins_ptr += 1;
            }
            Self::Call(operand) => {
                if *operand >= miku.program.len() {
                    panic!("UNDEFINED CALL");
                }

                // Push the return address (ins_ptr + 1) onto the stack:
                let return_address = MikuType::U64((miku.ins_ptr + 1) as u64);

                if miku.stack_top == miku.stack.len() {
                    miku.stack.push(return_address);
                } else {
                    miku.stack[miku.stack_top] = return_address;
                }

                miku.stack_top += 1;

                // Push current base ptr onto the stack:
                let base_ptr = MikuType::U64(miku.stack_base as u64);

                if miku.stack_top == miku.stack.len() {
                    miku.stack.push(base_ptr);
                } else {
                    miku.stack[miku.stack_top] = base_ptr;
                }

                miku.stack_top += 1;

                // Changing the base to the current top:
                miku.stack_base = miku.stack_top;

                // Jumping to the first instruction of the function:
                miku.ins_ptr = *operand;
            }
            Self::Ret => {
                // Resetting the base pointer:
                match miku.stack[miku.stack_top - 1] {
                    MikuType::U64(val) => miku.stack_base = val as usize,
                    _ => panic!("EXPECTED U64 AS RETURN STACK BASE"),
                }
                miku.stack_top -= 1;

                // Jumping back to return address:
                match miku.stack[miku.stack_top - 1] {
                    MikuType::U64(addr) => miku.ins_ptr = addr as usize,
                    _ => panic!("EXPECTED U64 AS RETURN ADDRESS"),
                }
                miku.stack_top -= 1;
            } 
            Self::RetV => {
                // If there are no values on the stack frame:
                if miku.stack_top == miku.stack_base {
                    panic!("NO RETURN VALUE SPECIFIED");
                }

                // Saving the return value:
                let return_value = miku.stack[miku.stack_top - 1];

                // Clear the stack frame:
                miku.stack_top -= miku.stack_top - miku.stack_base;

                // Resetting the base pointer:
                match miku.stack[miku.stack_top - 1] {
                    MikuType::U64(val) => miku.stack_base = val as usize,
                    _ => panic!("EXPECTED U64 AS RETURN STACK BASE"),
                }
                miku.stack_top -= 1;

                // Jumping back to return address:
                match miku.stack[miku.stack_top - 1] {
                    MikuType::U64(addr) => miku.ins_ptr = addr as usize,
                    _ => panic!("EXPECTED U64 AS RETURN ADDRESS"),
                }
                miku.stack_top -= 1;

                // Pushing the return value onto the stack: 
                if miku.stack_top == miku.stack.len() {
                    miku.stack.push(return_value);
                } else {
                    miku.stack[miku.stack_top] = return_value;
                }

                miku.stack_top += 1;
            }
        }
    }
}
