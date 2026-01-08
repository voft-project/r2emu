#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionType {
    R,
    I,
    S,
    SB,
    U,
    UJ
}

pub enum Riscv32InstKind {
    LB,
    LH,
    LW,
    LD,
    LBU,
    LHU,
    LWU,
    FENCE,
    FENCEI,
    ADDI,
    SLLI,
    SLTI,
    SLTIU,
    XORI,
    SRLI,
    SRAI,
    ORI,
    ANDI,
    AUIPC,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,
    SB,
    SH,
    SW,
    SD,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    LUI,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    JALR,
    JAL,
    ECALL,
    EBREAK,
    // TODO CSR
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction(u32);

impl Instruction {
    pub fn new(inst: u32) -> Self {
        Self(inst)
    }

    #[inline(always)]
    pub fn inst_type(&self) -> InstructionType {
        match self.opcode() {
            0b0000011 => InstructionType::I,
            0b0001111 => InstructionType::I,
            0b0010011 => InstructionType::I,
            0b0010111 => InstructionType::U,
            0b0100011 => InstructionType::S,
            0b0110011 => InstructionType::R,
            0b0110111 => InstructionType::U,
            0b0111011 => InstructionType::R,
            0b1100011 => InstructionType::SB,
            0b1100111 => InstructionType::I,
            0b1101111 => InstructionType::UJ,
            0b1110011 => InstructionType::I,
            _ => panic!("Illegal instruction opcode: 0b{:07b} inst: (0x{:x})", self.opcode(), self.0)
        }
    }

    #[inline(always)]
    pub fn funct3(&self) -> u32 {
        self.range(14, 12)
    }

    #[inline(always)]
    pub fn funct7(&self) -> u32 {
        self.range(31, 25)
    }

    #[inline(always)]
    pub fn opcode(&self) -> u32 {
        self.range(6, 0)
    }

    #[inline(always)]
    pub fn rd(&self) -> u32 {
        self.range(11, 7)
    }

    #[inline(always)]
    pub fn rs1(&self) -> u32 {
        self.range(19, 15)
    }

    #[inline(always)]
    pub fn rs2(&self) -> u32 {
        self.range(24, 20)
    }

    #[inline(always)]
    pub fn imm(&self) -> i32 {
        match self.inst_type() {
            InstructionType::R => panic!("R-Type no Imm"),
            InstructionType::I => self.sext(self.range(31, 20), 12),
            InstructionType::S => {
                let u_val = self.funct7() << 5 | self.rd();
                self.sext(u_val, 12)
            },
            InstructionType::SB => {
                let bit_12 = self.range(31, 31);
                let bit_11 = self.range(7, 7);
                let bit_10_5 = self.range(30, 25);
                let bit_4_1 = self.range(4, 1);
                let u_val = (bit_12 << 12) | (bit_11 << 1) | (bit_10_5 << 5) | (bit_4_1 << 1);
                self.sext(u_val, 13)
            },
            InstructionType::U => {
                let u_val = self.range(31, 12);
                self.sext(u_val, 31 - 12 + 1)
            },
            InstructionType::UJ => {
                let bit_20 = self.range(31, 31);
                let bit_19_12 = self.range(19, 12);
                let bit_11 = self.range(20, 20);
                let bit_10_1 = self.range(30, 21);
                let u_val = (bit_20 << 20) | (bit_19_12 << 12) | (bit_11 << 11) | (bit_10_1 << 1);
                self.sext(u_val, 20)
            }
        }
    }

    #[inline(always)]
    pub fn range(&self, end: u32, start: u32) -> u32 {
        let len = end - start + 1;
        let mask = (1 << len) - 1;
        (self.0 >> start) & mask
    }

    #[inline(always)]
    pub fn sext(&self, val: u32, len: u32) -> i32 {
        let shift = 32 - len;
        ((val as i32) << shift) >> shift
    }

