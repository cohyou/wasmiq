(module
    (memory 1)

    (func $write_memo (param $index i32) (param $value i32)
        local.get $index
        i32.const 4
        i32.mul
        local.get $value
        i32.store
    )
    (func $read_memo (param $index i32) (result i32)
        local.get $index
        i32.const 4
        i32.mul
        i32.load
    )

    (func $fibonacci_memo (export "fibonacci_memo")
        (param $n i32) (result i32)
        (local $result i32)
        local.get $n
        i32.const 1
        i32.le_u
        if
            local.get $n
            return
        end

        ;; メモ化した計算結果の読み込み
        local.get $n
        call $read_memo
        local.tee $result

        ;; 結果がまだ書き込まれていないなら、実際に計算する
        i32.eqz
        if
            local.get $n

            ;; fibonacci($n-1)を呼び出す
            local.get $n
            i32.const 1
            i32.sub
            call $fibonacci_memo

            ;; fibonacci($n-2)を呼び出す
            local.get $n
            i32.const 2
            i32.sub
            call $fibonacci_memo

            ;; fibonacci($n-1) + fibonacci($n-2)を計算し、結果をメモ化する
            i32.add
            local.tee $result
            call $write_memo
        end

        local.get $result
    )

    (func (export "main") (result i32)
        i32.const 30
        call $fibonacci_memo
    )
)