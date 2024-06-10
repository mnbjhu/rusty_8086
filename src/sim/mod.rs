use std::fmt::Display;

use crate::decoder::{
    instr::{decode_instr, Instr},
    loc::{eac::EffectiveAddress, eac_mode::EffectiveAddressMode, Location, Size},
    mov::{MoveInstr, BX, SI},
    state::Decoder,
};

use self::flags::Flags;

pub mod flags;
pub mod jmp;
pub mod op_kind;

pub struct SimState {
    registers: [u16; 8],
    ip: u16,
    flags: Flags,
    instr_len: u8,
    program_size: usize,
    memory: [u8; 0xFFFF],
}

impl SimState {
    pub fn new(src: Vec<u8>) -> Self {
        let mut memory = [0; 0xFFFF];
        let program_size = src.len();
        memory[..program_size].copy_from_slice(&src);
        Self {
            registers: [0; 8],
            flags: Flags::default(),
            memory,
            instr_len: 0,
            program_size,
            ip: 0,
        }
    }

    pub fn get_register_16(&self, name: &str) -> u16 {
        match name {
            "ax" => self.registers[0],
            "bx" => self.registers[1],
            "cx" => self.registers[2],
            "dx" => self.registers[3],
            "si" => self.registers[4],
            "di" => self.registers[5],
            "bp" => self.registers[6],
            "sp" => self.registers[7],
            "ip" => self.ip,
            _ => panic!("Unknown register: {}", name),
        }
    }

    pub fn set_register_16(&mut self, name: &str, value: u16) {
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
            Instr::Op(op) => op.execute(self),
            Instr::Je(offset) => self.execute_je(*offset),
            Instr::Jne(offset) => self.execute_jne(*offset),
        }
    }

    fn execute_mov(&mut self, mov: &MoveInstr) {
        match (mov.dest.implied_size(), mov.src.implied_size()) {
            (None, None) | (Some(Size::Word), _) | (_, Some(Size::Word)) => {
                self.set_value_word(&mov.dest, self.get_value_word(&mov.src))
            }
            (Some(Size::Byte), _) | (_, Some(Size::Byte)) => {
                self.set_value_byte(&mov.dest, self.get_value_byte(&mov.src))
            }
        }
    }

    pub fn run(&mut self) {
        while self.has_more() {
            let instr = decode_instr(self);
            self.advance();
            self.execute(&instr);
        }
    }

    pub fn get_value_byte(&self, loc: &Location) -> u8 {
        match loc {
            Location::Reg(reg) => self.get_register_8(reg),
            Location::Immediate8(value) => *value,
            Location::Immediate16(value) => panic!("Expected byte, got word: {}", value),
            Location::Mem(addr) => self.memory[*addr as usize],
            Location::Eac(eac) => {
                let addr = self.get_addr(eac);
                self.memory[addr as usize]
            }
        }
    }

    pub fn set_value_byte(&mut self, loc: &Location, value: u8) {
        match loc {
            Location::Reg(reg) => self.set_register_8(reg, value),
            Location::Immediate8(_) => panic!("Cannot set value to immediate"),
            Location::Immediate16(_) => panic!("Expected byte, got word: {}", value),
            Location::Mem(addr) => self.memory[*addr as usize] = value,
            Location::Eac(eac) => {
                let addr = self.get_addr(eac);
                self.memory[addr as usize] = value;
            }
        }
    }

    pub fn get_value_word(&self, loc: &Location) -> u16 {
        match loc {
            Location::Reg(reg) => self.get_register_16(reg),
            Location::Immediate8(value) => *value as u16,
            Location::Immediate16(value) => *value,
            Location::Mem(addr) => {
                let low = self.memory[*addr as usize] as u16;
                let high = self.memory[*addr as usize + 1] as u16;
                high << 8 | low
            }
            Location::Eac(eac) => {
                let addr = self.get_addr(eac);
                let low = self.memory[addr as usize] as u16;
                let high = self.memory[addr as usize + 1] as u16;
                high << 8 | low
            }
        }
    }

    pub fn set_value_word(&mut self, loc: &Location, value: u16) {
        match loc {
            Location::Reg(reg) => self.set_register_16(reg, value),
            Location::Immediate8(_) => panic!("Cannot set value to immediate"),
            Location::Immediate16(_) => panic!("Expected byte, got word: {}", value),
            Location::Mem(addr) => {
                self.memory[*addr as usize] = value as u8;
                self.memory[*addr as usize + 1] = (value >> 8) as u8;
            }
            Location::Eac(eac) => {
                let addr = self.get_addr(eac);
                self.memory[addr as usize] = value as u8;
                self.memory[addr as usize + 1] = (value >> 8) as u8;
            }
        }
    }

    fn get_addr(&self, eac: &EffectiveAddress) -> u16 {
        match eac.mode() {
            EffectiveAddressMode::BxSi => {
                let bx = self.get_register_16(BX);
                let si = self.get_register_16(SI);
                bx.wrapping_add(si).wrapping_add_signed(eac.offset())
            }
            EffectiveAddressMode::BxDi => {
                let bx = self.get_register_16(BX);
                let di = self.get_register_16(SI);
                bx.wrapping_add(di).wrapping_add_signed(eac.offset())
            }
            EffectiveAddressMode::BpSi => {
                let bp = self.get_register_16(BX);
                let si = self.get_register_16(SI);
                bp.wrapping_add(si).wrapping_add_signed(eac.offset())
            }
            EffectiveAddressMode::BpDi => {
                let bp = self.get_register_16(BX);
                let di = self.get_register_16(SI) as i16;
                bp.wrapping_add_signed(di).wrapping_add_signed(eac.offset())
            }
            EffectiveAddressMode::Si => {
                let si = self.get_register_16(SI);
                si.wrapping_add_signed(eac.offset())
            }
            EffectiveAddressMode::Di => {
                let di = self.get_register_16(SI);
                di.wrapping_add_signed(eac.offset())
            }
            EffectiveAddressMode::Bp => {
                let bp = self.get_register_16(BX);
                bp.wrapping_add_signed(eac.offset())
            }
            EffectiveAddressMode::Bx => {
                let bx = self.get_register_16(BX);
                bx.wrapping_add_signed(eac.offset())
            }
        }
    }
}

