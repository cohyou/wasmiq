(module
    (func (param i32) (param i32) (result i32)
        (local f32)
        local.get 0 ;; 最初の引数
        local.get 1 ;; 2番目の引数
        f32.const 1.1
        local.set 2 ;; local変数
        i32.add
    )

    (func)

    (func (export "main") (result i32)
        i32.const 2
        i32.const 3
        call 0 ;; 最初の関数を呼び出す
        call 1 ;; 2番目の関数を呼び出す
    )
)