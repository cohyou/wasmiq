use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_global(&mut self, token_global: Token) -> Result<(), RewriteError> {
        self.rewrite_inline_export_import(token_global)
    }
}

#[test]
fn test_rewrite_global() {
    // normal
    assert_eq_rewrite(
        "(global i32 end)", 
        "(module (global i32 end))"
    );
    assert_eq_rewrite(
        "(global i32 i64.const 8128 end)", 
        "(module (global i32 i64.const 8128 end))"
    );
    assert_eq_rewrite(
        "(global (mut f32) i64.const 8128 end)", 
        "(module (global (mut f32) i64.const 8128 end))"
    );
    assert_eq_rewrite(
        "(global $id1 i32 end)", 
        "(module (global $id1 i32 end))"
    );
    assert_eq_rewrite(
        "(global $id2 i32 i64.const 8128 end)", 
        "(module (global $id2 i32 i64.const 8128 end))"
    );
    assert_eq_rewrite(
        "(global $id3 (mut f32) i64.const 8128 end)", 
        "(module (global $id3 (mut f32) i64.const 8128 end))"
    );

    // import
    assert_eq_rewrite(
        r#"(global (import "name1" "name2") i32)"#, 
        r#"(module (import "name1" "name2" (global i32)))"#
    );
    assert_eq_rewrite(
        r#"(global (import "name1" "name2") (mut f64))"#,
        r#"(module (import "name1" "name2" (global (mut f64))))"#
    );
    assert_eq_rewrite(
        r#"(global $imp_global_const1 (import "name1" "name2") i64)"#, 
        r#"(module (import "name1" "name2" (global $imp_global_const1 i64)))"#
    );
    assert_eq_rewrite(
        r#"(global $imp_global_const2 (import "name1" "name2") (mut f32))"#, 
        r#"(module (import "name1" "name2" (global $imp_global_const2 (mut f32))))"#
    );
}

#[test]
fn test_rewrite_global_export() {
    assert_eq_rewrite(
        r#"(global (export "expname1"))"#, 
        r#"(module (export "expname1" (global <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(global $expid1 (export "expname2"))"#, 
        r#"(module (export "expname2" (global $expid1)))"#
    );
    assert_eq_rewrite(
        r#"(global (export "expname3") (export "expname4"))"#, 
        r#"(module (export "expname3" (global <#:gensym>)) (export "expname4" (global <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(global $expid2 (export "expname5") (export "expname6"))"#, 
        r#"(module (export "expname5" (global $expid2)) (export "expname6" (global $expid2)))"#
    );
}

#[test]
fn test_rewrite_global_import_export() {
    assert_eq_rewrite(
        r#"(global (export "expname3") (import "impname1" "impname2") i32)"#, 
        r#"(module (export "expname3" (global <#:gensym>)) (import "impname1" "impname2" (global i32)))"#
    );
    assert_eq_rewrite(
        r#"(global $expimpid (export "expname3") (import "impname1" "impname2") (mut f64))"#, 
        r#"(module (export "expname3" (global $expimpid)) (import "impname1" "impname2" (global $expimpid (mut f64))))"#
    );
}
