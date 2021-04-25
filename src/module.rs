mod validate;
mod instance;

pub use instance::{
    alloc_hostfunc,
    alloc_table,
    find_tabletype,
    grow_table,
    alloc_mem,
    find_memtype,
    grow_mem,
    alloc_global,
    find_globaltype,
};

use crate::{
    Name,
    Byte,
    ValType,
    FuncType,
    MemType,
    TableType,
    GlobalType,
    Expr,
    Error,
    Parser,
    ExternType,
};

#[derive(Default, Debug)]
pub struct Module {
    pub id: Option<String>,
    pub types: Vec<FuncType>,
    pub funcs: Vec<Func>,
    pub tables: Vec<Table>,
    pub mems: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elem: Vec<Elem>,
    pub data: Vec<Data>,
    pub start: Option<Start>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

pub type TypeIdx = u32;
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;
pub type LabelIdx = u32;
pub type LocalIdx = u32;

#[derive(Clone, Default, Debug)]
pub struct Func {
    pub tp: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}

#[derive(Debug)]
pub struct Table(pub TableType);

#[derive(Debug)]
pub struct Mem(pub MemType);

#[derive(Debug)]
pub struct Global {
    pub tp: GlobalType,
    pub init: Expr,
}

#[derive(Debug)]
pub struct Elem {
    pub table: TableIdx,
    pub offset: Expr,
    pub init: Vec<FuncIdx>,
}

#[derive(Clone, Debug)]
pub struct Data {
    pub data: MemIdx,
    pub offset: Expr,
    pub init: Vec<Byte>,
}

#[derive(Debug)]
pub struct Start(pub FuncIdx);

#[derive(Debug)]
pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}

#[derive(Debug)]
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

#[derive(Debug)]
pub struct Import {
    pub module: Name,
    pub name: Name,
    pub desc: ImportDesc,
}

#[derive(Debug)]
pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

pub use validate::Context;

use std::io::{Read, Seek};
use crate::{
    decode_module,
};

pub fn module_decode(reader: &mut impl Read) -> Result<Module, Error> {
    if let Ok(module) = decode_module(reader) {
        Ok(module)
    } else {
        Err(Error::Invalid("module_decode".to_owned()))
    }
}

pub fn module_parse(reader: &mut (impl Read + Seek)) -> Result<Module, Error> {
    let mut parser = Parser::new(reader);
    match parser.parse() {
        Ok(()) => Ok(parser.module),
        Err(err) => Err(Error::OnParse(err)),
    }
}

pub fn module_validate(module: Module) -> Result<(), Error> {
    let _externtype = module.validate()?;
    Ok(())
}

pub use instance::module_instanciate;

pub fn module_imports(module: Module) -> Vec<(Name, Name, ExternType)> {
    let externtypes = module.validate().unwrap();
    let imports = module.imports;
    assert_eq!(imports.len(), externtypes.0.len());
    let mut results = vec![];
    for (import, externtype) in imports.iter().zip(externtypes.0) {
        results.push( (import.module.clone(), import.name.clone(), externtype) );
    }
    results
}

pub fn module_exports(module: Module) -> Vec<(Name, ExternType)> {
    let externtypes = module.validate().unwrap();
    let exports = module.exports;
    assert_eq!(exports.len(), externtypes.1.len());
    let mut results = vec![];
    for (export, externtype) in exports.iter().zip(externtypes.1) {
        results.push( (export.name.clone(), externtype) );
    }
    results
}
