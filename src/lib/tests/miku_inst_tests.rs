use crate::{inst::*, miku::MikuVM, types::MikuType, DATA_START};

#[test]
fn push_test() {
    // Functionality test
    let mut vm = MikuVM::new();
    let i1: Box<dyn Inst> = Box::new(Push::new(MikuType::U8(69)));
    let i2: Box<dyn Inst> = Box::new(Push::new(MikuType::I64(-728463721)));
    let i3: Box<dyn Inst> = Box::new(Push::new(MikuType::F32(8947.2932)));
    vm.push_inst(&i1);
    vm.push_inst(&i2);
    vm.push_inst(&i3);
    let _ = vm.run_program();
    assert_eq!(
        vec![MikuType::U8(69), MikuType::I64(-728463721), MikuType::F32(8947.2932)], 
        vm.stack()[0..3].to_vec()
    );
    assert_eq!(3, vm.pc());
    assert_eq!(3, vm.stack_top());
    assert_eq!(0, vm.stack_base());

    // Encoding test
    let encoded_i1 = i1.encode();
    assert_eq!(vec![0x00, 0x00, 0x45], encoded_i1);
    let encoded_i2 = i2.encode();
    assert_eq!(vec![0x00, 0x07, 0x97, 0x86, 0x94, 0xD4, 0xFF, 0xFF, 0xFF, 0xFF], encoded_i2);

    // Decoding test
    assert_eq!(
        Push::new(MikuType::U8(69)),
        Push::decode(&vec![0x00, 0x00, 0x45]).unwrap()
    );
    assert_eq!(
        Push::new(MikuType::I64(-728463721)),
        Push::decode(&vec![0x00, 0x07, 0x97, 0x86, 0x94, 0xD4, 0xFF, 0xFF, 0xFF, 0xFF]).unwrap()
    );
}

#[test]
fn pop_test() {
    // Functionality test 1.
    let mut vm = MikuVM::new();
    let i1: Box<dyn Inst> = Box::new(Push::new(MikuType::U8(69)));
    let i2: Box<dyn Inst> = Box::new(Push::new(MikuType::I64(-728463721)));
    let i3: Box<dyn Inst> = Box::new(Push::new(MikuType::F32(8947.2932)));
    vm.push_inst(&i1);
    vm.push_inst(&i2);
    vm.push_inst(&i3);
    let i4: Box<dyn Inst> = Box::new(Pop::new());
    vm.push_inst(&i4);
    let _ = vm.run_program();
    assert_eq!(
        vec![MikuType::U8(69), MikuType::I64(-728463721), MikuType::F32(8947.2932)],
        vm.stack()[0..3].to_vec()
    );
    assert_eq!(4, vm.pc());
    assert_eq!(2, vm.stack_top());
    assert_eq!(0, vm.stack_base());

    // Functionality test 2.
    let mut vm = MikuVM::new();
    let i1: Box<dyn Inst> = Box::new(Push::new(MikuType::U8(69)));
    let i2: Box<dyn Inst> = Box::new(Push::new(MikuType::I64(-728463721)));
    let i3: Box<dyn Inst> = Box::new(Push::new(MikuType::F32(8947.2932)));
    vm.push_inst(&i1);
    vm.push_inst(&i2);
    vm.push_inst(&i3);
    let i4: Box<dyn Inst> = Box::new(Pop::new());
    let i5: Box<dyn Inst> = Box::new(Push::new(MikuType::I16(420)));
    vm.push_inst(&i4);
    vm.push_inst(&i5);
    let _ = vm.run_program();
    assert_eq!(
        vec![MikuType::U8(69), MikuType::I64(-728463721), MikuType::I16(420)],
        vm.stack()[0..3].to_vec()
    );
    assert_eq!(5, vm.pc());
    assert_eq!(3, vm.stack_top());
    assert_eq!(0, vm.stack_base());
    
    // Stack underflow test
    let mut vm = MikuVM::new();
    let i1: Box<dyn Inst> = Box::new(Pop::new());
    vm.push_inst(&i1);
    let status = vm.run_program();
    assert!(status.is_err());

    // Encoding test
    assert_eq!(vec![0x01], i4.encode());

    // Decoding test
    assert_eq!(Pop::new(), Pop::decode(&vec![0x01]).unwrap());
}

#[test]
fn def_test() {
    // Functionality test
    let mut vm = MikuVM::new();
    let i1: Box<dyn Inst> = Box::new(Def::new(MikuType::U32(420), DATA_START));
    let i2: Box<dyn Inst> = Box::new(Def::new(MikuType::U8(69), DATA_START + 1));
    vm.push_inst(&i1);
    vm.push_inst(&i2);
    let status = vm.run_program();
    assert!(status.is_ok());
    assert_eq!(
        vec![MikuType::U32(420), MikuType::U8(69)],
        vm.data_mem()[0..2].to_vec()
    );

    // Encoding test
    assert_eq!(
        vec![0x02, 0x00, 0x45, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        Def::new(MikuType::U8(69), 1).encode()
    );

    // Decoding test
    assert_eq!(
        Def::new(MikuType::U8(69), 1),
        Def::decode(&vec![0x02, 0x00, 0x45, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap()
    );
}

#[test]
fn set_test() {
    // Functionality test
    let mut vm = MikuVM::new();
    let i1: Box<dyn Inst> = Box::new(Set::new(0, either::Either::Left(MikuType::U8(69)), false));
    vm.push_inst(&i1);
    let _ = vm.run_program();
    assert_eq!(
        MikuType::U8(69),
        vm.read_register(0).unwrap()
    );
}
