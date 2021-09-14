// rust in action ch5, CPU

struct CPU {
    registers: [u8; 16],

    position_in_memory: usize,
    memory: [u8; 0x1000],

    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let c = self.position_in_memory;
        let byte1 = self.memory[c] as u16;
        let byte2 = self.memory[c + 1] as u16;
        byte1 << 8 | byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0x0, 0x0, 0x0, 0x0) => break,
                (0x8,   _,   _, 0x4) => self.add_xy(x, y),
                (0x2,   _,   _,   _) => self.call(opcode & 0x0FFF),
                (0x0, 0x0, 0xE, 0xE) => self.ret(),
                _ => todo!("opcode {:04x}", opcode)
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let a1 = self.registers[x as usize];
        let a2 = self.registers[y as usize];
        let (val, overflow) = a1.overflowing_add(a2);
        self.registers[x as usize] = val;
        self.registers[0xf] = if overflow { 1 } else { 0 }
    }

    fn call(&mut self, address: u16) {
        if self.stack_pointer >= self.stack.len() {
            panic!("stack overflow")
        }

        self.stack[self.stack_pointer] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = address as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack underflow")
        }
        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let mut cpu = CPU {
            registers: [0; 16],

            position_in_memory: 0,
            memory: [0; 4096],

            stack: [0; 16],
            stack_pointer: 0,
        };

        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        // start
        cpu.memory[0x000] = 0x21; // call 0x100
        cpu.memory[0x001] = 0x00;

        cpu.memory[0x002] = 0x21; // call 0x100
        cpu.memory[0x003] = 0x00;

        cpu.memory[0x004] = 0x00; // stop
        cpu.memory[0x005] = 0x00;

        // function
        cpu.memory[0x100] = 0x80; // add xy
        cpu.memory[0x101] = 0x14;

        cpu.memory[0x102] = 0x80; // add xy
        cpu.memory[0x103] = 0x14;

        cpu.memory[0x104] = 0x00; // return
        cpu.memory[0x105] = 0xee;

        cpu.run();

        assert_eq!(cpu.registers[0], 45);
    }
}
