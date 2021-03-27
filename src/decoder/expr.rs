use std::io::Read;
use crate::{
    Byte,
    Expr,
    Instr,
};
use super::instr::{decode_instr};


pub(super) fn decode_expr(reader: &mut impl Read) -> Expr {
    Expr(decode_instrs(reader))
}

pub(super) fn decode_instrs(reader: &mut impl Read) -> Vec<Instr> {
    decode_instrs_internal(reader, 0x05)
}
// pub(super) fn decode_instrs_else(reader: &mut impl Read) -> Vec<Instr> {
//     decode_instrs_internal(reader, 0x05)
// }

fn decode_instrs_internal(reader: &mut impl Read, stop: Byte) -> Vec<Instr> {
    let mut instrs = vec![];

    loop {
        if let Some(Ok(b)) = reader.bytes().next() {
            if b == stop { break; }  // end

            let instr = decode_instr(b, reader);
            instrs.push(instr);
        } else {
            break;
        }
    }

    instrs
}