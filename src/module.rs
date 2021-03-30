mod validate;
mod instance;

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

#[derive(Default)]
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

#[derive(Clone, Default)]
pub struct Func {
    pub tp: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}

pub struct Table(pub TableType);

pub struct Mem(pub MemType);

pub struct Global {
    pub tp: GlobalType,
    pub init: Expr,
}

pub struct Elem {
    pub table: TableIdx,
    pub offset: Expr,
    pub init: Vec<FuncIdx>,
}

#[derive(Clone)]
pub struct Data {
    pub data: MemIdx,
    pub offset: Expr,
    pub init: Vec<Byte>,
}

pub struct Start(pub FuncIdx);

pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}

pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

pub struct Import {
    pub module: Name,
    pub name: Name,
    pub desc: ImportDesc,
}

pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

pub use validate::Context;

use std::io::Read;
use crate::{
    decode_module,
};
pub fn module_decode(reader: &mut impl Read) -> Result<Module, Error> {
    if let Ok(module) = decode_module(reader) {
        Ok(module)
    } else {
        Err(Error::Invalid)
    }
}

use std::env;
use std::fs::File;

pub fn module_parse() {
    let args = env::args().collect::<Vec<String>>();
    let file_name = &args[1];
    let reader = File::open(file_name).unwrap();
    let mut parser = Parser::new(reader);
    match parser.parse() {
        Err(err) => {
            println!("PARSE ERROR: {:?}", err);
            return;
        },
        _ => {},
    }
}

pub fn module_validate(module: Module) -> Result<(), Error> {
    let _externtype = module.validate()?;
    Ok(())
}

pub use instance::module_instanciate;

pub fn module_imports(_module: Module) -> (Name, Name, Vec<ExternType>) {
    unimplemented!()
}

pub fn module_exports(_module: Module) -> (Name, Vec<ExternType>) {
    unimplemented!()
}
