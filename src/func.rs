use crate::{
    Module,
    Store,
    FuncAddr,
    Val,
    Error,
    FuncType,
    FuncInst,

    alloc_hostfunc,
};

pub fn func_alloc(store: &mut Store, functype: FuncType, hostfunc: fn()) -> FuncAddr {
    alloc_hostfunc(store, functype, hostfunc)
}

pub fn func_type(store: &Store, funcaddr: FuncAddr) -> FuncType {
    match &store.funcs[funcaddr] {
        FuncInst::User(user) => user.tp.clone(),
        FuncInst::Host(host) => host.tp.clone(),
    }
}

pub fn func_invoke<'a>(store: &'a mut Store, funcaddr: FuncAddr, vals: Vec<Val>) -> (&'a mut Store, Result<Vec<Val>, Error>) {
    Module::invoke(store, funcaddr, vals);
    unimplemented!()
}