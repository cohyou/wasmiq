use crate::{
    Store,
    GlobalType,
    Val,
    GlobalAddr,
    Error,
    Mut,

    alloc_global,
    find_globaltype,
};

pub fn global_alloc(store: &mut Store, globaltype: GlobalType, val: Val) -> GlobalAddr {
    alloc_global(store, globaltype, val)
}

pub fn global_type(store: &Store, globaladdr: GlobalAddr) -> GlobalType {
    find_globaltype(store, globaladdr).unwrap()
}

pub fn global_read(store: &Store, globaladdr: GlobalAddr) -> Val {
    let gi = &store.globals[globaladdr];
    gi.value
}

pub fn global_write(store: &mut Store, globaladdr: GlobalAddr, val: Val) -> Result<(), Error> {
    let gi = &mut store.globals[globaladdr];
    if gi.mutability != Mut::Var { return Err(Error::Invalid("global_write gi.mutability != Mut::Var".to_owned())); }
    gi.value = val;
    Ok(())
}
