use crate::{miku::MikuVM, types::MikuType, inst::*};

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
    assert_eq!(vec![MikuType::U8(69), MikuType::I64(-728463721), MikuType::F32(8947.2932)], vm.stack());
    assert_eq!(3, vm.pc());
    assert_eq!(3, vm.stack_top());
    assert_eq!(0, vm.stack_base());

    // Encoding test
    let encoded_i1 = i1.encode();
    assert_eq!(vec![0x03, 0x00, 0x00, 0x45], encoded_i1);
    let encoded_i2 = i2.encode();
    assert_eq!(vec![0x0A, 0x00, 0x07, 0x97, 0x86, 0x94, 0xD4, 0xFF, 0xFF, 0xFF, 0xFF], encoded_i2);

    // Decoding test
    assert_eq!(Push::new(MikuType::U8(69)), Push::decode(&vec![0x03, 0x00, 0x00, 0x45]).unwrap());
    assert_eq!(
        Push::new(MikuType::I64(-728463721)),
        Push::decode(&vec![0x0A, 0x00, 0x07, 0x97, 0x86, 0x94, 0xD4, 0xFF, 0xFF, 0xFF, 0xFF]).unwrap()
    );
}
