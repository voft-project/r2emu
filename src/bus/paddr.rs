use log::info;
use crate::device::ram::Ram;
use crate::monitor::Emulator;

pub fn init_mem(emu: &mut Emulator) {
    let main_memory_size = 1024 * 1024;
    let main_memory_base = 0x8000000;
    let name = "Physical memory".to_string();
    let ram = Ram::new(name.clone(), main_memory_size);
    emu.system.bus.map(main_memory_base, main_memory_size, Box::new(ram));
    info!("{} [ 0X{:08x}, 0X{:08x} )", name, main_memory_base, main_memory_base + main_memory_size)
}