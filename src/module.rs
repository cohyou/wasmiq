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
};

pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    tables: Vec<Table>,
    mems: Vec<Mem>,
    globals: Vec<Global>,
    elem: Vec<Elem>,
    data: Vec<Data>,
    start: Option<Start>,
    imports: Vec<Import>,
    exports: Vec<Export>,
}

pub type TypeIdx = u32;
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;
pub type LabelIdx = u32;
pub type LocalIdx = u32;

pub struct Func {
    tp: TypeIdx,
    locals: Vec<ValType>,
    body: Expr,
}

pub struct Table(TableType);

pub struct Mem(MemType);

pub struct Global {
    tp: GlobalType,
    init: Expr,
}

struct Elem {
    table: TableIdx,
    offset: Expr,
    init: Vec<FuncIdx>,
}

#[derive(Clone)]
struct Data {
    data: MemIdx,
    offset: Expr,
    init: Vec<Byte>,
}

struct Start(pub FuncIdx);

struct Export {
    name: Name,
    desc: ExportDesc,
}

enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

pub struct Import {
    module: Name,
    name: Name,
    desc: ImportDesc,
}

enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

pub use validate::Context;

pub fn module_decode() -> Result<Module, Error> {
    unimplemented!();
}

pub fn module_parse() {
    unimplemented!();
}

pub fn module_validate(module: Module) -> Result<(), Error> {
    let _externtype = module.validate()?;
    Ok(())
}

pub use instance::module_instanciate;

pub fn module_imports() {

}

pub fn module_exports() {

}