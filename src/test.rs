use super::*;

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

#[test]
fn test_wast_file_3_3() {
    let file_name = "./wast/3-3.wat";
    // show_file_parse_result(file_name);
    assert_eq!(invoke_file_assert_eq(file_name, 0), Some(vec![]));
}

#[test]
fn test_wast_file_instruction_type() {
    let file_name = "./wast/instruction-type.wat";
    assert_eq!(invoke_file_assert_eq(file_name, 0), Some(vec![]));
}

#[test]
fn test_wast_file_function() {
    let file_name = "./wast/function.wat";
    // show_file_parse_result(file_name);
    assert_eq!(invoke_file_assert_eq(file_name, 1), Some(vec![]));
}

#[test]
fn test_wast_file_condition_blocktype() {
    let file_name = "./wast/condition-blocktype.wat";
    assert_eq!(invoke_file_assert_eq(file_name, 1), Some(vec![]));
}

#[test]
fn test_wast_file_loop() {
    let file_name = "./wast/loop.wat";
    assert_eq!(invoke_file_assert_eq(file_name, 1), Some(vec![]));
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
    p!(store.funcs);
    p!(store.tables);
    p!(store.mems);
    p!(store.globals);

    Ok(vals)
}

#[allow(dead_code)]
fn invoke_file_assert_eq(file_name: &str, idx: usize) -> Option<Vec<Val>> {
    match invoke_file(file_name, idx) {
        Ok(vals) => Some(vals),
        Err(err) => {
            println!("error: {:?}", err);
            None
        },
    }
    
}

#[allow(dead_code)]
fn invoke_file(file_name: &str, idx: usize) -> Result<Vec<Val>, Error> {
    use std::fs::File;
    use std::io::{BufReader};
    let f = File::open(file_name).unwrap();
    let mut reader = BufReader::new(f);
    let module = module_parse(&mut reader)?;
    let mut store = store_init();
    let _moduleinst = module_instanciate(&mut store, module, vec![])?;
    // p!(store.funcs);
    // p!(store.tables);
    // p!(store.mems);
    // p!(store.globals);
    let vals = func_invoke(&mut store, idx, vec![])?;

    Ok(vals)
}