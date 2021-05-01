(module
    (import "myModule" "plusOne" (func $plusOne (param i32) (result i32)))
    (import "myModule" "table" (table 1 funcref))
    (import "myModule" "memory" (memory 1))
    (import "myModule" "global" (global $import_global (mut i32)))

    (func (export "main") (result i32)
        global.get $import_global
        call $plusOne
    )
)