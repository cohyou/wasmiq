use super::*;

#[test]
fn test_parse_if() {
    let s = r#"
    (func
        (param $exponent i32) (result i32)
        local.get $exponent
        i32.eqz
        if 
            i32.const 1
            return
        end
        i32.const 2
    )
    "#;
    assert!(parse_str(s).is_some());
}

#[test]
fn test_parse_blocktype() {
    let s = r#"
    (type (func (param i32 i32) (result i32)))
    (func
        block $b
            loop $l (type 0) (param i32 i32) (result i32)
                if (param f64)
                end
                br $b
            end
        end
    )
    "#;
    assert!(parse_str(s).is_some());
}

#[test]
fn test_parse_end_label() {
    let s = r#"
    (func
        block nop end
        loop nop end
        if nop end
        block $block nop end $block
        loop $loop nop end $loop
        if $if nop end $if
    )
    "#;
    assert!(parse_str(s).is_some());
}

#[test]
fn test_parse_label2() {
    let s = r#"
    (func $power (export "power")
        (param $base i32) (param $exponent i32) (result i32)
        (local $acc i32)
        local.get $exponent
        i32.eqz
        if
            i32.const 1
            return
        end

        i32.const 1
        local.set $acc

        loop $label (result i32)
            local.get $base
            local.get $acc
            i32.mul
            local.tee $acc

            local.get $exponent
            i32.const 1
            i32.sub
            local.tee $exponent
            i32.const 0
            i32.ne
            br_if $label
        end
    )
    "#;
    assert!(parse_str(s).is_some());
}

#[allow(dead_code)]
fn parse_str(s: &str) -> Option<()> {
    use std::io::{Cursor, BufReader};
    let cursor = Cursor::new(s);
    let reader = BufReader::new(cursor);
    let mut parser = Parser::new(reader);
    match parser.parse() {
        Ok(_) => {
            println!("module: {:?}", parser.module);
            Some(())
        },
        Err(err) => {
            println!("parse error: {:?}", err);
            None
        },
    }
}