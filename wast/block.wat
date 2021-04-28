(module
    (func $power (export "power")
        (param $base i32) (param $exponent i32) (result i32)
        (local $acc i32)
        local.get $exponent
        i32.eqz
        if 
            i32.const 1
            return
        end

        ;; $accを$baseで初期化
        local.get $base
        local.set $acc

        block $block (result i32)
            loop $loop (result i32)
                local.get $acc

                ;; $exponentから1を引き、0になったらループから抜ける
                local.get $exponent
                i32.const 1
                i32.sub
                local.tee $exponent
                i32.eqz
                br_if $block

                ;; 掛け算
                local.get $base
                i32.mul
                local.set $acc
                br $loop
            end
        end
    )
    (func (export "main") (result i32)
        i32.const 2
        i32.const 3
        call $power
    )
)