use crate::{
    Store,
    TableType,
    TableAddr,
    FuncAddr,
    Error,
};

pub fn table_alloc(_store: &mut Store, _tabletype: TableType) -> TableAddr {
    unimplemented!()
}

pub fn table_type(_store: &Store, _tableaddr: TableAddr) -> TableType {
    unimplemented!()
}

pub fn table_read(_store: &Store, _tableaddr: TableAddr, _i: u32) -> Result<Option<FuncAddr>, Error> {
    unimplemented!()
}

pub fn table_write(_store: &mut Store, _tableaddr: TableAddr, _i: u32, _funcaddr: Option<FuncAddr>) -> Result<(), Error> {
    unimplemented!()
}

pub fn table_size(_store: &Store, _tableaddr: TableAddr) -> u32 {
    unimplemented!()
}

pub fn table_grow(_store: &mut Store, _tableaddr: TableAddr, _n: u32) -> Result<(), Error> {
    unimplemented!()
}