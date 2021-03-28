(module $main
  ;; (type (func))
  ;; (type $i32ret (func (result i32)))
  ;; (type $i64ret (func (result i64)))
  ;; (type $tak (func (param i64) (param i64) (param i64) (result i64)))
  (type $fib (func (param i64) (result i64)))

  ;; (func $const (type $i32ret) i32.const 42)
  ;; (func $f1add (type $i32ret) i32.const 42 i32.const 8734 i32.add)
  ;; (func $f1sub (type $i32ret) i32.const 90672598 i32.const 2346789 i32.sub)
  ;; (func $f1mul (type $i32ret) i32.const 3 i32.const 4 i32.mul)
  ;; (func $f1div (type $i32ret) i32.const 45 i32.const 5 i32.div_u)
  ;; (func $f1rem (type $i32ret) i32.const 37 i32.const 10 i32.rem_u)

  ;; (func $f2call (type $i32ret) call $f1add)

  ;; (func $f2if (type $i32ret) i32.const 0 if (result i32) i32.const 1 else i32.const 0 end)

  ;; (func $f3sub (type $i64ret) i64.const 100 i64.const 48 i64.sub)

  ;; (func $tak (type $tak) (param $x i64) (param $y i64) (param $z i64) (result i64)
  ;;   local.get $x 
  ;;   local.get $y 
  ;;   i64.le_u
  ;;   if (result i64) 
  ;;     local.get $z
  ;;   else    
  ;;     local.get $x i64.const 1 i64.sub local.get $y local.get $z call $tak
  ;;     local.get $y i64.const 1 i64.sub local.get $z local.get $x call $tak
  ;;     local.get $z i64.const 1 i64.sub local.get $x local.get $y call $tak
  ;;     call $tak
  ;;   end
  ;; )

  (func $fib (type $fib)
    local.get 0
    i64.const 0
    i64.eq
    if (result i64)
      i64.const 0
    else
      local.get 0
      i64.const 1
      i64.eq
      if (result i64)
        i64.const 1
      else
        local.get 0 i64.const 1 i64.sub call $fib
        local.get 0 i64.const 2 i64.sub call $fib
        i64.add
      end
    end
  )

  (func $start (type $fib) i64.const 30 call $fib)

  (start $start)
)