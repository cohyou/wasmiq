struct Store {
    funcs: Vec<FuncInst>,
    tables: Vec<TableInst>,
    mems: Vec<MemInst>,
    globals: Vec<GlobalInst>,
}

struct FuncInst {
    tp: FuncType,
    module: ModuleInst,
    code: Func,
}

struct ModuleInst {
    funcaddrs: Weak<Vec<Funcinst>>,
    tableaddrs: Weak<Vec<Funcinst>>,
    memaddrs: Weak<Vec<Funcinst>>,
    globaladdrs: Weak<Vec<Funcinst>>,
}