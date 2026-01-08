use crate::common::common::vaddr_t;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    todo!()
}