impl Decoder for SimState {
    fn has_more(&self) -> bool {
        self.ip < self.program_size as u16
    }

    fn get_byte(&self, offset: usize) -> u8 {
        self.memory[self.ip as usize + offset]
    }

    fn add_len(&mut self, len: usize) {
        self.instr_len += len as u8;
    }

    fn next(&mut self) -> bool {
        self.advance();
        self.has_more()
    }

    fn get_instr_len(&self) -> usize {
        self.instr_len as usize
    }

    fn advance(&mut self) {
        self.ip += self.instr_len as u16;
        self.instr_len = 0;
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
        writeln!(f, "sp: {:04x}", self.registers[7])?;
        writeln!(f, "bp: {:04x}", self.registers[6])?;
        writeln!(f, "si: {:04x}", self.registers[4])?;
        writeln!(f, "di: {:04x}", self.registers[5])
    }
}

#[cfg(test)]
mod test {
    use crate::sim::SimState;

    #[test]
    fn test_register_16() {
        let mut state = SimState::new(vec![0; 0]);
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
        let mut state = SimState::new(vec![0b10110011, 0b1100100]);
        state.run();
        assert_eq!(state.get_register_8("bl"), 100);
    }

    #[test]
    fn test_mov_imm_to_reg_higher() {
        let mut state = SimState::new(vec![0b10110111, 0b1100100]);
        state.run();
        assert_eq!(state.get_register_8("bh"), 100);
    }

    #[test]
    fn test_mov_imm_to_reg_16bit() {
        let mut state = SimState::new(vec![0b10111011, 0b1100100, 0b0]);
        state.run();
        assert_eq!(state.get_register_16("bx"), 100);
    }

    #[test]
    fn test_mov_reg_high_to_reg_low() {
        let mut state = SimState::new(vec![0b10001000, 0b11010101]);
        state.set_register_8("dl", 100);
        state.run();
        assert_eq!(state.get_register_8("ch"), 100);
    }

    #[test]
    fn test_mov_reg_low_to_reg_high() {
        let mut state = SimState::new(vec![0b10001000, 0b11101010]);
        state.set_register_8("ch", 100);
        state.run();
        assert_eq!(state.get_register_8("dl"), 100);
    }

    #[test]
    fn mov_reg_to_reg() {
        let mut state = SimState::new(vec![0b10001001, 0b11000001]);
        state.set_register_16("ax", 1234);
        state.run();
        assert_eq!(state.get_register_16("cx"), 1234);
    }

    #[test]
    fn test_state_display() {
        let state = SimState {
            registers: [
                0x1234, 0x5678, 0x9ABC, 0xDEF0, 0x1357, 0x2468, 0xACE0, 0xBEEF,
            ],
            flags: Default::default(),
            memory: [0; 0xFFFF],
            instr_len: 0,
            program_size: 0,
            ip: 0,
        };
        let expected =
            "ax: 1234\nbx: 5678\ncx: 9abc\ndx: def0\nsp: beef\nbp: ace0\nsi: 1357\ndi: 2468\n";
        assert_eq!(format!("{}", state), expected);
    }
}
