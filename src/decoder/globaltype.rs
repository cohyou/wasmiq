use std::io::Read;
use crate::{
    GlobalType,
    Mut,
};
use super::{
    decode_valtype,
};

pub(super) fn decode_globaltype(reader: &mut impl Read) -> GlobalType {
    let valtype = decode_valtype(reader);
    let mutablilty = decode_mut(reader);
    GlobalType(valtype, mutablilty)
}

fn decode_mut(reader: &mut impl Read) -> Mut {
    if let Some(Ok(b)) = reader.bytes().next() {
        match b {
            0x00 => Mut::Const,
            0x01 => Mut::Var,
            _ => panic!("invalid on decode_mut"),
        }
    } else {
        unimplemented!()
    }   
}
