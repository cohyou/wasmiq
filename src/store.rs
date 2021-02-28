/// Return the enpty store.
pub fn store_init() -> Store {
    Store::default()
}

#[derive(Default)]
pub struct Store {
    funcs: Vec<FuncInst>,
    tables: Vec<TableInst>,
    mems: Vec<MemInst>,
    globals: Vec<GlobalInst>,
}

// struct FuncInst {
//     tp: FuncType,
//     module: ModuleInst,
//     code: Func,
// }

struct FuncInst;
struct TableInst;
struct MemInst;
struct GlobalInst;

#[cfg(test)]
mod tests {
    use super::store_init;

    #[test]
    fn test_store_init() {
        let s = store_init();
        assert_eq!(s.funcs.len(), 0);
        assert_eq!(s.mems.len(), 0);
        assert_eq!(s.tables.len(), 0);
        assert_eq!(s.globals.len(), 0);
    }
}
