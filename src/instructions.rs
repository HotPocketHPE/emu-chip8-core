#![allow(non_snake_case)]

use rand::Rng;

use crate::config::CHIP8_CONFIG;
use crate::keyboard::Fx0AStatus;
use crate::memory::Memory;
use crate::cpu::CPUState;

pub const OUTER_FUNC_TABLE: [fn(&mut CPUState); 0x10] = [
    op_0_innerlookup,
    op_1nnn,
    op_2nnn,
    op_3xkk,
    op_4xkk,
    op_5xy0,
    op_6xkk,
    op_7xkk,
    op_8_innerlookup,
    op_9xy0,
    op_Annn,
    op_Bnnn,
    op_Cxkk,
    op_Dxyn,
    op_E_innerlookup,
    op_F_innerlookup
];

fn op_0_innerlookup(cpu: &mut CPUState) {
    match cpu.d_addr() {
        0x00E0 => op_00E0(cpu),
        0x00EE => op_00EE(cpu),
        0x0000 => panic!("Tried to execute 0000! (Probably uninit memory) at addr {:X} {:X}", cpu.pc, cpu.get_opcode()),
        _ => op_0nnn(cpu)
    }
}

fn op_0nnn(_cpu: &mut CPUState) {
    //SYS
    //call to native machine code, unimplemented
    //panic!("SYS call to native machine attempted! {:X} at addr {:X}", cpu.get_opcode(), cpu.pc);
}

fn op_00E0(cpu: &mut CPUState) {
    //CLS
    cpu.disp.clear();
    cpu.pc += 2;
}

fn op_00EE(cpu: &mut CPUState) {
    //RET
    let addr = u16::from_be_bytes([cpu.mem.read((cpu.sp-1) as u16), cpu.mem.read(cpu.sp as u16)]);
    cpu.sp -= 2;
    cpu.pc = addr + 2;
}

fn op_1nnn(cpu: &mut CPUState) {
    //JMP
    cpu.pc = cpu.d_addr();
}

fn op_2nnn(cpu: &mut CPUState) {
    //CALL
    cpu.sp += 2;
    let bytes = cpu.pc.to_be_bytes();
    cpu.mem.write((cpu.sp-1) as u16, bytes[0]);
    cpu.mem.write((cpu.sp) as u16, bytes[1]);
    cpu.pc = cpu.d_addr();
    //println!("{}", cpu.reg_states());
}

fn skip_next_instr_if(cpu: &mut CPUState, cond: fn(&CPUState) -> bool) {
    if cond(cpu) {
        cpu.pc += 4;
    } else {
        cpu.pc += 2;
    }
}

fn op_3xkk(cpu: &mut CPUState) {
    skip_next_instr_if(cpu, |cpu| cpu.v[cpu.d_x()] == cpu.d_kk())
}

fn op_4xkk(cpu: &mut CPUState) {
    skip_next_instr_if(cpu, |cpu| cpu.v[cpu.d_x()] != cpu.d_kk())
}

fn op_5xy0(cpu: &mut CPUState) {
    skip_next_instr_if(cpu, |cpu| cpu.v[cpu.d_x()] == cpu.v[cpu.d_y()])
}

fn op_6xkk(cpu: &mut CPUState) {
    cpu.v[cpu.d_x()] = cpu.d_kk();
    cpu.pc += 2;
}

fn op_7xkk(cpu: &mut CPUState) {
    cpu.v[cpu.d_x()] = cpu.v[cpu.d_x()].wrapping_add(cpu.d_kk());
    cpu.pc += 2;
}

fn op_8_innerlookup(cpu: &mut CPUState) {

    const INNER_FUNC_TABLE: [fn(&mut CPUState); 8] = [
        op_8xy0,
        op_8xy1,
        op_8xy2,
        op_8xy3,
        op_8xy4,
        op_8xy5,
        op_8xy6,
        op_8xy7,
    ];

    match cpu.d_n() {
        i @ 0..=7 => INNER_FUNC_TABLE[i as usize](cpu),
        0xE => op_8xyE(cpu),
        _ => panic!("Unknown opcode! {:X}", cpu.get_opcode()) 
    }
}