    pub fn inst_decode(&self) -> Riscv32InstKind {
        match self.opcode() {
            0b0110111 => Riscv32InstKind::LUI,
            0b0010111 => Riscv32InstKind::AUIPC,
            0b1101111 => Riscv32InstKind::JAL,
            0b1100111 => {
                match self.funct3() {
                    0b000 => Riscv32InstKind::BEQ,
                    0b001 => Riscv32InstKind::BNE,
                    0b100 => Riscv32InstKind::BLT,
                    0b101 => Riscv32InstKind::BGE,
                    0b110 => Riscv32InstKind::BLTU,
                    0b111 => Riscv32InstKind::BGEU,
                    _  => panic!("Invalid Inst {:x}", self.0)
                }
            }
            0b0000011 => {
                match self.funct3() {
                    0b000 => Riscv32InstKind::LB,
                    0b001 => Riscv32InstKind::LH,
                    0b010 => Riscv32InstKind::LW,
                    0b100 => Riscv32InstKind::LBU,
                    0b101 => Riscv32InstKind::LHU,
                    _ => panic!("Invalid Inst {:x}", self.0)
                }
            }
            0b0100011 => {
                match self.funct3() {
                    0b000 => Riscv32InstKind::SB,
                    0b001 => Riscv32InstKind::SH,
                    0b010 => Riscv32InstKind::SW,
                    _ => panic!("Invalid Inst {:x}", self.0)
                }
            }
            0b0010011 => {
                match self.funct3() {
                    0b000 => Riscv32InstKind::ADDI,
                    0b010 => Riscv32InstKind::SLTI,
                    0b011 => Riscv32InstKind::SLTIU,
                    0b100 => Riscv32InstKind::XORI,
                    0b110 => Riscv32InstKind::ORI,
                    0b111 => Riscv32InstKind::ANDI,
                    0b001 => {
                        match self.funct7() {
                            0b0000000 => Riscv32InstKind::SLLI,
                            _ => panic!("Invalid Inst {:x}", self.0)
                        }
                    }
                    0b101 => {
                        match self.funct7() {
                            0b0000000 => Riscv32InstKind::SRLI,
                            0b0100000 => Riscv32InstKind::SRAI,
                            _ => panic!("Invalid Inst {:x}", self.0)
                        }
                    }
                    _ => panic!("Invalid Inst {:x}", self.0)
                }
            }
            0b0110011 => {
                match self.funct3() {
                    0b000 => {
                        match self.funct7() {
                            0b0000000 => Riscv32InstKind::ADD,
                            0b0100000 => Riscv32InstKind::SUB,
                            _ => panic!("Invalid Inst {:x}", self.0)
                        }
                    }
                    0b001 => Riscv32InstKind::SLL,
                    0b010 => Riscv32InstKind::SLT,
                    0b011 => Riscv32InstKind::SLTU,
                    0b100 => Riscv32InstKind::XOR,
                    0b101 => {
                        match self.funct7() {
                            0b0000000 => Riscv32InstKind::SRL,
                            0b0100000 => Riscv32InstKind::SRA,
                            _ => panic!("Invalid Inst {:x}", self.0)
                        }
                    }
                    0b110 => Riscv32InstKind::OR,
                    0b111 => Riscv32InstKind::AND,
                    // 实际上走完了
                    _ => panic!("Invalid Inst {:x}", self.0)
                }
            }
            0b0001111 => {
                match self.funct3() {
                    0b000 => Riscv32InstKind::FENCE,
                    0b001 => Riscv32InstKind::FENCEI,
                    _ => panic!("Invalid Inst {:x}", self.0)
                }
            }
            0b1110011 => {
                match self.range(31, 20) {
                    0 => Riscv32InstKind::ECALL,
                    1 => Riscv32InstKind::EBREAK,
                    _ => panic!("Invalid Inst {:x}", self.0)
                }
            }
            // CSR Inst
            _ => panic!("Invalid Inst {:x}", self.0)
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_inst_type() {
        let inst1 = Instruction::new(0x00000003);
        assert_eq!(inst1.inst_type(), InstructionType::I);
        let inst2 = Instruction::new(0x000002b7);
        assert_eq!(inst2.inst_type(), InstructionType::U);
        let inst3 = Instruction::new(0x00000023);
        assert_eq!(inst3.inst_type(), InstructionType::S);
        let inst4 = Instruction::new(0x00000063);
        assert_eq!(inst4.inst_type(), InstructionType::SB);
        let inst5 = Instruction::new(0x00005033);
        assert_eq!(inst5.inst_type(), InstructionType::R);
        let inst6 = Instruction::new(0x0000026f);
        assert_eq!(inst6.inst_type(), InstructionType::UJ);
    }
} 