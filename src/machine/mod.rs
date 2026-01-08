use log::trace;

use crate::bus::Bus;
use crate::cpu::core::CpuCore;
use crate::monitor::state::{R2emuState, R2emuStateEnum};

pub struct System {
    pub cpu: CpuCore,
    pub bus: Bus,
}

impl System {
    pub fn new() -> Self {
        Self {
            cpu: CpuCore::new(0),
            bus: Bus::new(),
        }
    }

    pub fn step(&mut self, steps: u64) -> R2emuStateEnum {
        trace!("Step!");
        for _ in 0..steps {
            let state = self.cpu.step(&mut self.bus);
            if state != R2emuStateEnum::Running {
                return state;
            }
        }
        R2emuStateEnum::Running
    }

    pub fn restart(&mut self) {
        self.cpu.restart();
    }
}