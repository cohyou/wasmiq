use crate::{
    Store,
    MemType,
    MemAddr,
    Byte,
    Error,

    alloc_mem,
    find_memtype,
    grow_mem,
};

pub fn mem_alloc(store: &mut Store, memtype: MemType) -> MemAddr {
    alloc_mem(store, memtype)
}

pub fn mem_type(store: &Store, memaddr: MemAddr) -> MemType {
    find_memtype(store, memaddr).unwrap()
}

pub fn mem_read(store: &Store, memaddr: MemAddr, i: u32) -> Result<Byte, Error> {
    let mi = &store.mems[memaddr];
    if i as usize >= mi.data.len() { return Err(Error::Invalid); }
    Ok(mi.data[i as usize])
}

pub fn mem_write(store: &mut Store, memaddr: MemAddr, i: u32, byte: Byte) -> Result<(), Error> {
    let mi = &mut store.mems[memaddr];
    if i as usize >= mi.data.len() { return Err(Error::Invalid); }
    mi.data[i as usize] = byte;
    Ok(())
}

pub fn mem_size(store: &Store, memaddr: MemAddr) -> u32 {
    let mi = &store.mems[memaddr];
    mi.data.len() as u32
}

pub fn mem_grow(store: &mut Store, memaddr: MemAddr, n: u32) -> Result<(), Error> {
    let mi = &mut store.mems[memaddr];
    grow_mem(mi, n as usize)
}