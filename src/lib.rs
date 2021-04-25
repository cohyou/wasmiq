#[macro_export]
macro_rules! p {
    ($e:expr) => { println!(concat!(stringify!($e), ": {:?}"), {&$e}); };
}

#[macro_export]
macro_rules! pp {
    ($i:expr, $e:expr) => { println!(concat!(stringify!($i), ": {:?}"), {&$e}); };
}

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
    ExecResult,
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
    let s = r#"
    (type (func (result i32)))
    (func $const (export "val42") (type 0) (result i32) i32.const 42)
    "#;
    assert_eq!(invoke_assert_eq(s), Some(vec![Val::I32Const(42)]));
}

#[allow(dead_code)]
fn invoke_assert_eq(s: &str) -> Option<Vec<Val>> {
    match invoke(s) {
        Ok(vals) => Some(vals),
        Err(err) => {
            println!("error: {:?}", err);
            None
        },
    }
    
}

#[allow(dead_code)]
fn invoke(s: &str) -> Result<Vec<Val>, Error> {
    use std::io::{Cursor, BufReader};
    let cursor = Cursor::new(s);
    let mut reader = BufReader::new(cursor);
    let module = module_parse(&mut reader)?;
    let mut store = store_init();
    let _moduleinst = module_instanciate(&mut store, module, vec![])?;
    let vals = func_invoke(&mut store, 0, vec![])?;
    println!("store: {:?}", store);

    Ok(vals)
}