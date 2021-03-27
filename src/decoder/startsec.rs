use std::io::Read;

use crate::{
    Start,
};
use super::idx::{decode_funcidx};


pub(super) fn decode_startsec(reader: &mut impl Read) -> Start {
    // prefixはsection number 8
    decode_start(reader)
}

fn decode_start(reader: &mut impl Read) -> Start {
    Start(decode_funcidx(reader))
}