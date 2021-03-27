use std::io::Read;

use crate::{
    FuncType,

};
use super::{
    decode_functype,
    decode_u32_from_leb128, 
    decode_vec,
};

pub(super) fn decode_typesec(reader: &mut impl Read) -> Vec<FuncType> {
    // prefixはsection number 1
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_functype)
}
