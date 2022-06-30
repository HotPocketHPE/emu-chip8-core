use rand::Rng;

struct CPUState {
    ///Program Counter
    pc: u16,
    ///Index Register
    i: u16,
    ///Registers
    v: [u8; 0x10],
    ///Stack Pointer
    sp: u8,
    ///Delay Timer
    dt: u8,
    ///Sound Timer
    st: u8
}

impl CPUState {
    fn write_mem(&mut self, val: u8) {
        todo!();
    }

    fn read_mem(&self, addr: u16) -> u8 {
        todo!();
    }

    fn get_opcode(&self) -> u16 {
        (self.read_mem(self.pc) as u16) << 8 & (self.read_mem(self.pc+1) as u16)
    }

    fn d_addr(&self) -> u16 {
        self.get_opcode() & 0x0FFF
    }

    fn d_n(&self) -> u8 {
        (self.get_opcode() & 0x000F) as u8
    }

    fn d_x(&self) -> usize {
        ((self.get_opcode() & 0x0F00) >> 8) as usize
    }

    fn d_y(&self) -> usize {
        ((self.get_opcode() & 0x00F0) >> 4) as usize
    }

    fn d_kk(&self) -> u8 {
        (self.get_opcode() & 0x00FF) as u8
    }
}

fn op_0nnn(cpu: &mut CPUState) {
    //SYS
    //call to native machine code, unimplemented
}

fn op_00E0(cpu: &mut CPUState) {
    //CLS
    todo!()
}

fn op_1nnn(cpu: &mut CPUState) {
    //JMP
    cpu.pc = cpu.d_addr();
}

fn op_2nnn(cpu: &mut CPUState) {
    //CALL
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

fn op_8xy8(cpu: &mut CPUState) {
    cpu.v[0xF] = (cpu.v[cpu.d_x()] & 0b10000000) >> 8;
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
    cpu.pc += 2;
}