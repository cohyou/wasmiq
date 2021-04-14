# FUNC
(func id? typeuse local* instr*)

(func)
(func (type typeidx))
(func (param valtype))
(func (param id valtype))
(func (param valtype valtype))
(func (param id? valtype) (param id? valtype))  // 本当はここだけで4種類
(func (result valtype))
(func (local valtype))
(func (local id valtype))
(func (local valtype valtype))
(func (local id? valtype) (local id? valtype))  // 本当はここだけで4種類
typeuse...20
local...5
instr...3だが別途考えるべき

(func (import "n1" "n2") typeuse)
(func (export "n1") ...)

(func id)
(func id (type typeidx))
(func id (param valtype))
(func id (param id valtype))
(func id (param valtype valtype))
(func id (param id? valtype) (param id? valtype))  // 本当はここだけで4種類
(func id (result valtype))
(func id (local valtype))
(func id (local id valtype))
(func id (local valtype valtype))
(func id (local id? valtype) (local id? valtype))  // 本当はここだけで4種類

(func id (import "n1" "n2") typeuse)
(func id (export "n") ...)

つまり、通常のfuncで600種類のパターンが存在する。


# TABLE
(table id? tabletype)

(table u32 funcref)
(table u32 u32 funcref)
(table (import "n1" "n2") tabletype)
(table (export "n") ...)
(table id u32 funcref)
(table id u32 u32 funcref)
(table id (import "n1" "n2") tabletype)
(table id (export "n") ...)

id...2
tabletype...2
4種類。


# MEMORY
(memory id? memtype)

(memory u32)
(memory u32 u32)
(memory (import "n1" "n2") memtype)
(memory (export "n") ...)
(memory id u32)
(memory id u32 u32)
(memory id (import "n1" "n2") memtype)
(memory id (export "n") ...)

id...2
memtype...2
4種類。


# GLOBAL
(global id? globaltype expr)

(global valtype)
(global (mut valtype))
(global valtype instr)
(global (mut valtype) instr)
(global valtype instr instr)
(global (mut valtype) instr instr)
(global (import "n1" "n2") globaltype)
(global (export "n") ...)

(global id valtype)
(global id (mut valtype))
(global id valtype instr)
(global id (mut valtype) instr)
(global id valtype instr instr)
(global id (mut valtype) instr instr)
(global id (import "n1" "n2") globaltype)
(global id (export "n") ...)

id...2
globaltype...2
expr...3
12種類。