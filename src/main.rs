mod common;
mod bus;
mod monitor;
mod utils;
mod machine;
mod isa;
mod device;
mod cpu;

use monitor::sdb::sdb::sdb_mainloop;
use crate::monitor::Emulator;

fn main() {
    let mut emu = Emulator::new();

    monitor::init_monitor(&mut emu);

    sdb_mainloop(&mut emu);
}
