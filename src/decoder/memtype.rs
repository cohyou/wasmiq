use std::io::Read;
use crate::{
    MemType,
};

use super::limits::{
    decode_limits
};


pub(super) fn decode_memtype(reader: &mut impl Read) -> MemType {
    MemType(decode_limits(reader))
}