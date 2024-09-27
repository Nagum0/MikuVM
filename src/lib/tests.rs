use crate::types::MikuType;
use crate::miku::Miku;
use crate::inst::Inst;

#[test]
fn push_test() {
    let mut miku = Miku::new();
    miku.program = vec![
        Inst::Push(MikuType::U8(255)),
        Inst::Push(MikuType::U16(65535)),
        Inst::Push(MikuType::U32(4294967295)),
        Inst::Push(MikuType::U64(18446744073709551615)),
    ];
    miku.run_program();
    assert_eq!(vec![
        MikuType::U8(255), 
        MikuType::U16(65535), 
        MikuType::U32(4294967295), 
        MikuType::U64(18446744073709551615)],
        miku.stack
    );
}

#[test]
fn push_test_2() {
    let mut miku = Miku::new();
    miku.program = vec![
        Inst::Push(MikuType::U8(255)),
        Inst::Push(MikuType::U16(65535)),
        Inst::Push(MikuType::U32(4294967295)),
        Inst::Push(MikuType::U64(18446744073709551615)),
        Inst::Pop,
        Inst::Push(MikuType::U64(69))
    ];
    miku.run_program();
    assert_eq!(vec![
        MikuType::U8(255), 
        MikuType::U16(65535), 
        MikuType::U32(4294967295), 
        MikuType::U64(69)],
        miku.stack
    );
}

#[test]
fn pop_test() {
    let mut miku = Miku::new();
    miku.program = vec![
        Inst::Push(MikuType::U8(255)),
        Inst::Push(MikuType::U16(65535)),
        Inst::Push(MikuType::U32(4294967295)),
        Inst::Push(MikuType::U64(18446744073709551615)),
        Inst::Pop,
        Inst::Pop,
    ];
    miku.run_program();
    assert_eq!(vec![MikuType::U8(255), MikuType::U16(65535)], &miku.stack[miku.stack_base..miku.stack_top]);
}

#[test]
fn retv_test() {
    let mut miku = Miku::new();
    miku.program = vec![
        Inst::Jmp(4),
        Inst::Push(MikuType::U8(69)),
        Inst::Push(MikuType::U16(420)),
        Inst::RetV,
        Inst::Call(1)
    ];
    miku.run_program();
    assert_eq!(
        vec![MikuType::U16(420)],
        &miku.stack[miku.stack_base..miku.stack_top]
    );
}