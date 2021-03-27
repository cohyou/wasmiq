use std::io::Read;

use crate::{
    Expr,
    Func,
};
use super::{decode_u32_from_leb128, decode_vec};

use super::idx::{decode_typeidx};

use super::codesec::Code;


impl Func {
    pub(super) fn set_code(&mut self, code: &Code) {
        self.locals = code.locals();
        self.body = code.body();
    }
}

pub(super) fn decode_funcsec(reader: &mut impl Read) -> Vec<Func> {
    // prefixはsection number 3
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, |reader| {
        Func {
            tp: decode_typeidx(reader),
            locals: vec![],
            body: Expr::default(),
        }
    })
}
