use std::io::Read;
use super::decode_u32_from_leb128;

pub(super) fn decode_customsec(reader: &mut impl Read) {
    // prefixはsection number 0
    let length = decode_u32_from_leb128(reader);
    let mut bytes = reader.bytes();
    let _ = (0..length).map(|_| bytes.next());
}