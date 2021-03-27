use std::io::Read;

pub fn decode_vec<T: Read, R>(reader: &mut T, f: fn(reader: &mut T) -> R) -> Vec<R> {
    let length = decode_u32_from_leb128(reader);
    (0..length).map(|_| f(reader)).collect()
}

pub fn decode_u32_from_leb128(reader: &mut impl Read) -> u32 {
    let mut acc: u32 = 0;
    let mut count: u8 = 0;
    for byte in reader.bytes() {
        if let Ok(b) = byte {
            let val: u32 = (b & 0b01111111) as u32;
            let shifted_val = val << (7 * count);
            acc += shifted_val as u32;
            count += 1;
            if b < 0b10000000 { break; }
        } else {
            break;
        }
    }
    acc
}

#[test]
pub fn test_decode_u32_from_leb128() {
    use std::io::BufReader;

    // let data: [u8; 4] = [0b11001100, 0b01000011, 86, 120];
    // let data: [u8; 4] = [0xE5, 0x8E, 0x26, 120];
    let data: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    let mut reader = BufReader::new(data.as_ref());
    let res = decode_u32_from_leb128(&mut reader);
    println!("{:x?}", res);
}