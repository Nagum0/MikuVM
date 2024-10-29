use crate::{miku::MikuVM, types::MikuType, inst::*};

#[test]
fn push_test() {
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
}
