use std::io::Read;

use crate::{
    Global,
};
use super::{decode_u32_from_leb128, decode_vec};
use super::globaltype::{decode_globaltype};
use super::expr::{decode_expr};


pub(super) fn decode_globalsec(reader: &mut impl Read) -> Vec<Global> {
    // prefixはsection number 6
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_global)
}

fn decode_global(reader: &mut impl Read) -> Global {
    let globaltype = decode_globaltype(reader);
    let expr = decode_expr(reader);
    Global { tp: globaltype, init: expr }
}