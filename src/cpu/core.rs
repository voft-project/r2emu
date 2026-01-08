use log::info;

use crate::{bus::Bus, device::Device, isa::riscv32::{RESET_VECTOR, arch::Riscv32Isa, instruction::{Instruction, InstructionType, Riscv32InstKind}}, monitor::state::R2emuStateEnum, utils::disasm::disasm};

pub struct CpuCore {
    pub core_id: u32,
    pub isa: Riscv32Isa,
}

impl CpuCore {
    pub fn new(core_id: u32) -> Self {
        Self {
            core_id: core_id,
            isa: Riscv32Isa::new(),
        }
    }

    fn fetch(&self, bus: &mut Bus) -> u32 {
        let pc = self.isa.get_pc();
        let inst = match bus.read(pc as u64, 4) {
            Ok(i) => i,
            Err(_) => panic!("Failed to load inst at {pc}")
        };
        inst as u32
    }

    pub fn step(&mut self, bus: &mut Bus) -> R2emuStateEnum {
        let inst = self.fetch(bus);
        let pc = self.isa.get_pc();
        let disasm_str = disasm(inst as u32, pc);
        let inst = Instruction::new(inst as u32);
        let res = self.execute(inst, bus);
        println!("{disasm_str}");
        self.isa.set_pc(pc + 4);
        res
    }

    pub fn restart(&mut self) {
        self.isa.set_pc(RESET_VECTOR);
    }

    fn execute(&mut self, inst: Instruction, _bus: &mut Bus) -> R2emuStateEnum {
        match inst.inst_decode() {
            Riscv32InstKind::ADDI => {
                let rd = inst.rd();
                let rs1 = inst.rs1();
                let imm = inst.imm();
                let val_rs1 = self.isa.get_gpr(rs1);
                self.isa.set_gpr(rd, (val_rs1 as i32 + imm) as u32);
                R2emuStateEnum::Running
            }
            Riscv32InstKind::EBREAK => {
                R2emuStateEnum::End
            }
            _ => unimplemented!()
        }
    }
}