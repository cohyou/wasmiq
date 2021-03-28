use crate::{
    Module,
    Store,
    FuncAddr,
    Val,
    Error,
};

pub fn func_alloc() {

}
pub fn func_type() {

}

pub fn func_invoke<'a>(store: &'a mut Store, funcaddr: FuncAddr, vals: Vec<Val>) -> (&'a mut Store, Result<Vec<Val>, Error>) {
    Module::invoke(store, funcaddr, vals);
    unimplemented!()
}