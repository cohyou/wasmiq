(module
    (memory 1)
    (func $start
        ;; メモリのオフセット0に11を書き込む
        i32.const 0
        i32.const 11
        i32.store
    )

    (start $start)
    (export "memory" (memory 0))
)