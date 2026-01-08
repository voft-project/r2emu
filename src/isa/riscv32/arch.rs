pub struct Riscv32RegFile {
    gpr: [u32; 32],
}

impl Riscv32RegFile {
    pub fn new() -> Self {
        Self {
            gpr: [0; 32]
        }
    }

    pub fn read(&self, idx: u32) -> u32 {
        if idx == 0 {
            return 0;
        }
        self.gpr[idx as usize]
    }

    pub fn write(&mut self, idx: u32, val: u32) {
        if idx == 0 {
            return;
        }
        self.gpr[idx as usize] = val;
    }

    pub fn dump_regs(&self) {
        for (i, reg) in self.gpr.iter().enumerate() {
            println!("x{}: 0x{:x}", i, reg);
        }
    }
}

pub struct Riscv32Isa {
    gpr: Riscv32RegFile,
    pc: u32
}

impl Riscv32Isa {
    pub fn new() -> Self {
        Self {
            gpr: Riscv32RegFile::new(),
            pc: 0
        }
    }

    pub fn decode(&mut self, inst: u32) -> (u32, u32, u32, u32) {
        unimplemented!()
    }

    pub fn exec(&mut self, inst: u32) {
        unimplemented!()
    }

    pub fn dump_regs(&self) {
        self.gpr.dump_regs();
    }

    pub fn dump_pc(&self) {
        println!("pc: 0x{:x}", self.pc);
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn get_gpr(&self, idx: u32) -> u32 {
        self.gpr.read(idx)
    }

    pub fn set_gpr(&mut self, idx: u32, val: u32) {
        self.gpr.write(idx, val);
    }
}