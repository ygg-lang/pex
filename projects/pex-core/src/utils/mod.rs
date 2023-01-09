#[inline(always)]
pub const fn hex_to_u8(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[inline(always)]
pub const fn hex2_to_u8(c1: u8, c2: u8) -> Option<u8> {
    let mut result = 0;
    // result += (hex_to_u8(c1)? as u32) << 4;
    // result += (hex_to_u8(c2)? as u32) << 0;
    match hex_to_u8(c1) {
        Some(v) => result += (v as u32) << 4,
        None => return None,
    }
    match hex_to_u8(c2) {
        Some(v) => result += (v as u32) << 0,
        None => return None,
    }
    Some(result as u8)
}

#[inline(always)]
pub const fn hex4_to_char(c1: u8, c2: u8, c3: u8, c4: u8) -> Option<char> {
    let mut result = 0;
    // result += (hex_to_u8(c1)? as u32) << 12;
    // result += (hex_to_u8(c2)? as u32) << 8;
    // result += (hex_to_u8(c3)? as u32) << 4;
    // result += (hex_to_u8(c4)? as u32) << 0;
    match hex_to_u8(c1) {
        Some(v) => result += (v as u32) << 12,
        None => return None,
    }
    match hex_to_u8(c2) {
        Some(v) => result += (v as u32) << 8,
        None => return None,
    }
    match hex_to_u8(c3) {
        Some(v) => result += (v as u32) << 4,
        None => return None,
    }
    match hex_to_u8(c4) {
        Some(v) => result += (v as u32) << 0,
        None => return None,
    }
    char::from_u32(result)
}
