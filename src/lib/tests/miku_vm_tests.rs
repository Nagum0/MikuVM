use crate::{miku::MikuVM, types::MikuType};

#[test]
fn define_data_test() {
    let mut vm = MikuVM::new();
    let _ = vm.define_data(MikuType::U8(69), 0);
    let _ = vm.define_data(MikuType::F64(420.69), 1);
    let x = vm.define_data(MikuType::U64(3476539), 0);
    assert!(x.is_err());
    let y = vm.define_data(MikuType::U64(3476539), 600);
    assert!(y.is_err());
    let _ = vm.define_data(MikuType::F64(420.69), 10);
}
