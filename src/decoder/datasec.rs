use std::io::Read;

use crate::{
    Data,
};
use super::{
    decode_u32_from_leb128, 
    decode_vec,
};
use super::idx::{decode_memidx};
use super::expr::{decode_expr};


pub(super) fn decode_datasec(reader: &mut impl Read) -> Vec<Data> {
    // prefixはsection number 11
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_data)
}

fn decode_data(reader: &mut impl Read) -> Data {
    let memidx = decode_memidx(reader);
    let expr = decode_expr(reader);
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    let init = decode_vec(&mut handle, |reader| reader.bytes().next().unwrap().unwrap());

    Data {
        data: memidx,
        offset: expr,
        init: init,
    }
}