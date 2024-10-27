# Miku VM

<img src="logo.jpeg" width=250>

- A very simple and minimal virtual machine built in rust for fun.
- Nothing serious and professional I'm just fucking around and finding out!

### Usage
```command
$ masm fib.masm fib.mm
$ miku fib.mm
```

### Primitive types (MikuType)
- Types:
  - U8
  - U16
  - U32
  - U64 (this also represents a pointer)
  - I8
  - I16
  - I32
  - I64

### Registers
- A1
- A2
- A3
- A4
- A5
- RET (Used for function return values)

### Stack
- Grows in size
- Each entry is a MikuType
