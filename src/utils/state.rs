use crate::common::common::vaddr_t;
use std::sync::{LazyLock, Mutex};

pub static R2EMU_STATE: LazyLock<Mutex<R2emuState>> =
    LazyLock::new(|| Mutex::new(R2emuState::new()));

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum R2emuStateEnum {
    Running,
    Stop,
    End,
    Abort,
    Quit,
}

#[derive(Debug)]
pub struct R2emuState {
    state: R2emuStateEnum,
    halt_ret: u32,
    halt_pc: vaddr_t,
}

impl R2emuState {
    pub fn new() -> Self {
        Self {
            state: R2emuStateEnum::Stop,
            halt_ret: 0,
            halt_pc: 0,
        }
    }

    pub fn get_mut_ref() -> std::sync::MutexGuard<'static, Self> {
        R2EMU_STATE.lock().unwrap()
    }

    pub fn set_state(&mut self, new_state: R2emuStateEnum) {
        self.state = new_state;
    }

    pub fn get_state(&self) -> R2emuStateEnum {
        self.state
    }

    pub fn set_halt_ret(&mut self, new_halt_ret: u32) {
        self.halt_ret = new_halt_ret
    }

    pub fn get_halt_ret(&self) -> u32 {
        self.halt_ret
    }

    pub fn set_halt_pc(&mut self, new_halt_pc: vaddr_t) {
        self.halt_pc = new_halt_pc;
    }

    pub fn get_halt_pc(&self) -> vaddr_t {
        self.halt_pc
    }
}

pub fn is_exist_status_bad() -> bool {
    // 优化: 用RWLock优化
    let global_state = R2emuState::get_mut_ref();
    let current_state = global_state.get_state();
    let current_halt_ret = global_state.get_halt_ret();
    let good = (current_state == R2emuStateEnum::End && current_halt_ret == 0)
        || (current_state == R2emuStateEnum::Quit);

    !good
}
