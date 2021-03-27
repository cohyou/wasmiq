use std::io::Read;

use crate::{
    Mem,
};
use super::{decode_u32_from_leb128, decode_vec};
use super::memtype::{decode_memtype};

pub(super) fn decode_memsec(reader: &mut impl Read) -> Vec<Mem> {
    // prefixはsection number 5
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_mem)
}

fn decode_mem(reader: &mut impl Read) -> Mem {
    Mem(decode_memtype(reader))
}