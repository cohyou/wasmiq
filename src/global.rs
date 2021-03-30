use crate::{
    Store,
    GlobalType,
    Val,
    GlobalAddr,
    Error,
};

pub fn global_alloc(_store: &mut Store, _globaltype: GlobalType, _val: Val) -> GlobalAddr {
    unimplemented!()
}

pub fn global_type(_store: &Store, _globaladdr: GlobalAddr) -> GlobalType {
    unimplemented!()
}

pub fn global_read(_store: &Store, _globaladdr: GlobalAddr) -> Val {
    unimplemented!()
}

pub fn global_write(_store: &mut Store, _globaladdr: GlobalAddr, _val: Val) -> Result<(), Error> {
    unimplemented!()
}
