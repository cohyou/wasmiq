use std::io::Read;
use crate::{
    ElemType,
};

pub fn decode_elemtype(reader: &mut impl Read) -> ElemType {
    if let Some(Ok(byte)) = reader.bytes().next() {
        if byte == 0x70 {
            ElemType::FuncRef
        } else {
            panic!("invalid on decode_elemtype");
        }
    } else {
        panic!("invalid on decode_elemtype");
    }   
}
