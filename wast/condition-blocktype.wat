(module
    (func $abs (param $input i32) (result i32)
        local.get $input
        i32.const 0
        i32.lt_s
        if (result i32)
            i32.const 0
            local.get $input
            i32.sub
        else
            local.get $input
        end
    )

    (func (export "main") (result i32)
        i32.const -132
        call $abs
    )
)