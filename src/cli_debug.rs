use crate::{cpu::CPUState, disassembler::disassemble_opcode};


pub fn debug_state(cpu: &CPUState) -> String {
    let opcode1 = fmt_opcode(cpu.pc, cpu.get_opcode());
    let opcode2 = fmt_opcode(cpu.pc+2, cpu.mem.read_opcode(cpu.pc+2));
    let opcode3 = fmt_opcode(cpu.pc+4, cpu.mem.read_opcode(cpu.pc+4));
    let width = opcode1.len().max(opcode2.len()).max(opcode3.len());
    let mut s = format!("-> |{:width$}|\n   |{:width$}|\n   |{:width$}|",
        opcode1,
        opcode2,
        opcode3,
    );
    s = format!("{}\n{}", s, cpu.debug_state());
    return s;
}

fn unwrap_ok_or_err(x: Result<String, String>) -> String {
    match x {
        Ok(v) => v,
        Err(v) => v,
    }
}

fn fmt_opcode(addr: u16, opcode: u16) -> String {
    format!("{:X} - {}", addr, unwrap_ok_or_err(disassemble_opcode(opcode)))
}