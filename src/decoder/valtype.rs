use std::io::Read;
use crate::{
    ValType,
    Byte,
};

pub(super) fn byte_to_valtype(b :Byte) -> ValType {
    match b {
        0x7F => ValType::I32,
        0x7E => ValType::I64,
        0x7D => ValType::F32,
        0x7C => ValType::F64,
        _ => panic!("invalid on byte_to_valtype: {:x?}", b),
    }
}

pub(super) fn decode_valtype(reader: &mut impl Read) -> ValType {
    let byte = reader.bytes().next();
    if let Some(Ok(b)) = byte {
        byte_to_valtype(b)
    } else {
        panic!("invalid on read_valtype");
    }
}
