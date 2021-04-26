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

#[test]
fn test_invoke2() {
    let s = r#"
    (type (func (result i32)))
    (func $const (export "addtest") (type 0) (result i32) i32.const 42 i32.const 42 i32.add)
    "#;
    assert_eq!(invoke_assert_eq(s), Some(vec![Val::I32Const(84)]));
}

#[test]
fn test_invoke3() {
    let s = r#"
    (type (func (result i32)))
    (func $const (export "addtest") (type 0) (result i32)
        i32.const 100 
        i32.const 100 
        i32.add 
        i32.const 1 
        i32.sub)
    "#;
    assert_eq!(invoke_assert_eq(s), Some(vec![Val::I32Const(199)]));
}

#[test]
fn test_invoke4() {
    let s = r#"
    (type (func (result i32)))
    (func $const (export "calctest") (type 0) (result i32)
        (i32.sub (i32.add (i32.const 100) (i32.const 50)) (i32.const 1)))
    "#;
    assert_eq!(invoke_assert_eq(s), Some(vec![Val::I32Const(149)]));
}

#[test]
fn test_gensym() {
    let s = r#"
    (type (func (param f32)))
    (func (export "main") (param i32) (result i32) i32.const 32)
    "#;
    show_parse_result(s);
}

#[test]
fn test_resolve_func_id() {
    let s = r#"
    (type (func))
    (func (export "main") nop)
    "#;
    show_parse_result(s);
}

#[test]
fn test_wast() {
    let s = r#"
    (type (func (result i32)))
    (type (func))
    (func $const (export "calctest") (type 0) (result i32)
        (i32.sub (i32.add (i32.const 100) (i32.const 50)) (i32.const 1)))
    ;; (func (export "nop") (type 1))
    "#;
    show_parse_result(s);
}

#[test]
fn test_wast_export() {
    let s = r#"
    (type (func (result i32)))
    (type (func))
    (func (export "nop") (type 1))
    "#;
    show_parse_result(s);
}

#[test]
fn test_wast_file() {
    show_file_parse_result("./wast/chapter3-1.wat");
}

#[allow(dead_code)]
fn show_file_parse_result(file_name: &str) {
    use std::fs::File;
    use std::io::{BufReader};
    let f = File::open(file_name).unwrap();
    let mut reader = BufReader::new(f);
    match module_parse(&mut reader) {
        Ok(module) => {
            p!(module.types);
            p!(module.imports);
            p!(module.funcs);
            p!(module.tables);
            p!(module.mems);
            p!(module.globals);
            p!(module.exports);
            p!(module.elem);
            p!(module.data);
            p!(module.start);
        },
        Err(err) => p!(err),
    }
}

#[allow(dead_code)]
fn show_parse_result(s: &str) {
    use std::io::{Cursor, BufReader};
    let cursor = Cursor::new(s);
    let mut reader = BufReader::new(cursor);
    match module_parse(&mut reader) {
        Ok(module) => {
            p!(module.types);
            p!(module.imports);
            p!(module.funcs);
            p!(module.exports);
        },
        Err(err) => p!(err),
    }
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