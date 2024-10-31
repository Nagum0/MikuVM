use crate::{miku::MikuVM, types::MikuType, DATA_END, DATA_START, STACK_START};

#[test]
fn define_data_test() {
    let mut vm = MikuVM::new();
    let _ = vm.define_data(MikuType::U8(69), DATA_START);
    let _ = vm.define_data(MikuType::F64(420.69), DATA_START + 1);
    let x = vm.define_data(MikuType::U64(3476539), 0);
    assert!(x.is_err());
    let y = vm.define_data(MikuType::U64(3476539), DATA_END - 1);
    assert!(y.is_ok());
}

#[test]
fn deref_address_test() {
    let mut vm = MikuVM::new();
    let _ = vm.define_data(MikuType::U8(69), DATA_START);
    let read_data = vm.deref_ptr(MikuType::U64(DATA_START as u64)).unwrap();
    assert_eq!(MikuType::U8(69), read_data);

    let _ = vm.stack_push(MikuType::U64(0));
    let read_data = vm.deref_ptr(MikuType::U64(STACK_START as u64)).unwrap();
    assert_eq!(MikuType::U64(0), read_data);

    let read_data = vm.deref_ptr(MikuType::NULL);
    assert!(read_data.is_err());

    let read_data = vm.deref_ptr(MikuType::U32(0));
    assert!(read_data.is_err());

    let read_data = vm.deref_ptr(MikuType::U64(1024));
    assert!(read_data.is_err());
}

#[test]
fn registers_test() {
    let mut vm = MikuVM::new();
    let registers = vm.registers();
    assert_eq!(6, registers.len());

    let _ = vm.set_register(0, MikuType::I32(-100));
    let reg_val = vm.read_register(0).unwrap();
    assert_eq!(MikuType::I32(-100), reg_val);

    let reg_val = vm.read_register(5).unwrap();
    assert_eq!(MikuType::NULL, reg_val);

    let status = vm.set_register(6, MikuType::U8(69));
    assert!(status.is_err());

    let status = vm.read_register(7);
    assert!(status.is_err());
}
