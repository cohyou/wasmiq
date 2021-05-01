(module
    ;; (import "myModule" "memory" (memory 1))
    (memory 1)

    (func (export "toUpper") (param $offset i32) (param $length i32)
        (local $address i32) (local $char i32)
        block $b
            loop $l
                ;; $lengthを-1する
                local.get $length
                i32.const 1
                i32.sub
                local.tee $length

                i32.const 0
                i32.lt_s
                br_if $b ;; $lengthが0未満になったら処理を終了

                local.get $offset
                local.get $length
                i32.add
                ;; 変換するメモリ位置($offset+$length)を$addressに代入
                local.tee $address
                i32.load8_u
                local.tee $char ;; 変換対象の文字を$charに代入

                ;; a以上z以下の判定
                i32.const 97 ;; 'a'
                i32.ge_u
                local.get $char
                i32.const 122 ;; 'z'
                i32.le_u
                i32.and

                if ;; 32を引いた文字を、$address位置に保存
                    local.get $address
                    local.get $char
                    i32.const 32
                    i32.sub
                    i32.store8
                end
                br $l
            end
        end
    )
)