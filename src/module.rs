mod validate;
mod instance;

use crate::{
    Name,
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
    imports: Vec<Import>,
}

pub type TypeIdx = u32;
pub type FuncIdx = u32;
// pub(super) type Tableidx = u32;
// pub(super) type Memidx = u32;
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
    module.validate()
}

pub use instance::module_instanciate;

pub fn module_imports() {

}

pub fn module_exports() {

}