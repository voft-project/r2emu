use crate::common::common::{paddr_t, word_t};

// TODO: 改成safe的

#[inline]
pub fn host_read(addr: *const paddr_t, len: usize) -> word_t {
    unsafe {
        match len {
            1 => *(addr as *const u8) as word_t,
            2 => *(addr as *const u16) as word_t,
            4 => *(addr as *const u32) as word_t,
            8 => *(addr as *const u64) as word_t,
            _ => panic!("invalid len: {len}"),
        }
    }
}

#[inline]
pub fn host_write(addr: *mut paddr_t, len: usize, data: word_t) {
    unsafe {
        match len {
            1 => *(addr as *mut u8) = data as u8,
            2 => *(addr as *mut u16) = data as u16,
            4 => *(addr as *mut u32) = data as u32,
            8 => *(addr as *mut u64) = data as u64,
            _ => panic!("invalid len: {len}"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::random;

    #[test]
    fn test_host_write_and_read() {
        // 必须对齐，C是UB， Rust直接panic
        let mut buf: u64 = 0;
        let p = &mut buf as *mut paddr_t;
        // B
        let data: u8 = random();
        host_write(p, 1, data as word_t);
        assert_eq!(host_read(p, 1) as u8, data);
        // H
        let data: u16 = random();
        host_write(p, 2, data as word_t);
        assert_eq!(host_read(p, 2) as u16, data);
        // W
        let data: u32 = random();
        host_write(p, 4, data as word_t);
        assert_eq!(host_read(p, 4) as u32, data);
        // DW
        let data: u64 = random();
        host_write(p, 8, data as word_t);
        assert_eq!(host_read(p, 8) as u64, data);
    }
}
