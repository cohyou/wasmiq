(module
    (func (export "main")
        i32.const 1
        i32.const 2
        i32.add ;; 足し算
        drop

        f32.const 10.5
        f32.const 1.25
        f32.sub ;; 引き算
        drop
    )
)
