use std::env;

use log::info;
use crate::device::ram::Ram;
use crate::monitor::Emulator;

pub fn init_mem(emu: &mut Emulator) {
    let mut mbase_str = env!("CONFIG_MBASE");
    mbase_str = mbase_str.strip_prefix("0x").unwrap();
    let mut msize_str = env!("CONFIG_MSIZE");
    msize_str = msize_str.strip_prefix("0x").unwrap();
    let main_memory_size = u64::from_str_radix(msize_str, 16).unwrap();
    let main_memory_base =  u64::from_str_radix(mbase_str, 16).unwrap();

    let name = "Physical memory".to_string();
    let ram = Ram::new(name.clone(), main_memory_size);
    emu.system.bus.map(main_memory_base, main_memory_size, Box::new(ram));
    info!("{} [ 0X{:08x}, 0X{:08x} )", name, main_memory_base, main_memory_base + main_memory_size)
}