fn op_8xy0(cpu: &mut CPUState) {
    cpu.v[cpu.d_x()] = cpu.v[cpu.d_y()];
    cpu.pc += 2;
}


fn op_8xy1(cpu: &mut CPUState) {
    let result = cpu.v[cpu.d_x()] | cpu.v[cpu.d_y()];
    cpu.v[0xF] = 0;
    cpu.v[cpu.d_x()] = result;
    cpu.pc += 2;
}

fn op_8xy2(cpu: &mut CPUState) {
    let result = cpu.v[cpu.d_x()] & cpu.v[cpu.d_y()];
    cpu.v[0xF] = 0;
    cpu.v[cpu.d_x()] = result;
    cpu.pc += 2;
}

fn op_8xy3(cpu: &mut CPUState) {
    let result = cpu.v[cpu.d_x()] ^ cpu.v[cpu.d_y()];
    cpu.v[0xF] = 0;
    cpu.v[cpu.d_x()] = result;
    cpu.pc += 2;
}

fn op_8xy4(cpu: &mut CPUState) {
    let result = cpu.v[cpu.d_x()] as u16 + cpu.v[cpu.d_y()] as u16;
    cpu.v[cpu.d_x()] = result as u8;
    cpu.v[0xF] = (result > 0xFF) as u8;
    cpu.pc += 2;
}

fn sub_regs(cpu: &mut CPUState, x: usize, y: usize) -> u8 {
    let result = cpu.v[x].wrapping_sub(cpu.v[y]);
    cpu.v[0xF] = (cpu.v[x] > cpu.v[y]) as u8;
    return result;
}

fn op_8xy5(cpu: &mut CPUState) {
    cpu.v[cpu.d_x()] = sub_regs(cpu, cpu.d_x(), cpu.d_y());
    cpu.pc += 2;
}

fn op_8xy6(cpu: &mut CPUState) {
    let reg_to_shift = if CHIP8_CONFIG.shifting_with_Vy
        { cpu.v[cpu.d_y()] } else { cpu.v[cpu.d_x()] };
    let result = reg_to_shift >> 1;
    cpu.v[cpu.d_x()] = result;
    cpu.v[0xF] = reg_to_shift & 1;
    cpu.pc += 2;
}

fn op_8xy7(cpu: &mut CPUState) {
    cpu.v[cpu.d_x()] = sub_regs(cpu, cpu.d_y(), cpu.d_x());
    cpu.pc += 2;
}

fn op_8xyE(cpu: &mut CPUState) {
    let reg_to_shift = if CHIP8_CONFIG.shifting_with_Vy
        { cpu.v[cpu.d_y()] } else { cpu.v[cpu.d_x()] };
    let result = reg_to_shift << 1;
    cpu.v[cpu.d_x()] = result;
    cpu.v[0xF] = (reg_to_shift & 0b10000000) >> 7;
    cpu.pc += 2;
}

fn op_9xy0(cpu: &mut CPUState) {
    skip_next_instr_if(cpu, |cpu| cpu.v[cpu.d_x()] != cpu.v[cpu.d_y()])
}

fn op_Annn(cpu: &mut CPUState) {
    cpu.i = cpu.d_addr();
    cpu.pc += 2;
}

fn op_Bnnn(cpu: &mut CPUState) {
    cpu.pc = cpu.d_addr().wrapping_add(cpu.v[0] as u16);
}

fn op_Cxkk(cpu: &mut CPUState) {
    let mut rng = rand::thread_rng();
    cpu.v[cpu.d_x()] = cpu.v[cpu.d_x()] & rng.gen_range(0..=0xFF);
    cpu.pc += 2;
}

fn op_Dxyn(cpu: &mut CPUState) {
    //DRW
    if CHIP8_CONFIG.emulate_draw_vblank_delay {
        //hack to make the draw instr take longer
        if cpu.idle_cycles > 0 {
            cpu.idle_cycles -= 1;
            if cpu.idle_cycles == 0 {
                draw(cpu);
            }
        } else {
            cpu.idle_cycles = CHIP8_CONFIG.vblank_idle_cycles;
        }
    } else {
        draw(cpu);
    }


}

