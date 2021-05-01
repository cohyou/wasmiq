(module
    (memory 1)

    (func (export "main")
        i32.const 2
        memory.grow
        drop ;; memory.growは伸長前のサイズ1を返す

        memory.size ;; 1+2=3を返す
        drop

        i32.const 170000
        i32.const 1
        i32.store
    )
)