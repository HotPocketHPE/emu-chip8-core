use crate::cpu::{nnn, x, kk, y, n};


pub fn disassemble_opcode(opcode: u16) -> Result<String, String> {
    match opcode {
        0x00E0 => Ok("CLS".into()),
        0x00EE => Ok("RET".into()),
        _ if matches_opcode(opcode, 0x1, None, None, None) => Ok(format!("JP {:X}", nnn(opcode))),
        _ if matches_opcode(opcode, 0x2, None, None, None) => Ok(format!("CALL {:X}", nnn(opcode))),
        _ if matches_opcode(opcode, 0x3, None, None, None) => Ok(format!("SE V{:X} {:X}", x(opcode), kk(opcode))),
        _ if matches_opcode(opcode, 0x4, None, None, None) => Ok(format!("SNE V{:X} {:X}", x(opcode), kk(opcode))),
        _ if matches_opcode(opcode, 0x5, None, None, Some(0x0)) => Ok(format!("SE V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x6, None, None, None) => Ok(format!("LD V{:X} {:X}", x(opcode), kk(opcode))),
        _ if matches_opcode(opcode, 0x7, None, None, None) => Ok(format!("ADD V{:X} {:X}", x(opcode), kk(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0x0)) => Ok(format!("LD V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0x1)) => Ok(format!("OR V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0x2)) => Ok(format!("AND V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0x3)) => Ok(format!("XOR V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0x4)) => Ok(format!("ADD V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0x5)) => Ok(format!("SUB V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0x6)) => Ok(format!("SHR V{:X} (V{:X})", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0x7)) => Ok(format!("SUBN V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x8, None, None, Some(0xE)) => Ok(format!("SHL V{:X} (V{:X})", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0x9, None, None, Some(0x0)) => Ok(format!("SNE V{:X} V{:X}", x(opcode), y(opcode))),
        _ if matches_opcode(opcode, 0xA, None, None, None) => Ok(format!("LD I {:X}", nnn(opcode))),
        _ if matches_opcode(opcode, 0xB, None, None, None) => Ok(format!("JP V0 {:X}", nnn(opcode))),
        _ if matches_opcode(opcode, 0xC, None, None, None) => Ok(format!("RND V{:X} {:X}", x(opcode), kk(opcode))),
        _ if matches_opcode(opcode, 0xD, None, None, None) => Ok(format!("DRW V{:X} V{:X} {:X}", x(opcode), y(opcode), n(opcode))),
        _ if matches_opcode(opcode, 0xE, None, Some(0x9), Some(0xE)) => Ok(format!("SKP {:X}", x(opcode))),
        _ if matches_opcode(opcode, 0xE, None, Some(0xA), Some(0x1)) => Ok(format!("SKNP {:X}", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x0), Some(0x7)) => Ok(format!("LD V{:X} DT", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x0), Some(0xA)) => Ok(format!("LD V{:X} K", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x1), Some(0x5)) => Ok(format!("LD DT V{:X}", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x1), Some(0x8)) => Ok(format!("LD ST V{:X}", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x1), Some(0xE)) => Ok(format!("ADD I V{:X}", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x2), Some(0x9)) => Ok(format!("LD F V{:X}", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x3), Some(0x3)) => Ok(format!("LD B V{:X}", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x5), Some(0x5)) => Ok(format!("LD [I] V{:X}", x(opcode))),
        _ if matches_opcode(opcode, 0xF, None, Some(0x6), Some(0x5)) => Ok(format!("LD V{:X} [I]", x(opcode))),
        _ => Err(format!("{:X} | Unknown opcode (probably data)", opcode))
    }
}

fn matches_opcode(opcode: u16, n1: u8, n2: Option<u8>, n3: Option<u8>, n4: Option<u8>) -> bool { 
    if (opcode >> 12) as u8 != n1 { return false; }
    if let Some(n) = n2 {
        if n as u16 != ((opcode & 0x0F00) >> 8) {
            return false;
        }
    }
    if let Some(n) = n3 {
        if n as u16 != ((opcode & 0x00F0) >> 4) {
            return false;
        }
    }
    if let Some(n) = n4 {
        if n as u16 != (opcode & 0x000F) {
            return false;
        }
    }
    return true;
}

