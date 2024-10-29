use super::super::types::MikuType;

#[test]
fn addition_test() {
    assert_eq!(MikuType::U8(69), (MikuType::U8(34) + MikuType::U8(35)).unwrap());
    assert_eq!(MikuType::U16(200), (MikuType::U16(100) + MikuType::U16(100)).unwrap());
    assert_eq!(MikuType::U32(3000), (MikuType::U32(1500) + MikuType::U32(1500)).unwrap());
    assert_eq!(MikuType::U64(70000), (MikuType::U64(35000) + MikuType::U64(35000)).unwrap());

    assert_eq!(MikuType::I8(-10), (MikuType::I8(-15) + MikuType::I8(5)).unwrap());
    assert_eq!(MikuType::I16(0), (MikuType::I16(100) + MikuType::I16(-100)).unwrap());
    assert_eq!(MikuType::I32(500), (MikuType::I32(250) + MikuType::I32(250)).unwrap());
    assert_eq!(MikuType::I64(-500), (MikuType::I64(-300) + MikuType::I64(-200)).unwrap());

    assert_eq!(MikuType::F32(10.5), (MikuType::F32(5.5) + MikuType::F32(5.0)).unwrap());
    assert_eq!(MikuType::F64(25.0), (MikuType::F64(15.0) + MikuType::F64(10.0)).unwrap());

    // Error case: mismatched types
    assert!((MikuType::U8(10) + MikuType::I32(10)).is_err());
}

#[test]
fn subtraction_test() {
    assert_eq!(MikuType::U8(4), (MikuType::U8(9) - MikuType::U8(5)).unwrap());
    assert_eq!(MikuType::U16(50), (MikuType::U16(100) - MikuType::U16(50)).unwrap());
    assert_eq!(MikuType::U32(1000), (MikuType::U32(1500) - MikuType::U32(500)).unwrap());
    assert_eq!(MikuType::U64(20000), (MikuType::U64(30000) - MikuType::U64(10000)).unwrap());

    assert_eq!(MikuType::I8(0), (MikuType::I8(-5) - MikuType::I8(-5)).unwrap());
    assert_eq!(MikuType::I16(-100), (MikuType::I16(0) - MikuType::I16(100)).unwrap());
    assert_eq!(MikuType::I32(-250), (MikuType::I32(-500) - MikuType::I32(-250)).unwrap());
    assert_eq!(MikuType::I64(100), (MikuType::I64(200) - MikuType::I64(100)).unwrap());

    assert_eq!(MikuType::F32(1.0), (MikuType::F32(5.5) - MikuType::F32(4.5)).unwrap());
    assert_eq!(MikuType::F64(-5.0), (MikuType::F64(10.0) - MikuType::F64(15.0)).unwrap());

    // Error case: mismatched types
    assert!((MikuType::U16(100) - MikuType::F32(5.5)).is_err());
}

#[test]
fn multiplication_test() {
    assert_eq!(MikuType::U8(25), (MikuType::U8(5) * MikuType::U8(5)).unwrap());
    assert_eq!(MikuType::U16(1000), (MikuType::U16(100) * MikuType::U16(10)).unwrap());
    assert_eq!(MikuType::U32(3000), (MikuType::U32(1000) * MikuType::U32(3)).unwrap());
    assert_eq!(MikuType::U64(50000), (MikuType::U64(500) * MikuType::U64(100)).unwrap());

    assert_eq!(MikuType::I8(-25), (MikuType::I8(-5) * MikuType::I8(5)).unwrap());
    assert_eq!(MikuType::I16(-200), (MikuType::I16(20) * MikuType::I16(-10)).unwrap());
    assert_eq!(MikuType::I32(300), (MikuType::I32(15) * MikuType::I32(20)).unwrap());
    assert_eq!(MikuType::I64(-1000), (MikuType::I64(-100) * MikuType::I64(10)).unwrap());

    assert_eq!(MikuType::F32(9.9), (MikuType::F32(3.3) * MikuType::F32(3.0)).unwrap());
    assert_eq!(MikuType::F64(20.0), (MikuType::F64(5.0) * MikuType::F64(4.0)).unwrap());

    // Error case: mismatched types
    assert!((MikuType::I8(-10) * MikuType::U64(100)).is_err());
}

#[test]
fn division_test() {
    assert_eq!(MikuType::U8(2), (MikuType::U8(10) / MikuType::U8(5)).unwrap());
    assert_eq!(MikuType::U16(4), (MikuType::U16(100) / MikuType::U16(25)).unwrap());
    assert_eq!(MikuType::U32(20), (MikuType::U32(400) / MikuType::U32(20)).unwrap());
    assert_eq!(MikuType::U64(500), (MikuType::U64(1000) / MikuType::U64(2)).unwrap());

    assert_eq!(MikuType::I8(-5), (MikuType::I8(-25) / MikuType::I8(5)).unwrap());
    assert_eq!(MikuType::I16(-10), (MikuType::I16(-100) / MikuType::I16(10)).unwrap());
    assert_eq!(MikuType::I32(3), (MikuType::I32(15) / MikuType::I32(5)).unwrap());
    assert_eq!(MikuType::I64(-2), (MikuType::I64(-200) / MikuType::I64(100)).unwrap());

    assert_eq!(MikuType::F32(5.0), (MikuType::F32(10.0) / MikuType::F32(2.0)).unwrap());
    assert_eq!(MikuType::F64(4.5), (MikuType::F64(9.0) / MikuType::F64(2.0)).unwrap());

    // Error case: division by zero
    let z = MikuType::I32(10) / MikuType::I32(0);
    assert_eq!(true, z.is_err());
}

