use std::io::Read;

use crate::{
    TypeIdx,
    FuncIdx,
    TableIdx,
    GlobalIdx,
    LabelIdx,
    LocalIdx,

};
use super::{
    decode_u32_from_leb128, 
    decode_vec,
};

pub(super) fn decode_typeidx(reader: &mut impl Read) -> TypeIdx {
    decode_u32_from_leb128(reader)
}

pub(super) fn decode_funcidx(reader: &mut impl Read) -> FuncIdx {
    decode_u32_from_leb128(reader)
}

pub(super) fn decode_tableidx(reader: &mut impl Read) -> TableIdx {
    decode_u32_from_leb128(reader)
}

pub(super) fn decode_memidx(reader: &mut impl Read) -> TypeIdx {
    decode_u32_from_leb128(reader)
}

pub(super) fn decode_globalidx(reader: &mut impl Read) -> GlobalIdx {
    decode_u32_from_leb128(reader)
}

pub(super) fn decode_labelidx(reader: &mut impl Read) -> LabelIdx {
    decode_u32_from_leb128(reader)
}

pub(super) fn decode_localidx(reader: &mut impl Read) -> LocalIdx {
    decode_u32_from_leb128(reader)
}

pub(super) fn decode_funcindices(reader: &mut impl Read) -> Vec<FuncIdx> {
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_funcidx)
}

pub(super) fn decode_labelindices(reader: &mut impl Read) -> Vec<LabelIdx> {
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_labelidx)
}