use crate::{
    Store,
    TableType,
    TableAddr,
    FuncAddr,
    Error,

    alloc_table,
    find_tabletype,
    grow_table,
};

pub fn table_alloc(store: &mut Store, tabletype: TableType) -> TableAddr {
    alloc_table(store, tabletype)
}

pub fn table_type(store: &Store, tableaddr: TableAddr) -> TableType {
    find_tabletype(store, tableaddr).unwrap()
}

pub fn table_read(store: &Store, tableaddr: TableAddr, i: u32) -> Result<Option<FuncAddr>, Error> {
    let ti = &store.tables[tableaddr];
    if i as usize >= ti.elem.len() { return Err(Error::Invalid("table_read i as usize >= ti.elem.len()".to_owned())); }
    Ok(ti.elem[i as usize])
}

pub fn table_write(store: &mut Store, tableaddr: TableAddr, i: u32, funcaddr: Option<FuncAddr>) -> Result<(), Error> {
    let ti = &mut store.tables[tableaddr];
    if i as usize >= ti.elem.len() { return Err(Error::Invalid("table_write i as usize >= ti.elem.len()".to_owned())); }
    ti.elem[i as usize] = funcaddr;
    Ok(())
}

pub fn table_size(store: &Store, tableaddr: TableAddr) -> u32 {
    let ti = &store.tables[tableaddr];
    ti.elem.len() as u32
}

pub fn table_grow(store: &mut Store, tableaddr: TableAddr, n: u32) -> Result<(), Error> {
    let ti = &mut store.tables[tableaddr];
    grow_table(ti, n as usize)
}