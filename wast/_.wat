(module
  (type $void (func))
  (type $two (func (param $aaa i64) (param i64)))
  (type $three (func (param i32) (param i64) (param i64)))
  (import "wasi" "log" (func (type 1)))
  (import "wasi" "table" (table 111 222 anyfunc))
  (import "wasi" "mem" (memory 3 4))
  (import "wasi" "global" (global i32))
  (func $bbb (type $two) (param $bb i64) (param i64)
    (local i32) (local $aa4 i64)
    get_local $aa4
  )
  (table $tab 5 anyfunc)
  (memory $mem 1 2)
  (global $gg1 (mut i32) nop)
  (export "ex2" (func $bbb))
  (export "ex4" (table $tab))
  (export "ex6" (memory $mem))
  (export "ex8" (global $gg1))
  (start 1)
  (elem $tab (offset nop) 0 $bbb 7 8)
  (data $mem (offset nop) "fceiqwxgabkfiuha")
)