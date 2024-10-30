use crate::{miku::MikuVM, types::MikuType, DATA_END, DATA_START};

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