#[test]
fn convert_to_bytes_test() {
    let a_u8 = MikuType::U8(69);
    let a_u8_vec = Vec::from(a_u8);
    assert_eq!(&vec![0x00, 0x45], &a_u8_vec);

    let a_u16 = MikuType::U16(420);
    let a_u16_vec = Vec::from(a_u16);
    assert_eq!(&vec![0x01, 0xA4, 0x01], &a_u16_vec);

    let a_u32 = MikuType::U32(526470706);
    let a_u32_vec = Vec::from(a_u32);
    assert_eq!(&vec![0x02, 0x32, 0x4E, 0x61, 0x1F], &a_u32_vec);

    let a_u64 = MikuType::U64(12031311536003071336);
    let a_u64_vec = Vec::from(a_u64);
    assert_eq!(&vec![0x03, 0x68, 0xBD, 0x85, 0xED, 0x18, 0xCE, 0xF7, 0xA6], &a_u64_vec);

    let a_i8 = MikuType::I8(-69);
    let a_i8_vec = Vec::from(a_i8);
    assert_eq!(&vec![0x04, 0xBB], &a_i8_vec);
    
    let a_i16 = MikuType::I16(-420);
    let a_i16_vec = Vec::from(a_i16);
    assert_eq!(&vec![0x05, 0x5C, 0xFE], &a_i16_vec);

    let a_i32 = MikuType::I32(526470706);
    let a_i32_vec = Vec::from(a_i32);
    assert_eq!(&vec![0x06, 0x32, 0x4E, 0x61, 0x1F], &a_i32_vec);

    let a_i64 = MikuType::I64(-3989038795165003400);
    let a_i64_vec = Vec::from(a_i64);
    assert_eq!(&vec![0x07, 0x78, 0x11, 0xF0, 0xC2, 0x59, 0x16, 0xA4, 0xC8], &a_i64_vec);

    let a_f32 = MikuType::F32(89093747.5830423);
    let a_f32_vec = Vec::from(a_f32);
    assert_eq!(&vec![0x08, 0xCE, 0xEE, 0xA9, 0x4C], &a_f32_vec);
    
    let a_f64 = MikuType::F64(-8909374798909789.590);
    let a_f64_vec = Vec::from(a_f64);
    assert_eq!(&vec![0x09, 0x5E, 0x51, 0x0F, 0x78, 0x07, 0xA7, 0x3F, 0xC3], &a_f64_vec);
}

#[test]
fn convert_from_bytes_test() {
    // U8
    let a_u8_vec = vec![0x00, 0x45];
    assert_eq!(MikuType::try_from(&a_u8_vec[..]).unwrap(), MikuType::U8(69));

    // U16
    let a_u16_vec = vec![0x01, 0xA4, 0x01];
    assert_eq!(MikuType::try_from(&a_u16_vec[..]).unwrap(), MikuType::U16(420));

    // U32
    let a_u32_vec = vec![0x02, 0x32, 0x4E, 0x61, 0x1F];
    assert_eq!(MikuType::try_from(&a_u32_vec[..]).unwrap(), MikuType::U32(526470706));

    // U64
    let a_u64_vec = vec![0x03, 0x68, 0xBD, 0x85, 0xED, 0x18, 0xCE, 0xF7, 0xA6];
    assert_eq!(MikuType::try_from(&a_u64_vec[..]).unwrap(), MikuType::U64(12031311536003071336));

    // I8
    let a_i8_vec = vec![0x04, 0xBB];
    assert_eq!(MikuType::try_from(&a_i8_vec[..]).unwrap(), MikuType::I8(-69));

    // I16
    let a_i16_vec = vec![0x05, 0x5C, 0xFE];
    assert_eq!(MikuType::try_from(&a_i16_vec[..]).unwrap(), MikuType::I16(-420));

    // I32
    let a_i32_vec = vec![0x06, 0x32, 0x4E, 0x61, 0x1F];
    assert_eq!(MikuType::try_from(&a_i32_vec[..]).unwrap(), MikuType::I32(526470706));

    // I64
    let a_i64_vec = vec![0x07, 0x78, 0x11, 0xF0, 0xC2, 0x59, 0x16, 0xA4, 0xC8];
    assert_eq!(MikuType::try_from(&a_i64_vec[..]).unwrap(), MikuType::I64(-3989038795165003400));

    // F32
    let a_f32_vec = vec![0x08, 0xCE, 0xEE, 0xA9, 0x4C];
    assert_eq!(MikuType::try_from(&a_f32_vec[..]).unwrap(), MikuType::F32(89093747.5830423));

    // F64
    let a_f64_vec = vec![0x09, 0x5E, 0x51, 0x0F, 0x78, 0x07, 0xA7, 0x3F, 0xC3];
    assert_eq!(MikuType::try_from(&a_f64_vec[..]).unwrap(), MikuType::F64(-8909374798909789.590));
}
