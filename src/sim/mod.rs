use std::fmt::Display;

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

    #[allow(dead_code)]
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
        match (&mov.dest, &mov.src) {
            (Location::Reg(dest), Location::Reg(src)) => {
                if is_byte(dest) {
                    let value = self.get_register_8(src);
                    self.set_register_8(dest, value);
                } else {
                    let value = self.get_register_16(src);
                    self.set_register_16(dest, value);
                }
            }
            (Location::Reg(dest), Location::Immediate8(value)) => {
                self.set_register_8(dest, *value);
            }
            (Location::Reg(dest), Location::Immediate16(value)) => {
                self.set_register_16(dest, *value);
            }
            _ => unimplemented!(),
        }
    }
}

pub fn is_byte(reg: &str) -> bool {
    matches!(reg, "al" | "ah" | "bl" | "bh" | "cl" | "ch" | "dl" | "dh")
}

impl Display for SimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ax: {:04x}", self.registers[0])?;
        writeln!(f, "bx: {:04x}", self.registers[1])?;
        writeln!(f, "cx: {:04x}", self.registers[2])?;
        writeln!(f, "dx: {:04x}", self.registers[3])?;
        writeln!(f, "si: {:04x}", self.registers[4])?;
        writeln!(f, "di: {:04x}", self.registers[5])?;
        writeln!(f, "bp: {:04x}", self.registers[6])?;
        writeln!(f, "sp: {:04x}", self.registers[7])
    }
}

#[cfg(test)]
mod test {
    use crate::{decoder::decode, sim::SimState};

    #[test]
    fn test_register_16() {
        let mut state = SimState::new();
        state.set_register_16("ax", 0x1234);
        state.set_register_16("bx", 0x5678);
        state.set_register_16("cx", 0x9ABC);
        state.set_register_16("dx", 0xDEF0);
        state.set_register_16("si", 0x1357);
        state.set_register_16("di", 0x2468);
        state.set_register_16("bp", 0xACE0);
        state.set_register_16("sp", 0xBEEF);

        assert_eq!(state.get_register_16("ax"), 0x1234);
        assert_eq!(state.get_register_16("bx"), 0x5678);
        assert_eq!(state.get_register_16("cx"), 0x9ABC);
        assert_eq!(state.get_register_16("dx"), 0xDEF0);
        assert_eq!(state.get_register_16("si"), 0x1357);
        assert_eq!(state.get_register_16("di"), 0x2468);
        assert_eq!(state.get_register_16("bp"), 0xACE0);
        assert_eq!(state.get_register_16("sp"), 0xBEEF);
    }

    #[test]
    fn test_mov_imm_to_reg_lower() {
        let mut bytes = vec![0b10110011, 0b1100100].into_iter();
        let mut state = SimState::new();
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_8("bl"), 100);
    }

    #[test]
    fn test_mov_imm_to_reg_higher() {
        let mut bytes = vec![0b10110111, 0b1100100].into_iter();
        let mut state = SimState::new();
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_8("bh"), 100);
    }

    #[test]
    fn test_mov_imm_to_reg_16bit() {
        let mut bytes = vec![0b10111011, 0b1100100, 0b0].into_iter();
        let mut state = SimState::new();
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_16("bx"), 100);
    }

    #[test]
    fn test_mov_reg_high_to_reg_low() {
        let mut bytes = vec![0b10001000, 0b11010101].into_iter();
        let mut state = SimState::new();
        state.set_register_8("dl", 100);
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_8("ch"), 100);
    }

    #[test]
    fn test_mov_reg_low_to_reg_high() {
        let mut bytes = vec![0b10001000, 0b11101010].into_iter();
        let mut state = SimState::new();
        state.set_register_8("ch", 100);
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_8("dl"), 100);
    }

    #[test]
    fn mov_reg_to_reg() {
        let mut bytes = vec![0b10001001, 0b11000001].into_iter();
        let mut state = SimState::new();
        state.set_register_16("ax", 1234);
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_16("cx"), 1234);
    }

    #[test]
    fn test_state_display() {
        let state = SimState {
            registers: [
                0x1234, 0x5678, 0x9ABC, 0xDEF0, 0x1357, 0x2468, 0xACE0, 0xBEEF,
            ],
        };
        let expected =
            "ax: 1234\nbx: 5678\ncx: 9abc\ndx: def0\nsi: 1357\ndi: 2468\nbp: ace0\nsp: beef\n";
        assert_eq!(format!("{}", state), expected);
    }
}
