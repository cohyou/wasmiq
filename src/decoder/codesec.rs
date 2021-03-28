use std::io::Read;

use crate::{
    ValType,
    Expr,
};
use super::{decode_u32_from_leb128, decode_vec};
use super::{decode_valtype};
use super::expr::{decode_expr};


pub struct Code {
    #[allow(dead_code)]
    size: u32,
    locals: Vec<Locals>,
    body: Expr,
}

// #[derive(Copy, Clone)]
struct Locals(u32, ValType);


impl Code {
    pub(super) fn locals(&self) -> Vec<ValType> {
        let mut res = vec![];
        for locals in &self.locals {
            res.extend(std::iter::repeat(locals.1).take(locals.0 as usize));
        }
        res
    }
    pub(super) fn body(&self) -> Expr { self.body.clone() }
}

pub(super) fn decode_codesec(reader: &mut impl Read) -> Vec<Code> {
    // prefixはsection number 10
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_code)
}

fn decode_code(reader: &mut impl Read) -> Code {
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);

    let length = decode_u32_from_leb128(&mut handle);
    let mut handle = reader.take(length as u64);
    let locals_vec = decode_vec(&mut handle, decode_locals);
    let expr = decode_expr(reader);

    Code { size: length, locals: locals_vec, body: expr }
}

fn decode_locals(reader: &mut impl Read) -> Locals {
    let n = decode_u32_from_leb128(reader);
    let valtype = decode_valtype(reader);
    Locals(n, valtype)
}