(module $moddd
  ;; types
  (type $tp1 (func))
  (type $tp2 (func (param i32)))
  (type $tp3 (func (param i32) (param i64)))
  (type $tp4 (func (param i32) (param i64) (result f32)))
  (type $tp5 (func (result f64)))
  ;; (type $uniop (func (param i64)))

  ;; imports
  ;; (import "imp_func" "1" (func $f1 (type 0)))
  ;; (import "imp_func" "2" (func (type $tp2)))
  ;; (import "imp_func" "3" (func (type 1) (param i32) (param i64)))
  ;; (import "imp_func" "4" (func $fjiao (type $tp3) (param f32) (param f64) (result i32)))
  ;; (import "imp_func" "5" (func (type 2)))
  ;; (import "imp_func" "6" (func (type 3) (result i64)))
  ;; (import "imp_table" "b" (table $first 2 funcref))
  ;; (import "imp_table" "b" (table 54 83 funcref))
  ;; (import "imp_mem" "1" (memory $fqew 1234))
  ;; (import "imp_mem" "2" (memory 64 56849))
  ;; (import "imp_global" "1" (global $ggg f64))
  ;; (import "imp_global" "2" (global (mut i32)))

  ;; tables
  (table $second 43 funcref)

  ;; mems
  ;; (memory $aaaaaa 98)

  ;; globals
  (global $g1 i32)
  (global $g2 (mut f64))

  ;; funcs
  (func $f1 (type 0))
  ;; (func $fjao (type 1))
  ;; (func $11 (type $tp2))
  ;; (func $fa (type 1) (param $p2 i32) (param i64))
  ;; (func $afm (type $tp3) (param $arg1 f32) (param f64) (result i32))
  ;; (func $aaa (type 2) (local i32))
  ;; (func $aaa (type 2) (local $local1 i32) (local i64))
  ;; (func $bbb (type 3) (result i64))
  ;; (func (type 3) (result i64))
  ;; (func (type $uniop) (param i64) (local i32))
  ;; (func (type $tp2) (param i32) (param i64) (local i32))
  ;; (func (type 3) (result i64) (local $ooo f64) (local $jjj i32))
  (func $instrs (type 0)
    (local i32)
    (local $l i32)

    ;; block $label (result f64)
    ;;   nop br $label 
    ;;   br_table 3 6 89 798 $label
    ;;   block $inner (result f64)
    ;;   end
    ;; end
    ;; loop $label_loop (result f64) nop br $label_loop end
    ;; if $if_label (result i32) else $if_label nop end $if_label
    ;; br 0
    ;; br $if_label
    ;; br_if 0
    ;; br_if $lbl
    
    ;; call 0
    ;; call $f1
    call_indirect (type $tp1)
    call_indirect (type $tp2) (param i32)
    call_indirect (type $tp3) (param i32) (param i64)
    call_indirect (type $tp4) (param i32) (param i64) (result f32)
    call_indirect (type $tp5) (result f64)

    ;; local.get 1
    ;; local.get $l
    ;; local.set 1
    ;; local.set $l
    ;; local.tee 1
    ;; local.tee $l
    ;; global.get 1
    ;; global.get $g2
    ;; global.set 0
    ;; global.set $g1

    ;; i32.load
    ;; i64.load
    ;; f32.load
    ;; f64.load
    ;; i32.load8_s
    ;; i32.load8_u
    ;; i32.load16_s
    ;; i32.load16_u
    ;; i64.load8_s
    ;; i64.load8_u
    ;; i64.load16_s
    ;; i64.load16_u
    ;; i64.load32_s
    ;; i64.load32_u
    ;; i32.store
    ;; i64.store
    ;; f32.store
    ;; f64.store
    ;; i32.store8
    ;; i32.store16
    ;; i64.store8
    ;; i64.store16
    ;; i64.store32

    ;; i32.const 4
    ;; i32.const 3456789
  )


  ;; exports
  ;; (export "t" (table $second))
  ;; (export "m" (memory $aaaaaa))
  ;; (export "g" (global $wowowwow))

  ;; start
  ;; (start 52)

  ;; elem
  ;; (elem 0 (offset nop) 1 2 3 4 5)

  ;; data
  ;; (data 4 (offset) "jlac84myrtqp9")
)