(module
    (func $fibonacci (export "fibonacci") (param $n i32) (result i32)
        local.get $n
        i32.const 1
        i32.le_u
        if
            local.get $n
            return
        end

        ;; n-1を求める
        local.get $n
        i32.const 1
        i32.sub
        call $fibonacci

        ;; n-2を求める
        local.get $n
        i32.const 2
        i32.sub
        call $fibonacci

        i32.add
    )

    (func (export "main") (result i32)
        i32.const 20
        call $fibonacci
    )
)