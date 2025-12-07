use log::info;

use crate::monitor::args::*;
use crate::utils::logger::init_log;

fn welcome() {
    info!("Welcome to RISCV32-r2emu!");
}

pub fn init_monitor() {
    parse_args();

    init_log();

    welcome();
}
