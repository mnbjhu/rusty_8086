use crate::decoder::{loc::Location, op::OpInstr};

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
                let current = state.get_register_16(dest);
                let res = self.kind.execute_word(state, current, *value);
                state.set_register_16(dest, res);
            }
            _ => unimplemented!(),
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

    #[test]
    fn test_add_imm_to_reg_byte() {
        let mut bytes = vec![0b10000011, 0b11000000, 0b1].into_iter();
        let mut state = SimState::default();
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_8("al"), 1);
        assert_eq!(state.flags.zero, false, "zero flag should be false");
        assert_eq!(state.flags.sign, false, "sign flag should be false");
    }

    #[test]
    fn test_add_imm_to_reg_byte_plus() {
        let mut bytes = vec![0b101, 0b11101000, 0b11].into_iter();
        let mut state = SimState::default();
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_16("ax"), 1000);
        assert_eq!(state.flags.zero, false, "zero flag should be false");
        assert_eq!(state.flags.sign, false, "sign flag should be false");
    }

    #[test]
    fn test_add_reg_to_reg_byte() {
        let mut bytes = vec![0b100, 0b1].into_iter();
        let mut state = SimState::default();
        state.set_register_8("bl", 1);
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_8("al"), 1);
        assert_eq!(state.flags.zero, false, "zero flag should be false");
        assert_eq!(state.flags.sign, false, "sign flag should be false");
    }

    #[test]
    fn test_add_reg_to_reg_byte_twice() {
        let mut bytes = vec![0b0, 0b11011000, 0b0, 0b11011000].into_iter();
        let mut state = SimState::default();
        state.set_register_8("bl", 1);
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_8("al"), 2);
        assert_eq!(state.flags.zero, false, "zero flag should be false");
        assert_eq!(state.flags.sign, false, "sign flag should be false");
    }

    #[test]
    fn test_add_reg_to_reg_word() {
        let mut bytes = vec![0b1, 0b11011000].into_iter();
        let mut state = SimState::default();
        state.set_register_16("bx", 1);
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_16("ax"), 1);
        assert_eq!(state.flags.zero, false, "zero flag should be false");
        assert_eq!(state.flags.sign, false, "sign flag should be false");
    }

    #[test]
    fn test_add_reg_to_reg_word_twice() {
        let mut bytes = vec![0b1, 0b11011000, 0b1, 0b11011000].into_iter();
        let mut state = SimState::default();
        state.set_register_16("bx", 1);
        decode(&mut bytes).into_iter().for_each(|instr| {
            state.execute(&instr);
        });
        assert_eq!(state.get_register_16("ax"), 2);
        assert_eq!(state.flags.zero, false, "zero flag should be false");
        assert_eq!(state.flags.sign, false, "sign flag should be false");
    }
}
