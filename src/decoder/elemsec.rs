use std::io::Read;

use super::{decode_u32_from_leb128, decode_vec};
use crate::{
    Elem,
};
use super::idx::{decode_tableidx, decode_funcindices};
use super::expr::{decode_expr};

pub(super) fn decode_elemsec(reader: &mut impl Read) -> Vec<Elem> {
    // prefixはsection number 9
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_elem)
}

fn decode_elem(reader: &mut impl Read) -> Elem {
    let tableidx = decode_tableidx(reader);
    let expr = decode_expr(reader);
    let init = decode_funcindices(reader);
    Elem {
        table: tableidx,
        offset: expr,
        init: init,
    }
}