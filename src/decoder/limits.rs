use std::io::Read;
use crate::{
    Limits,
    
};
use super::{
    decode_u32_from_leb128,
};

impl Limits {
    pub fn new1(min: u32) -> Limits {
        Limits { min: min, max: None }
    }
    pub fn new2(min: u32, max: u32) -> Limits {
        Limits { min: min, max: Some(max) }
    }
}

pub(super) fn decode_limits(reader: &mut impl Read) -> Limits {
    if let Some(Ok(byte)) = reader.bytes().next() {
        match byte {
            0x00 => {
                // only min
                let min_size = decode_u32_from_leb128(reader);  // min
                Limits::new1(min_size)
            },  
            0x01 => {
                // min and max
                let min_size = decode_u32_from_leb128(reader);  // min
                let max_size = decode_u32_from_leb128(reader);  // max
                Limits::new2(min_size, max_size)
            }, 
            _ => panic!("invalid on decode_limits"),
        }
    } else {
        panic!("invalid on decode_limits");
    }
}


