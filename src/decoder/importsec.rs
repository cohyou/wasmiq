use std::io::Read;

use crate::{
    Import,
    ImportDesc,

};

use super::{
    decode_u32_from_leb128, 
    decode_vec
};
use super::memtype::{decode_memtype};
use super::globaltype::{decode_globaltype};
use super::tabletype::decode_tabletype;
use super::idx::decode_typeidx;
use super::name::{decode_name};


pub(super) fn decode_importsec(reader: &mut impl Read) -> Vec<Import> {
    // prefixはsection number 2
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_import)
}

fn decode_import(reader: &mut impl Read) -> Import {
    let module_identifier = decode_name(reader);
    let name_identifier = decode_name(reader);
    let importdesc = decode_importdesc(reader);
    Import {
        module: module_identifier,
        name: name_identifier,
        desc: importdesc,
    }
}

fn decode_importdesc(reader: &mut impl Read) -> ImportDesc {
    if let Some(Ok(byte)) = reader.bytes().next() {
        match byte {
            0x00 => { return decode_importdesc_func(reader) },
            0x01 => { return decode_importdesc_tabletype(reader) },
            0x02 => { return decode_importdesc_memtype(reader) },
            0x03 => { return decode_importdesc_globaltype(reader) },
            _ => panic!("invalid on decode_importdesc"),
        }
    }

    unimplemented!()
}

fn decode_importdesc_func(reader: &mut impl Read) -> ImportDesc {
    ImportDesc::Func(decode_typeidx(reader))
}

fn decode_importdesc_tabletype(reader: &mut impl Read) -> ImportDesc {
    ImportDesc::Table(decode_tabletype(reader))
}

fn decode_importdesc_memtype(reader: &mut impl Read) -> ImportDesc {
    ImportDesc::Mem(decode_memtype(reader))
}

fn decode_importdesc_globaltype(reader: &mut impl Read) -> ImportDesc {
    ImportDesc::Global(decode_globaltype(reader))
}

