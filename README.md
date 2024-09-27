# Miku VM

<img src="logo.jpeg" width=250>

- A very simple and minimal virtual machine built in rust for studying purposes.
- I'm just fucking around and finding out!

### Usage

```command
$ masm fib.masm fib.mm
$ miku fib.mm
```

### Stack

- Types:
  - U8
  - U16
  - U32
  - U64

### Instrucions

| number | name  | arg1  | arg2  | arg3 | arg4 | description                                                                                                                                           |
| ------ | ----- | ----- | ----- | ---- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0      | push  | type  | value | -    | -    | Pushes a value onto the top of the stack with the given type                                                                                          |
| 1      | pop   | -     | -     | -    | -    | Pops the top value off the stack (That value is lost)                                                                                                 |
| 2      | dupt  | value | -     | -    | -    | Duplicates the value at the given relative index from the top and pushes it onto the top of the stack                                                 |
| 3      | swap  | -     | -     | -    | -    | Swaps the top 2 values on the stack                                                                                                                   |
| 4      | plus  | -     | -     | -    | -    | Pops the top 2 values off the stack, adds them together and pushes the result back on the stack                                                       |
| 5      | minus | -     | -     | -    | -    | Pops the top 2 values off the stack, subtracts the second popped value from the first and pushes the result back on the stack                         |
| 6      | mult  | -     | -     | -    | -    | Pops the top 2 values off the stack, multiplies them together and pushes the result back on the stack                                                 |
| 7      | div   | -     | -     | -    | -    | Pops the top 2 values off the stack, divides the first popped with the second and pushes the result back on the stack                                 |
| 8      | eq    | -     | -     | -    | -    | Pops the top 2 values off the stack, checks whether they're equal, if they are it pushes U8(0) onto the stack else U8(1)                              |
| 9      | jmp   | value | -     | -    | -    | Jumps to the given address (label or hardcoded address)                                                                                               |
| 10     | jmpz  | value | -     | -    | -    | Jumps to the given address (label or hardcoded address) if the top value on the stack is a 0 byte (U8). Pops the top of the stack                     |
| 11     | jmpnz | value | -     | -    | -    | Jumps to the given address (label or hardcoded address) if the top value on the stack is not a 0 byte (U8). Pops the top of the stack                 |
| 12     | dupb  | value | -     | -    | -    | Duplicates the value at the given relative index from the base and pushes it onto the top of the stack                                                |
| 13     | call  | value | -     | -    | -    | Pushes the next instuctions address and the old stack base pointers value onto the stack and jumps to the specified functions address                 |
| 14     | ret   | -     | -     | -    | -    | Resets the old stack base pointer and jumps back to the return address                                                                                |
| 15     | retv  | -     | -     | -    | -    | Save the top of the stack as the return value. Clear the stack frame (top -= top - base). Return as before and push saved return value onto the stack |

### Examples

- Program that counts up to 5

```asm
push u8 0
dup 0
push u8 5
eq
jmpz 8
push u8 1
plus
jmp 1
push u8 69
```

- Program that computes the first 10 fibonacci numbers

```asm
push u8 0
push u8 1
push u8 1
dup 2
dup 2
plus
dup 1
push u8 1
plus
dup 0
push u8 10
eq
jmpz 17
dup 3
dup 2
dup 2
jmp 3
dup 1
```

### TODOs

- [ ] Tidying up the code
- [ ] Error handling