pub mod ram;

pub trait Device {
    fn out_of_bound(&self, addr: u64, size: u8) -> bool;
    fn read(&self, addr: u64, size: u8) -> Result<u64, ()>;
    fn write(&mut self, addr: u64, size: u8, data: u64) -> Result<(), ()>;
}