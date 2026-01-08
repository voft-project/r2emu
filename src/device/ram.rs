use std::vec::Vec;
use crate::device::Device;

pub struct Ram {
    pub name: String,
    pub size: u64,
    pub memory: Vec<u8>
}

impl Ram {
    pub fn new(name: String, size: u64) -> Self {
        Ram {
            name: name,
            size: size,
            memory: vec![0u8; size as usize]
        }
    }
}

impl Device for Ram {
    fn read(&self, addr: u64, size: u8) -> Result<u64, ()> {
        if self.out_of_bound(addr, size) {
            return Err(());
        }

        let index = addr as usize;
        let res = match size {
            1 => self.memory[index] as u64,
            2 => {
                let low = self.memory[index] as u64;
                let high = self.memory[index + 1] as u64;
                (high << 8 | low) as u64
            }
            4 => {
                let l1 = self.memory[index + 0] as u64;
                let l2 = self.memory[index + 1] as u64;
                let l3 = self.memory[index + 2] as u64;
                let l4 = self.memory[index + 3] as u64;
                (l4 << 24 | l3 << 16 | l2 << 8 | l1) as u64
            }
            _ => {
                return Err(());
            }
        };

        Ok(res)
    }

    fn write(&mut self, addr: u64, size: u8, data: u64) -> Result<(), ()> {
        if self.out_of_bound(addr, size) {
            return Err(());
        }

        let index = addr as usize;
        match size {
            1 => {
                self.memory[index] = data as u8;
                Ok(())
            }
            2 => {
                let low = data as u8;
                let hight = (data >> 8) as u8;
                self.memory[index + 0] = low;
                self.memory[index + 1] = hight;
                Ok(())
            }
            4 => {
                let l1 = (data >> 0) as u8;
                let l2 = (data >> 8) as u8;
                let l3 = (data >> 16) as u8;
                let l4 = (data >> 24) as u8;
                self.memory[index + 0] = l1;
                self.memory[index + 1] = l2;
                self.memory[index + 2] = l3;
                self.memory[index + 3] = l4;
                Ok(())
            }
            _ => {
                Err(())
            }
        }
    }
    
    fn out_of_bound(&self, addr: u64, size: u8) -> bool {
        addr >= self.size || (addr + size as u64) >= self.size
    }
}