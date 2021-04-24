mod store;
pub use store::store_init;

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
    TableIdx,
    LocalIdx,
    LabelIdx,
    MemIdx,
    Func,
    Context,

    Import,
    ImportDesc,
    Table,
    Mem,
    Global,
    Export,
    ExportDesc,
    Start,
    Elem,
    Data,

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


mod instance;
pub use instance::instance_export;

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
use val::{
    Byte,
    Name,
    Mut,
};

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
};

mod instr;
use instr::{
    MemArg,
    Instr, 
    Expr,
    ValSize,
    ValSign,
    IUnOp,
    FUnOp,
    IBinOp,
    FBinOp,
    ITestOp,
    IRelOp,
    FRelOp,
    CvtOp,
    BlockType,
};

mod error;
pub use error::Error;

mod runtime;
use runtime::{
    Store,
    ModuleInst,
    ExternVal,
    Frame,
    FuncInst,
    Thread,
    FuncAddr,
    TableAddr,
    MemAddr,
    StackEntry,
    Val,
    GlobalAddr,
    GlobalInst,
    TableInst,
    MemInst,
    ExportInst,
    Result,
};

mod decoder;
use decoder::{
    decode_module,
};

mod parser;
use parser::{
    Parser,
};

mod encoder;
pub use encoder::{
    module_encode,
};

#[test]
fn test_invoke() {
    use std::io::{Cursor, BufReader};
    let s = r#"
    (type (func (result i32)))
    (func $const (type 0) (result i32) i32.const 42)
    "#;
    let cursor = Cursor::new(s);
    let mut reader = BufReader::new(cursor);
    if let Ok(module) = module_parse(&mut reader) {
        let mut store = store_init();
        if let Ok(moduleinst) = module_instanciate(&mut store, module, vec![]) {
            println!("store1: {:?}", store);
            println!("moduleinst: {:?}", moduleinst);
            if let Ok(vals) = func_invoke(&mut store, 1, vec![]) {
                println!("store2: {:?}", store);
                println!("vals: {:?}", vals);
            }
        }
    }
}