use std::io::Read;
use crate::FuncType;
use super::resulttype::decode_resulttype;


pub fn decode_functype(reader: &mut impl Read) -> FuncType {
    // 0x60がprefix
    let param_types = decode_resulttype(reader);
    let ret_types = decode_resulttype(reader);
    (param_types, ret_types)
}
