use crate::decoder::{instr::Instr, loc::Location, mov::MoveInstr};

pub struct SimState {
    registers: [u16; 8],
}

impl SimState {
    pub fn new() -> Self {
        Self { registers: [0; 8] }
    }

    fn get_register_16(&self, name: &str) -> u16 {
        match name {
            "ax" => self.registers[0],
            "bx" => self.registers[1],
            "cx" => self.registers[2],
            "dx" => self.registers[3],
            "si" => self.registers[4],
            "di" => self.registers[5],
            "bp" => self.registers[6],
            "sp" => self.registers[7],
            _ => panic!("Unknown register: {}", name),
        }
    }

    fn set_register_16(&mut self, name: &str, value: u16) {
        match name {
            "ax" => self.registers[0] = value,
            "bx" => self.registers[1] = value,
            "cx" => self.registers[2] = value,
            "dx" => self.registers[3] = value,
            "si" => self.registers[4] = value,
            "di" => self.registers[5] = value,
            "bp" => self.registers[6] = value,
            "sp" => self.registers[7] = value,
            _ => panic!("Unknown register: {}", name),
        }
    }

    fn get_register_8(&self, name: &str) -> u8 {
        match name {
            "al" => self.registers[0] as u8,
            "ah" => (self.registers[0] >> 8) as u8,
            "bl" => self.registers[1] as u8,
            "bh" => (self.registers[1] >> 8) as u8,
            "cl" => self.registers[2] as u8,
            "ch" => (self.registers[2] >> 8) as u8,
            "dl" => self.registers[3] as u8,
            "dh" => (self.registers[3] >> 8) as u8,
            _ => panic!("Unknown register: {}", name),
        }
    }

    fn set_register_8(&mut self, name: &str, value: u8) {
        let value = value as u16;
        match name {
            "al" => self.registers[0] = (self.registers[0] & 0xFF00) | value,
            "ah" => self.registers[0] = (self.registers[0] & 0x00FF) | (value << 8),
            "bl" => self.registers[1] = (self.registers[1] & 0xFF00) | value,
            "bh" => self.registers[1] = (self.registers[1] & 0x00FF) | (value << 8),
            "cl" => self.registers[2] = (self.registers[2] & 0xFF00) | value,
            "ch" => self.registers[2] = (self.registers[2] & 0x00FF) | (value << 8),
            "dl" => self.registers[3] = (self.registers[3] & 0xFF00) | value,
            "dh" => self.registers[3] = (self.registers[3] & 0x00FF) | (value << 8),
            _ => panic!("Unknown register: {}", name),
        }
    }

    pub fn execute(&mut self, instr: &Instr) {
        match instr {
            Instr::Mov(mov) => self.execute_mov(mov),
            _ => unimplemented!(),
        }
    }

    fn execute_mov(&mut self, mov: &MoveInstr) {
        let value = match &mov.src {
            Location::Reg(reg) => self.get_register_16(reg),
            Location::Immediate8(val) => *val as u16,
            _ => unimplemented!(),
        };

        match &mov.dest {
            Location::Reg(reg) => self.set_register_16(reg, value),
            Location::Mem(mem) => {
                let addr = self.get_register_16(&mem.base) + mem.disp;
                self.set_mem_16(addr, value);
            }
        }
    }
}
