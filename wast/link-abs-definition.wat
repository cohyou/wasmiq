(module
    (table (export "table") 2 funcref)
    (elem (i32.const 0) $identity $invert)
    (type $func_type (func (param i32) (result i32)))

    (func $identity (type $func_type)
        local.get 0
    )

    (func $invert (type $func_type)
        i32.const 0
        local.get 0
        i32.sub
    )
)