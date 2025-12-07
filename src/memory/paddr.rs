// TODO: 改成unsafe

use std::ptr::addr_of_mut;

use crate::common::common::{paddr_t, word_t};
use crate::memory::host::{host_read, host_write};

pub const CONFIG_MSIZE: u64 = 0x800000;
pub const CONFIG_MBASE: paddr_t = 0x800000;

pub static mut PMEM: [u8; CONFIG_MSIZE as usize] = [0; CONFIG_MSIZE as usize];

pub fn guest_to_host(guest_paddr: paddr_t) -> *mut u8 {
    let offset = (guest_paddr - CONFIG_MBASE) as usize;
    unsafe { addr_of_mut!(PMEM).cast::<u8>().add(offset) }
}

pub fn host_to_guest(host_addr: *mut u8) -> paddr_t {
    unsafe {
        let pmem_base = addr_of_mut!(PMEM).cast::<u8>();
        let offset = host_addr.offset_from(pmem_base) as u64;
        (CONFIG_MBASE + offset) as paddr_t
    }
}

fn out_of_bound(addr: paddr_t) {
    panic!("address = {addr} is out of bound of pmem");
}

fn pmem_read(addr: paddr_t, len: usize) -> word_t {
    host_read(guest_to_host(addr) as *const paddr_t, len)
}

fn pmem_write(addr: paddr_t, len: usize, data: word_t) {
    host_write(guest_to_host(addr) as *mut paddr_t, len, data);
}

fn paddr_read(addr: paddr_t, len: usize) -> word_t {
    if in_pmem(addr) {
        return pmem_read(addr, len);
    }
    // TODO: MMIO
    out_of_bound(addr);
    return 0;
}

fn paddr_write(addr: paddr_t, len: usize, data: word_t) {
    if in_pmem(addr) {
        paddr_write(addr, len, data);
    }
    // TODO: MMIO
    out_of_bound(addr);
}

#[inline]
fn in_pmem(addr: paddr_t) -> bool {
    // 模拟C语言溢出
    addr.wrapping_sub(CONFIG_MBASE) < CONFIG_MSIZE
}

pub fn init_mem() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_pmem() {
        let addr: paddr_t = CONFIG_MBASE - 1;
        assert_eq!(in_pmem(addr), false);
        let addr: paddr_t = CONFIG_MBASE;
        assert_eq!(in_pmem(addr), true);
        let addr: paddr_t = CONFIG_MBASE + CONFIG_MSIZE - 1;
        assert_eq!(in_pmem(addr), true);
        let addr: paddr_t = CONFIG_MBASE + CONFIG_MSIZE;
        assert_eq!(in_pmem(addr), false);
    }
}
