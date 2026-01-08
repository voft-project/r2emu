use crate::{device::Device, isa::riscv32::{RESET_VECTOR, img::DEFAULT_RISCV32_IMG}, monitor::Emulator};

pub mod riscv32;

pub trait Isa {
    fn exec(&mut self, inst: u32);
    fn exec_continue(&mut self);
}

pub fn init_isa(emu: &mut Emulator) {
    for i in 0..DEFAULT_RISCV32_IMG.len() {
        // TODO bus 改为xlen
        let _ = emu.system.bus.write((RESET_VECTOR + (i as u32) * 4) as u64, 4, DEFAULT_RISCV32_IMG[i] as u64);
    }

    emu.system.restart();
}