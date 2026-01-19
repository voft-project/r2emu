pub const fn parse_hex(s: &str) -> u32 {
    let mut val: u32 = 0;
    let mut i: usize = 0;
    let bytes = s.as_bytes();

    if bytes[0] == b'0' && (bytes[1] == b'x' || bytes[1] == b'X') {
        i = 2;
    }

    while i < bytes.len() {
        let b = bytes[i];
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => panic!("Invalid hex digit"),
        };
        val = val * 16 + digit as u32;
        i += 1;
    }

    val as u32
}