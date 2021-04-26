(module
    (func (export "main")
        i32.const 1
        f32.convert_i32_s
        f32.const 1.25
        f32.sub ;; 引き算
        drop
    )
)