(module
    (func $return (export "return") (param $n i32) (result i32)
        i32.const 10000
        return
        i32.const 256
    )

    (func $mid (export "mid") (param $n i32) (result i32)
        i32.const 1
        call $return
        i32.const 512
        i32.add
    )

    (func (export "main") (result i32)
        i32.const 3
        call $mid
    )
)