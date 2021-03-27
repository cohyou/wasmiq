mod idx;
mod elemtype;
mod globaltype;
mod instr;

mod limits;
mod name;
mod expr;

mod memtype;
mod tabletype;

mod customsec;
mod typesec;
mod importsec;
mod funcsec;
mod tablesec;
mod memsec;
mod globalsec;
mod exportsec;
mod startsec;
mod elemsec;
mod codesec;
mod datasec;

mod valtype;
mod functype;
mod resulttype;
mod util;
use crate::{
    Module,
    Byte,
    // FuncType,
    // Import,
    // Func,
    // Table,
    // Mem,
    // Global,
    // Export,
    // Start,
    // Elem,
    // Data,
};

use std::io::{
    self,
    Read,
};

use customsec::decode_customsec;
use typesec::decode_typesec;
use importsec::{decode_importsec};
use funcsec::{decode_funcsec};
use tablesec::{decode_tablesec};
use memsec::{decode_memsec};
use globalsec::{decode_globalsec};
use exportsec::{decode_exportsec};
use startsec::{decode_startsec};
use elemsec::{decode_elemsec};
use codesec::{decode_codesec};
use datasec::{decode_datasec};

use valtype::{
    decode_valtype,
    byte_to_valtype,
};
use functype::decode_functype;
// use resulttype::decode_resulttype;

use util::*;

enum Section {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

pub fn decode_module(reader: &mut impl Read) -> io::Result<Module> {
    let mut module = Module::default();

    decode_magic(reader)?;
    decode_version(reader)?;
    while let Some(Ok(section_id)) = reader.bytes().next() {
        match id_to_section(section_id) {
            Section::Custom => decode_customsec(reader),
            Section::Type => { module.types = decode_typesec(reader); },
            Section::Import => { module.imports = decode_importsec(reader) },
            Section::Function => { module.funcs = decode_funcsec(reader) },
            Section::Table => { module.tables = decode_tablesec(reader) },
            Section::Memory => { module.mems = decode_memsec(reader) },
            Section::Global => { module.globals = decode_globalsec(reader) },
            Section::Export => { module.exports = decode_exportsec(reader) },
            Section::Start => { module.start = Some(decode_startsec(reader)) },
            Section::Element => { module.elem = decode_elemsec(reader) },
            Section::Code => {
                for (i, code) in decode_codesec(reader).iter().enumerate() {
                    module.funcs[i].set_code(code);
                }
            },
            Section::Data => { module.data = decode_datasec(reader) },
        }
    }

    Ok(module)
}

fn decode_magic(reader: &mut impl Read) -> io::Result<()> {
    let magic: [u8; 4] = [0x00, 0x61, 0x73, 0x6D,];
    let mut buf: [u8; 4] = [0x00; 4];
    reader.read_exact(&mut buf)?;
    if buf == magic {
        Ok(())
    } else {
        panic!("invalid on decode_magic");
    }
}

fn decode_version(reader: &mut impl Read) -> io::Result<()> {
    let magic: [u8; 4] = [0x01, 0x00, 0x00, 0x00,];
    let mut buf: [u8; 4] = [0x00; 4];
    reader.read_exact(&mut buf)?;
    if buf == magic {
        Ok(())
    } else {
        panic!("invalid on decode_magic");
    }
}

fn id_to_section(id: Byte) -> Section {
    match id {
        0 => Section::Custom,
        1 => Section::Type,
        2 => Section::Import,
        3 => Section::Function,
        4 => Section::Table,
        5 => Section::Memory,
        6 => Section::Global,
        7 => Section::Export,
        8 => Section::Start,
        9 => Section::Element,
        10 => Section::Code,
        11 => Section::Data,
        _ => panic!("invalid on id_to_section")
    }
}