use std::io::Read;

use crate::{
    TableType,
};
use super::limits::{decode_limits};
use super::elemtype::{decode_elemtype};


pub(super) fn decode_tabletype(reader: &mut impl Read) -> TableType {
    let limits = decode_limits(reader);
    let elem_type = decode_elemtype(reader);
    TableType(limits, elem_type)
}

