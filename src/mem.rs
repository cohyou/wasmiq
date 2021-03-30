use crate::{
    Store,
    MemType,
    MemAddr,
    Byte,
    Error,
};

pub fn mem_alloc(_store: &mut Store, _memtype: MemType) -> MemAddr {
    unimplemented!()
}

pub fn mem_type(_store: &Store, _memaddr: MemAddr) -> MemType {
    unimplemented!()
}

pub fn mem_read(_store: &Store, _memaddr: MemAddr, _i: u32) -> Result<Byte, Error> {
    unimplemented!()
}

pub fn mem_write(_store: &mut Store, _memaddr: MemAddr, _i: u32, _byte: Byte) -> Result<(), Error> {
    unimplemented!()
}

pub fn mem_size(_store: &Store, _memaddr: MemAddr) {
    unimplemented!()
}

pub fn mem_grow(_store: &mut Store, _memaddr: MemAddr, _n: u32) -> Result<(), Error> {
    unimplemented!()
}