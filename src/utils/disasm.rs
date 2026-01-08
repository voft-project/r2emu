use capstone::prelude::*;
use std::cell::RefCell;
 
thread_local! {
    static DISASM: RefCell<Capstone> = RefCell::new(
        Capstone::new()
            .riscv()
            .mode(capstone::arch::riscv::ArchMode::RiscV32)
            .build()
            .expect("Failed to create Capstone object")
    );
}

struct SyncCapstone(Capstone);

// todo: pc改为xlen
pub fn disasm(inst: u32, pc: u32) -> String {
    DISASM.with(|cs_cell| {
        // Riscv小端
        // 实际上不传入也行
        let cs = cs_cell.borrow_mut();
        let bytes = inst.to_le_bytes();
        let cs_inst = cs.disasm_all(&bytes, pc as u64).expect("Failed to disasm inst {inst} {pc}");

        // 确保一次传入一条指令
        if let Some(i) = cs_inst.iter().next() {
            return i.to_string()
        } else {
            panic!("Invalid Inst {inst}!");
        }
    })
}


mod tests {
    use super::*;

    #[test]
    fn test_disasm() {
        let res = disasm(0x00100073, 0);
        assert_eq!(res, "0x0: ebreak ");
        let res = disasm(0x00000013, 0);
        assert_eq!(res, "0x0: nop ");
        let res = disasm(0x00100013, 0);
        assert_eq!(res, "0x0: addi zero, zero, 1");
    }
} 