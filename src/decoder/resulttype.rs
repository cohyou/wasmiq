use std::io::Read;
use crate::{
    ValType,
};

use super::valtype::{decode_valtype};
use super::decode_vec;

pub(super) fn decode_resulttype(reader: &mut impl Read) -> Vec<ValType> {
    decode_vec(reader, decode_valtype)
}

#[test]
fn test_decode_resulttype() {
    use std::io::BufReader;

    let data: [u8; 6] = [
        4,
        0x7D, 0x7C, 0x7F, 0x7E, 0x7D,
    ];
    let mut reader = BufReader::new(data.as_ref());
    let correct = vec![ValType::F32, ValType::F64, ValType::I32, ValType::I64];
    assert_eq!(decode_resulttype(&mut reader), correct);
}