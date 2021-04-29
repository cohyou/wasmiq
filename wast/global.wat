(module
    ;; (global $counter (mut i32) (i32.const 0))
    ;; (global $step i32 (i32.const 1))
    ;; (func $count (export "count")
    ;;     global.get $counter
    ;;     global.get $step
    ;;     i32.add
    ;;     global.set $counter
    ;; )

    ;; (func (export "main") (result i32)
    ;;     call $count
    ;;     call $count
    ;;     call $count
    ;;     ;; global.get $counter
    ;; )

    (global (mut i32) (i32.const 0))
    (global i32 (i32.const 1))
    (func $count (export "count")
        global.get 0
        global.get 1
        i32.add
        global.set 0
    )

    (func (export "main") (result i32)
        call $count
        call $count
        call $count
        global.get 0
    )
)