fn draw(cpu: &mut CPUState) {
    let sprite_mem = &cpu.mem.slice()[cpu.i as usize..(cpu.i+cpu.d_n() as u16) as usize];
    let collision = cpu.disp.draw(sprite_mem, cpu.v[cpu.d_x()] as usize, cpu.v[cpu.d_y()] as usize);
    cpu.v[0xF] = collision as u8;
    cpu.pc += 2;
}

fn op_E_innerlookup(cpu: &mut CPUState) {
    match cpu.d_kk() {
        0x9E => op_Ex9E(cpu),
        0xA1 => op_ExA1(cpu),
        _ => panic!("Unknown opcode! {:X}", cpu.get_opcode()) 
    }
}

fn op_Ex9E(cpu: &mut CPUState) {
    //skip if key pressed
    skip_next_instr_if(cpu, |cpu| {
        let key: usize = cpu.v[cpu.d_x()].try_into().unwrap();
        cpu.kbstate.key[key]
    }); 
}

fn op_ExA1(cpu: &mut CPUState) {
    //skip if key not pressed
    skip_next_instr_if(cpu, |cpu| {
        let key: usize = cpu.v[cpu.d_x()].try_into().unwrap();
        !cpu.kbstate.key[key]
    }); 
}

fn op_F_innerlookup(cpu: &mut CPUState) {
    match cpu.d_kk() {
        0x07 => op_Fx07(cpu),
        0x0A => op_Fx0A(cpu),
        0x15 => op_Fx15(cpu),
        0x18 => op_Fx18(cpu),
        0x1E => op_Fx1E(cpu),
        0x29 => op_Fx29(cpu),
        0x33 => op_Fx33(cpu),
        0x55 => op_Fx55(cpu),
        0x65 => op_Fx65(cpu),
        _ => panic!("Unknown opcode! {:X}", cpu.get_opcode())
    }
}

fn op_Fx07(cpu: &mut CPUState) {
    cpu.v[cpu.d_x()] = cpu.dt;
    cpu.pc += 2;
}

fn op_Fx0A(cpu: &mut CPUState) {
    match cpu.kbstate.Fx0A {
        Fx0AStatus::Inactive => {
            cpu.kbstate.Fx0A = Fx0AStatus::WaitingForPress;
        },
        Fx0AStatus::WaitingForPress => {},
        Fx0AStatus::WaitingForRelease(_) => {},
        Fx0AStatus::JustReleased(key) => {
            cpu.kbstate.Fx0A = Fx0AStatus::Inactive;
            cpu.v[cpu.d_x()] = key;
            cpu.pc += 2;
        },
    } 
}

fn op_Fx15(cpu: &mut CPUState) {
    cpu.dt = cpu.v[cpu.d_x()];
    cpu.pc += 2;
}

fn op_Fx18(cpu: &mut CPUState) {
    cpu.st = cpu.v[cpu.d_x()];
    cpu.pc += 2;
}

fn op_Fx1E(cpu: &mut CPUState) {
    cpu.i = cpu.i.wrapping_add(cpu.v[cpu.d_x()] as u16);
    cpu.pc += 2;
}

fn op_Fx29(cpu: &mut CPUState) {
    //set I to location of Vx sprite
    cpu.i = Memory::get_font_addr(cpu.d_x() as u8);
    cpu.pc += 2;
}

fn op_Fx33(cpu: &mut CPUState) {
    //store Vx as BCD in I, I+1, I+2
    let mut n = cpu.v[cpu.d_x()];
    for i in (0..=2).rev() {
        cpu.mem.write(cpu.i + i, n % 10);
        n = n / 10;
    }
    cpu.pc += 2;
}

fn op_Fx55(cpu: &mut CPUState) {
    for i in 0..=cpu.d_x() { 
        let addr = cpu.i + (i as u16);
        let val = cpu.v[i];
        cpu.mem.write(addr, val)
    }
    cpu.i += cpu.d_x() as u16 + 1;
    cpu.pc += 2;
}

fn op_Fx65(cpu: &mut CPUState) {
    for i in 0..=cpu.d_x() {
        cpu.v[i] = {
            let addr = cpu.i + (i as u16);
            cpu.mem.read(addr)
        };
    }
    cpu.i += cpu.d_x() as u16 + 1;
    cpu.pc += 2;
}