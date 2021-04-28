(module
    (func $power (export "power")
        (param $base i32) (param $exponent i32) ;; (result i32)
        (local $acc i32)
        local.get $exponent
        i32.eqz
        if
            i32.const 1
            return
        end

        i32.const 1
        local.set $acc

        ;; loop $label (result i32)
        ;;     local.get $base
        ;;     local.get $acc
        ;;     i32.mul
        ;;     local.tee $acc

        ;;     local.get $exponent
        ;;     i32.const 1
        ;;     i32.sub
        ;;     local.tee $exponent
        ;;     i32.const 0
        ;;     i32.ne
        ;;     br_if $label
        ;; end
    )

    (func (export "main") ;; (result i32)
        i32.const 2
        i32.const 3
        call $power
    )
)