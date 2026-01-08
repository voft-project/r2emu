pub mod args;
pub mod sdb;
pub mod state;

use core::panic;

use log::info;
use crate::isa::init_isa;
use crate::monitor::args::*;
use crate::monitor::state::R2emuStateEnum;
use crate::utils::logger::init_log;
use crate::bus::paddr::init_mem;
use sdb::sdb::init_sdb;
use state::R2emuState;
use crate::machine::System;
use std::{time::{Duration, Instant}, u64};

struct EmulatorStatistic {
    pub g_time: u128
}

impl EmulatorStatistic {
    pub fn new() -> Self {
        Self {
            g_time: 0
        }
    }

    pub fn update_g_time(&mut self, cost: Duration) {
        self.g_time += cost.as_micros();
    }
}

pub struct Emulator {
    pub system: System,
    pub emu_state: R2emuState,
    pub statistic: EmulatorStatistic
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            system: System::new(),
            emu_state: R2emuState::new(),
            statistic: EmulatorStatistic::new()
        }
    }

    // 重构
    pub fn exec(&mut self, steps: u64) {
        let init_state = self.emu_state.get_state();
        if init_state == R2emuStateEnum::Abort ||
            init_state == R2emuStateEnum::End  ||
            init_state == R2emuStateEnum::Quit {
                println!("Program execution has ended.");
                return;
            }
        
        self.emu_state.set_state(R2emuStateEnum::Running);
        let start_time = Instant::now();
        let res = self.system.step(steps);
        let end_time = start_time.elapsed();
        self.statistic.update_g_time(end_time);
        if res != R2emuStateEnum::Running {
            self.emu_state.set_state(res);
            let halt_pc = self.system.cpu.isa.get_pc();
            let a0 = self.system.cpu.isa.get_gpr(10);
            self.emu_state.set_halt_pc(halt_pc as u64);
            self.emu_state.set_halt_ret(a0);
            self.emu_state.set_state(res);
        }

        // 检查R2EMU状态
        match self.emu_state.get_state() {
            R2emuStateEnum::Running => {
                self.emu_state.set_state(R2emuStateEnum::Stop);
            },
            R2emuStateEnum::Abort | R2emuStateEnum::End => {
                let state = self.emu_state.get_state();
                let pc = self.emu_state.get_halt_pc();
                let a0 = self.emu_state.get_halt_ret();

                let msg = if state == R2emuStateEnum::Abort {
                    "\x1b[1;31mABORT\x1b[0m"
                } else if a0 == 0 {
                    "\x1b[1;32mHIT GOOD TRAP\x1b[0m"
                } else {
                    "\x1b[1;31mHIT BAD TRAP\x1b[0m"
                };

                println!("r2emu: {} at pc = 0x{:08x}", msg, pc);

                self.dump_statistic();
            }, 
            R2emuStateEnum::Quit => {
                self.dump_statistic();
            },
            _ => panic!("R2Emu Never reach here!")
        };
    }

    pub fn exec_continue(&mut self) {
        self.exec(u64::MAX);
    }

    fn dump_statistic(&self) {
        println!("host time spent {} us", self.statistic.g_time);
    }
}

fn welcome() {
    info!("Welcome to RISCV32-r2emu!");
}

pub fn init_monitor(emu: &mut Emulator) {
    parse_args();

    let _ = init_log();

    init_mem(emu);

    init_isa(emu);

    init_sdb();

    welcome();
}