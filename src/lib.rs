mod store;
pub use store::store_init;
use store::Store;

mod module;
pub use module::{
    module_decode,
    module_parse,
    module_validate,
    module_instanciate,
    module_imports,
    module_exports,
};
pub use module::{
    Module,
};
use module::{
    TypeIdx,
    FuncIdx,
    GlobalIdx,
    LocalIdx,
    LabelIdx,
    Context,
};

mod instance;
pub use instance::instance_export;
use instance::{
    ModuleInst, 
    ExternVal,
};

mod func;
pub use func::{
    func_alloc,
    func_type,
    func_invoke,
};

mod table;
pub use table::{
    table_alloc,
    table_type,
    table_read,
    table_write,
    table_size,
    table_grow,
};

mod mem;
pub use mem::{
    mem_alloc,
    mem_type,
    mem_read,
    mem_write,
    mem_size,
    mem_grow,
};

mod global;
pub use global::{
    global_alloc,
    global_type,
    global_read,
    global_write,
};

mod val;
use val::Name;

mod types;
use types::{
    ValType,
    ResultType,
    FuncType,
    Limits,
    MemType,
    ElemType,
    TableType,
    GlobalType,
    ExternType,
    Mut,
};

mod instr;
use instr::{
    MemArg,
    Instr, 
    Expr,
};

mod error;
pub use error::Error;