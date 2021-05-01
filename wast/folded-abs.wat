(module
    (func $abs (param $input i32) (result i32)
        (if (result i32)
            (i32.lt_s (local.get $input) (i32.const 0))
            (then
                (i32.sub (i32.const 0) (local.get $input))
            )
            (else (local.get $input))
        )
    )

    (func (export "main") (result i32)
        (call $abs (i32.const -132))
    )
)