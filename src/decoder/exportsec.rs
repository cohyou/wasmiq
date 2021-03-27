use std::io::Read;

use crate::{
    Export,
    ExportDesc,
};
use super::{
    decode_u32_from_leb128, decode_vec,
};
use super::idx::{decode_funcidx, decode_tableidx, decode_memidx, decode_globalidx};
use super::name::{decode_name};


pub(super) fn decode_exportsec(reader: &mut impl Read) -> Vec<Export> {
    // prefixはsection number 7
    let length = decode_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    decode_vec(&mut handle, decode_export)
}

fn decode_export(reader: &mut impl Read) -> Export {
    let name_identifier = decode_name(reader);
    let exportdesc = decode_exportdesc(reader);
    Export {
        name: name_identifier,
        desc: exportdesc,
    }
}

fn decode_exportdesc(reader: &mut impl Read) -> ExportDesc {
    if let Some(Ok(byte)) = reader.bytes().next() {
        match byte {
            0x00 => { return decode_exportdesc_func(reader) },
            0x01 => { return decode_exportdesc_tabletype(reader) },
            0x02 => { return decode_exportdesc_memtype(reader) },
            0x03 => { return decode_exportdesc_globaltype(reader) },
            _ => panic!("invalid on decode_exportdesc"),
        }
    }

    unimplemented!()
}

fn decode_exportdesc_func(reader: &mut impl Read) -> ExportDesc {
    ExportDesc::Func(decode_funcidx(reader))
}

fn decode_exportdesc_tabletype(reader: &mut impl Read) -> ExportDesc {
    ExportDesc::Table(decode_tableidx(reader))
}

fn decode_exportdesc_memtype(reader: &mut impl Read) -> ExportDesc {
    ExportDesc::Mem(decode_memidx(reader))
}

fn decode_exportdesc_globaltype(reader: &mut impl Read) -> ExportDesc {
    ExportDesc::Global(decode_globalidx(reader))
}

