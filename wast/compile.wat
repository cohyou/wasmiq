(module
    (type (func (result i64)))
    (func (type 0)
        i32.const 1
        if (result i64)
            i64.const 100
        else
            i64.const 200
        end
    )
)