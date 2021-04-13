use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_memory(&mut self, token_mem: Token) -> Result<(), RewriteError> {
        self.rewrite_inline_export_import(token_mem)
    }
}

#[test]
fn test_rewrite_mem_normal() {
    assert_eq_rewrite("(memory 1)", "(module (memory 1))");
    assert_eq_rewrite("(memory 10 100)", "(module (memory 10 100))");
    assert_eq_rewrite("(memory $id1 1000)", "(module (memory $id1 1000))");
    assert_eq_rewrite("(memory $id2 10000 20000)", "(module (memory $id2 10000 20000))");
}

#[test]
fn test_rewrite_mem_import() {
    assert_eq_rewrite(
        r#"(memory (import "name1" "name2") 2)"#, 
        r#"(module (import "name1" "name2" (memory 2)))"#
    );
    assert_eq_rewrite(
        r#"(memory (import "name3" "name4") 20 40)"#, 
        r#"(module (import "name3" "name4" (memory 20 40)))"#
    );
    assert_eq_rewrite(
        r#"(memory $imp_mem1 (import "name1" "name2") 2)"#, 
        r#"(module (import "name1" "name2" (memory $imp_mem1 2)))"#
    );
    assert_eq_rewrite(
        r#"(memory $imp_mem2 (import "name3" "name4") 20 40)"#, 
        r#"(module (import "name3" "name4" (memory $imp_mem2 20 40)))"#
    );
}

#[test]
fn test_rewrite_mem_export() {
    assert_eq_rewrite(
        r#"(memory (export "expname1"))"#, 
        r#"(module (export "expname1" (memory <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(memory $expid1 (export "expname2"))"#, 
        r#"(module (export "expname2" (memory $expid1)))"#
    );
    assert_eq_rewrite(
        r#"(memory (export "expname3") (export "expname4"))"#, 
        r#"(module (export "expname3" (memory <#:gensym>)) (export "expname4" (memory <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(memory $expid2 (export "expname5") (export "expname6"))"#, 
        r#"(module (export "expname5" (memory $expid2)) (export "expname6" (memory $expid2)))"#
    );
}

#[test]
fn test_rewrite_mem_import_export() {
    assert_eq_rewrite(
        r#"(memory (export "expname3") (import "impname1" "impname2") 1234)"#, 
        r#"(module (export "expname3" (memory <#:gensym>)) (import "impname1" "impname2" (memory 1234)))"#
    );
    assert_eq_rewrite(
        r#"(memory $expimpid (export "expname3") (import "impname1" "impname2") 4321 5678)"#, 
        r#"(module (export "expname3" (memory $expimpid)) (import "impname1" "impname2" (memory $expimpid 4321 5678)))"#
    );
}