use crate::decoder::{
    loc::Location,
    op::{OpInstr, OpKind},
};

use super::{is_byte, SimState};

#[derive(Default)]
pub struct Flags {
    pub zero: bool,
    pub sign: bool,
}

impl OpInstr {
    pub fn execute(&self, state: &mut SimState) {
        match (&self.dest, &self.src) {
            (Location::Reg(dest), Location::Reg(src)) => {
                if is_byte(dest) {
                    let value = state.get_register_8(src);
                    let current = state.get_register_8(dest);
                    let res = self.kind.execute_byte(state, current, value);
                    state.set_register_8(dest, res);
                } else {
                    let value = state.get_register_16(src);
                    let current = state.get_register_16(dest);
                    let res = self.kind.execute_word(state, current, value);
                    state.set_register_16(dest, res);
                }
            }
            (Location::Reg(dest), Location::Immediate8(value)) => {
                if is_byte(dest) {
                    let current = state.get_register_8(dest);
                    let res = self.kind.execute_byte(state, current, *value);
                    state.set_register_8(dest, res);
                } else {
                    let current = state.get_register_16(dest);
                    let res = self.kind.execute_word(state, current, *value as u16);
                    state.set_register_16(dest, res);
                }
            }
            (Location::Reg(dest), Location::Immediate16(value)) => {
                if is_byte(dest) {
                    let current = state.get_register_8(dest);
                    let res = self.kind.execute_byte(state, current, *value as u8);
                    state.set_register_8(dest, res);
                } else {
                    let current = state.get_register_16(dest);
                    let res = self.kind.execute_word(state, current, *value);
                    state.set_register_16(dest, res);
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl OpKind {
    pub fn execute_byte(&self, state: &mut SimState, first: u8, second: u8) -> u8 {
        match self {
            OpKind::Add => {
                let result = first.wrapping_add(second);
                state.flags.zero = result == 0;
                state.flags.sign = result & 0x80 != 0;
                result
            }
            OpKind::Sub => {
                let result = first.wrapping_sub(second);
                state.flags.zero = result == 0;
                state.flags.sign = result & 0x80 != 0;
                result
            }
            OpKind::Cmp => {
                let result = first.wrapping_sub(second);
                state.flags.zero = result == 0;
                state.flags.sign = result & 0x80 != 0;
                first
            }
        }
    }

    pub fn execute_word(&self, state: &mut SimState, first: u16, second: u16) -> u16 {
        match self {
            OpKind::Add => {
                let result = first.wrapping_add(second);
                state.flags.zero = result == 0;
                state.flags.sign = result & 0x8000 != 0;
                result
            }
            OpKind::Sub => {
                let result = first.wrapping_sub(second);
                state.flags.zero = result == 0;
                state.flags.sign = result & 0x8000 != 0;
                result
            }
            OpKind::Cmp => {
                let result = first.wrapping_sub(second);
                state.flags.zero = result == 0;
                state.flags.sign = result & 0x8000 != 0;
                first
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{decoder::decode, sim::SimState};

    #[test]
    fn test_add_imm_to_reg() {
        let mut bytes = vec![0b10000011, 0b11000000, 0b1].into_iter();
        let mut state = SimState::default();
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_16("ax"), 1);
        assert_eq!(state.flags.zero, false, "zero flag should be false");
        assert_eq!(state.flags.sign, false, "sign flag should be false");
    }
}
