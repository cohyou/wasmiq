(module
    (func $abs (param $input i32) (result i32)
        (local $answer i32)
        local.get $input
        i32.const 0
        i32.lt_s
        if
            i32.const 0
            local.get $input
            i32.sub
            local.set $answer
        else
            local.get $input
            local.set $answer
        end
        local.get $answer
    )

    (func (export "main") (result i32)
        i32.const -132
        call $abs
    )
)