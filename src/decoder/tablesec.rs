use std::io::Read;

use crate::{
    Table
};
use super::{decode_u32_from_leb128, decode_vec};
use super::tabletype::{decode_tabletype};


pub(super) fn decode_tablesec(reader: &mut impl Read) -> Vec<Table> {
    // prefixはsection number 4
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_table)
}

fn decode_table(reader: &mut impl Read) -> Table {
    let tabletype = decode_tabletype(reader);
    Table(tabletype)
}