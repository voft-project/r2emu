pub mod paddr;

use crate::device::Device;
use std::boxed::Box;

struct MapEntry {
    base: u64,
    size: u64,
    device: Box<dyn Device>
}

pub struct Bus {
    devices: Vec<MapEntry>
}

impl Bus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new()
        }
    }

    pub fn map(&mut self, base: u64, size: u64, device: Box<dyn Device>) -> bool {
        if self.check_overlap(base, size) {
            return false;
        }
        self.devices.push(MapEntry {
            base,
            size,
            device
        });
        true
    }

    pub fn check_overlap(&self, base: u64, size: u64) -> bool {
        for entry in &self.devices {
            if base < entry.base + entry.size && base + size > entry.base {
                return true;
            }
        }
        false
    }
}

impl Device for Bus {
    fn read(&self, addr: u64, size: u8) -> Result<u64, ()> {
        for entry in &self.devices {
            if addr >= entry.base && addr < entry.base + entry.size {
                return entry.device.read(addr - entry.base, size);
            }
        }
        Ok(0)
    }

    fn write(&mut self, addr: u64, size: u8, data: u64) -> Result<(), ()> {
        for entry in &mut self.devices {
            if addr >= entry.base && addr < entry.base + entry.size {
                entry.device.write(addr - entry.base, size, data)?
            }
        }
        Ok(())
    }
    
    fn out_of_bound(&self, _addr: u64, _size: u8) -> bool {
        true
    }
}