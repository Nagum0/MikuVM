# Miku VM

<img src="logo.jpeg" width=250>

- A very simple and minimal virtual machine built in rust for studying purposes.
- I'm just fucking around and finding out!

### Stack

- Types:
  - U8
  - U16
  - U32
  - U64

### Instrucions

| number | name  | arg1  | arg2  | arg3 | arg4 | description                                                                                                                   |
| ------ | ----- | ----- | ----- | ---- | ---- | ----------------------------------------------------------------------------------------------------------------------------- |
| 0      | push  | type  | value | -    | -    | Pushes a value onto the top of the stack with the given type                                                                  |
| 1      | pop   | -     | -     | -    | -    | Pops the top value off the stack (That value is lost)                                                                         |
| 2      | dup   | value | -     | -    | -    | Duplicates the value at the given relative index and pushes it onto the top of the stack                                      |
| 3      | swap  | -     | -     | -    | -    | Swaps the top 2 values on the stack                                                                                           |
| 4      | plus  | -     | -     | -    | -    | Pops the top 2 values off the stack, adds them together and pushes the result back on the stack                               |
| 5      | minus | -     | -     | -    | -    | Pops the top 2 values off the stack, subtracts the second popped value from the first and pushes the result back on the stack |
| 6      | mult  | -     | -     | -    | -    | Pops the top 2 values off the stack, multiplies them together and pushes the result back on the stack                         |
| 7      | div   | -     | -     | -    | -    | Pops the top 2 values off the stack, divides the first popped with the second and pushes the result back on the stack         |
| 8      | eq    | -     | -     | -    | -    | Pops the top 2 values off the stack, checks whether they're equal, if they are it pushes U8(0) onto the stack else U8(1)      |
| 9      | jmp   | value | -     | -    | -    | Jumps to the given address (label or hardcoded address)                                                                       |
