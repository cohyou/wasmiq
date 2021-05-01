(module
    (func (export "unreachable")
        unreachable
    )
    (func (export "div_zero") (result i32)
        i32.const 10
        i32.const 0
        i32.div_s
    )
)