use super::*;

impl<R> Rewriter<R> where R: Read + Seek {
    pub fn rewrite_table(&mut self, token_table: Token) -> Result<(), RewriteError> {
        self.rewrite_inline_export_import(token_table)
    }
}

#[test]
fn test_rewrite_table_normal() {
    assert_eq_rewrite("(table 1 funcref)", "(module (table 1 funcref))");
    assert_eq_rewrite("(table 10 100 funcref)", "(module (table 10 100 funcref))");
    assert_eq_rewrite("(table $id1 1000 funcref)", "(module (table $id1 1000 funcref))");
    assert_eq_rewrite("(table $id2 10000 20000 funcref)", "(module (table $id2 10000 20000 funcref))");
}

#[test]
fn test_rewrite_table_import() {
    assert_eq_rewrite(
        r#"(table (import "name1" "name2") 2 funcref)"#, 
        r#"(module (import "name1" "name2" (table 2 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table (import "name3" "name4") 20 40 funcref)"#, 
        r#"(module (import "name3" "name4" (table 20 40 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table $imp_table1 (import "name1" "name2") 2 funcref)"#, 
        r#"(module (import "name1" "name2" (table $imp_table1 2 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table $imp_table2 (import "name3" "name4") 20 40 funcref)"#, 
        r#"(module (import "name3" "name4" (table $imp_table2 20 40 funcref)))"#
    );
}

#[test]
fn test_rewrite_table_export() {
    assert_eq_rewrite(
        r#"(table (export "expname1"))"#, 
        r#"(module (export "expname1" (table <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(table $expid1 (export "expname2"))"#, 
        r#"(module (export "expname2" (table $expid1)))"#
    );
    assert_eq_rewrite(
        r#"(table (export "expname3") (export "expname4"))"#, 
        r#"(module (export "expname3" (table <#:gensym>)) (export "expname4" (table <#:gensym>)))"#
    );
    assert_eq_rewrite(
        r#"(table $expid2 (export "expname5") (export "expname6"))"#, 
        r#"(module (export "expname5" (table $expid2)) (export "expname6" (table $expid2)))"#
    );
}

#[test]
fn test_rewrite_table_import_export() {
    assert_eq_rewrite(
        r#"(table $expimpid (export "expname3") (import "impname1" "impname2") 1234 funcref)"#, 
        r#"(module (export "expname3" (table $expimpid)) (import "impname1" "impname2" (table $expimpid 1234 funcref)))"#
    );
    assert_eq_rewrite(
        r#"(table (export "expname3") (import "impname1" "impname2") 4321 5678 funcref)"#, 
        r#"(module (export "expname3" (table <#:gensym>)) (import "impname1" "impname2" (table 4321 5678 funcref)))"#
    );
}