#![allow(non_snake_case)]

use rand::Rng;
use super::cpu::CPUState;

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
    op_e_innerlookup,
    op_f_innerlookup
];

fn op_0_innerlookup(cpu: &mut CPUState) {
    if cpu.d_addr() == 0x0E0 {
        op_00E0(cpu);
    } else {
        op_0nnn(cpu);
    }
}

fn op_0nnn(cpu: &mut CPUState) {
    //SYS
    //call to native machine code, unimplemented
    panic!("SYS call to native machine attempted! {}", cpu.get_opcode());
}

fn op_00E0(cpu: &mut CPUState) {
    //CLS
    cpu.disp.clear();
    cpu.pc += 2;
}

fn op_1nnn(cpu: &mut CPUState) {
    //JMP
    cpu.pc = cpu.d_addr();
}

fn op_2nnn(cpu: &mut CPUState) {
    //CALL
    todo!()
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
    cpu.v[cpu.d_x()] = cpu.v[cpu.d_x()] | cpu.v[cpu.d_y()];
    cpu.pc += 2;
}

fn op_8xy2(cpu: &mut CPUState) {
    cpu.v[cpu.d_x()] = cpu.v[cpu.d_x()] & cpu.v[cpu.d_y()];
    cpu.pc += 2;
}

fn op_8xy3(cpu: &mut CPUState) {
    cpu.v[cpu.d_x()] = cpu.v[cpu.d_x()] ^ cpu.v[cpu.d_y()];
    cpu.pc += 2;
}

fn op_8xy4(cpu: &mut CPUState) {
    let result = cpu.v[cpu.d_x()] as u16 + cpu.v[cpu.d_y()] as u16;
    cpu.v[cpu.d_x()] = result as u8;
    cpu.v[0xF] = (result > 0xFF) as u8;
    cpu.pc += 2;
}

fn sub_regs(cpu: &mut CPUState, x: usize, y: usize) {
    cpu.v[0xF] = (cpu.v[x] < cpu.v[y]) as u8;
    cpu.v[x] = cpu.v[x].wrapping_sub(cpu.v[y]);
    cpu.pc += 2;
}

fn op_8xy5(cpu: &mut CPUState) {
    sub_regs(cpu, cpu.d_x(), cpu.d_y());
}

fn op_8xy6(cpu: &mut CPUState) {
    cpu.v[0xF] = cpu.v[cpu.d_x()] & 0b00000001;
    cpu.v[cpu.d_x()] = cpu.v[cpu.d_x()] >> 1;
    cpu.pc += 2;
}

fn op_8xy7(cpu: &mut CPUState) {
    sub_regs(cpu, cpu.d_y(), cpu.d_x());
}

fn op_8xyE(cpu: &mut CPUState) {
    cpu.v[0xF] = (cpu.v[cpu.d_x()] & 0b10000000) >> 7;
    cpu.v[cpu.d_x()] = cpu.v[cpu.d_x()] << 1;
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
    //draw sprite
    let mut sprite_mem: Vec<u8> = Vec::with_capacity(cpu.d_n() as usize);
    for mem_offset in 0..cpu.d_n() as u16 {
        sprite_mem.push(cpu.mem.read(cpu.i + mem_offset));
    }
    cpu.disp.draw(&sprite_mem, cpu.d_x(), cpu.d_y());
    cpu.pc += 2;
}

fn op_e_innerlookup(cpu: &mut CPUState) {
    match cpu.d_kk() {
        0x9E => op_Ex9E(cpu),
        0xA1 => op_ExA1(cpu),
        _ => panic!("Unknown opcode! {:X}", cpu.get_opcode()) 
    }
}

fn op_Ex9E(cpu: &mut CPUState) {
    //skip if key pressed
    skip_next_instr_if(cpu, |cpu| cpu.kbstate.key[cpu.d_x()])
}

fn op_ExA1(cpu: &mut CPUState) {
    //skip if key not pressed
    skip_next_instr_if(cpu, |cpu| !cpu.kbstate.key[cpu.d_x()])
}

fn op_f_innerlookup(cpu: &mut CPUState) {
    match cpu.d_kk() {
        0x15 => op_fx15(cpu),
        0x18 => op_fx18(cpu),
        0x1E => op_fx1e(cpu),
        0x29 => op_fx29(cpu),
        0x33 => op_fx33(cpu),
        0x55 => op_fx55(cpu),
        0x65 => op_fx65(cpu),
        _ => panic!("Unknown opcode! {:X}", cpu.get_opcode())
    }
}

fn op_fx15(cpu: &mut CPUState) {
    cpu.dt = cpu.v[cpu.d_x()];
    cpu.pc += 2;
}

fn op_fx18(cpu: &mut CPUState) {
    cpu.st = cpu.v[cpu.d_x()];
    cpu.pc += 2;
}

fn op_fx1e(cpu: &mut CPUState) {
    cpu.i = cpu.i.wrapping_add(cpu.v[cpu.d_x()] as u16);
    cpu.pc += 2;
}

fn op_fx29(cpu: &mut CPUState) {
    //set I to location of Vx sprite
    todo!()
}

fn op_fx33(cpu: &mut CPUState) {
    //store Vx as BCD in I, I+1, I+2
    todo!()
}

fn op_fx55(cpu: &mut CPUState) {
    for i in 0..=cpu.d_x() {
        {
            let addr = cpu.i + (i as u16);
            let val = cpu.v[i];
            cpu.mem.write(addr, val)
        };
    }
    cpu.pc += 2;
}

fn op_fx65(cpu: &mut CPUState) {
    for i in 0..=cpu.d_x() {
        cpu.v[i] = {
            let addr = cpu.i + (i as u16);
            cpu.mem.read(addr)
        };
    }
    cpu.pc += 2;
}