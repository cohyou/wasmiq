(module
    (global $counter (mut i32) (i32.const 0))
    (global $step i32 (i32.const 1))
    (func $count (export "count")
        global.get $counter
        global.get $step
        i32.add
        global.set $counter
    )

    (func (export "main") (result i32)
        call $count
        call $count
        call $count
        global.get $counter
    )
)