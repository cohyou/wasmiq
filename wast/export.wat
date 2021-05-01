(module
    (func $export_func (result i32)
        i32.const 1
    )

    (table $export_table 1 funcref)
    (elem (i32.const 0) $internal_func)
    (func $internal_func (result i32)
        i32.const 2
    )

    (memory $export_memory 1)

    (global $export_global i32 (i32.const 2))

    (export "func" (func $export_func))
    (export "table" (table $export_table))
    (export "memory" (memory $export_memory))
    (export "global" (global $export_global))
)