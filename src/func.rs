use crate::{
    Module,
    Store,
    FuncAddr,
    Val,
    Error,
    FuncType,
};

pub fn func_alloc(_store: &mut Store, _functype: FuncType, _hostfunc: fn()) -> FuncAddr {
    unimplemented!()
}

pub fn func_type(_store: &Store, _funcaddr: FuncAddr) -> FuncType {
    unimplemented!()
}

pub fn func_invoke<'a>(store: &'a mut Store, funcaddr: FuncAddr, vals: Vec<Val>) -> (&'a mut Store, Result<Vec<Val>, Error>) {
    Module::invoke(store, funcaddr, vals);
    unimplemented!()
}