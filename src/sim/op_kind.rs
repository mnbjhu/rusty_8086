use crate::decoder::op::OpKind;

use super::SimState;

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
    use crate::{
        decoder::{
            mov::{AL, AX, BL, BX},
            op::OpKind,
        },
        sim::SimState,
    };

    #[test]
    fn add_to_pos_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 1);
        state.set_register_8(BL, 2);

        let result = OpKind::Add.execute_byte(&mut state, 1, 2);

        assert_eq!(result, 3);

        assert_eq!(state.get_register_8(AL), 1);
        assert_eq!(state.get_register_8(BL), 2);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn add_to_neg_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 0);
        state.set_register_8(BL, 0xFF);

        let result = OpKind::Add.execute_byte(&mut state, 0, 0xFF);

        assert_eq!(result, 0xFF);

        assert_eq!(state.get_register_8(AL), 0);
        assert_eq!(state.get_register_8(BL), 0xFF);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, true);
    }

    #[test]
    fn add_to_zero_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 0);
        state.set_register_8(BL, 0);

        let result = OpKind::Add.execute_byte(&mut state, 0, 0);

        assert_eq!(result, 0);

        assert_eq!(state.get_register_8(AL), 0);
        assert_eq!(state.get_register_8(BL), 0);
        assert_eq!(state.flags.zero, true);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn sub_to_pos_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 3);
        state.set_register_8(BL, 2);

        let result = OpKind::Sub.execute_byte(&mut state, 3, 2);

        assert_eq!(result, 1);

        assert_eq!(state.get_register_8(AL), 3);
        assert_eq!(state.get_register_8(BL), 2);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn sub_to_neg_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 0);
        state.set_register_8(BL, 0xFF);

        let result = OpKind::Sub.execute_byte(&mut state, 0, 0xFF);

        assert_eq!(result, 0x01);

        assert_eq!(state.get_register_8(AL), 0);
        assert_eq!(state.get_register_8(BL), 0xFF);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn sub_to_zero_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 0);
        state.set_register_8(BL, 0);

        let result = OpKind::Sub.execute_byte(&mut state, 0, 0);

        assert_eq!(result, 0);

        assert_eq!(state.get_register_8(AL), 0);
        assert_eq!(state.get_register_8(BL), 0);
        assert_eq!(state.flags.zero, true);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn cmp_to_pos_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 3);
        state.set_register_8(BL, 2);

        let result = OpKind::Cmp.execute_byte(&mut state, 3, 2);

        assert_eq!(result, 3);

        assert_eq!(state.get_register_8(AL), 3);
        assert_eq!(state.get_register_8(BL), 2);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn cmp_to_neg_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 0);
        state.set_register_8(BL, 0xFF);

        let result = OpKind::Cmp.execute_byte(&mut state, 0, 0xFF);

        assert_eq!(result, 0);

        assert_eq!(state.get_register_8(AL), 0);
        assert_eq!(state.get_register_8(BL), 0xFF);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn cmp_to_zero_byte() {
        let mut state = SimState::default();
        state.set_register_8(AL, 0);
        state.set_register_8(BL, 0);

        let result = OpKind::Cmp.execute_byte(&mut state, 0, 0);

        assert_eq!(result, 0);

        assert_eq!(state.get_register_8(AL), 0);
        assert_eq!(state.get_register_8(BL), 0);
        assert_eq!(state.flags.zero, true);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn add_to_pos_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 1);
        state.set_register_16(BX, 2);

        let result = OpKind::Add.execute_word(&mut state, 1, 2);

        assert_eq!(result, 3);

        assert_eq!(state.get_register_16(AX), 1);
        assert_eq!(state.get_register_16(BX), 2);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn add_to_neg_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 0);
        state.set_register_16(BX, 0xFFFF);

        let result = OpKind::Add.execute_word(&mut state, 0, 0xFFFF);

        assert_eq!(result, 0xFFFF);

        assert_eq!(state.get_register_16(AX), 0);
        assert_eq!(state.get_register_16(BX), 0xFFFF);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, true);
    }

    #[test]
    fn add_to_zero_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 0);
        state.set_register_16(BX, 0);

        let result = OpKind::Add.execute_word(&mut state, 0, 0);

        assert_eq!(result, 0);

        assert_eq!(state.get_register_16(AX), 0);
        assert_eq!(state.get_register_16(BX), 0);
        assert_eq!(state.flags.zero, true);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn sub_to_pos_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 3);
        state.set_register_16(BX, 2);

        let result = OpKind::Sub.execute_word(&mut state, 3, 2);

        assert_eq!(result, 1);

        assert_eq!(state.get_register_16(AX), 3);
        assert_eq!(state.get_register_16(BX), 2);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn sub_to_neg_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 0);
        state.set_register_16(BX, 0xFFFF);

        let result = OpKind::Sub.execute_word(&mut state, 0, 0xFFFF);

        assert_eq!(result, 0x0001);

        assert_eq!(state.get_register_16(AX), 0);
        assert_eq!(state.get_register_16(BX), 0xFFFF);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn sub_to_zero_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 0);
        state.set_register_16(BX, 0);

        let result = OpKind::Sub.execute_word(&mut state, 0, 0);

        assert_eq!(result, 0);

        assert_eq!(state.get_register_16(AX), 0);
        assert_eq!(state.get_register_16(BX), 0);
        assert_eq!(state.flags.zero, true);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn cmp_to_pos_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 3);
        state.set_register_16(BX, 2);

        let result = OpKind::Cmp.execute_word(&mut state, 3, 2);

        assert_eq!(result, 3);

        assert_eq!(state.get_register_16(AX), 3);
        assert_eq!(state.get_register_16(BX), 2);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn cmp_to_neg_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 0);
        state.set_register_16(BX, 0xFFFF);

        let result = OpKind::Cmp.execute_word(&mut state, 0, 0xFFFF);

        assert_eq!(result, 0);

        assert_eq!(state.get_register_16(AX), 0);
        assert_eq!(state.get_register_16(BX), 0xFFFF);
        assert_eq!(state.flags.zero, false);
        assert_eq!(state.flags.sign, false);
    }

    #[test]
    fn cmp_to_zero_word() {
        let mut state = SimState::default();
        state.set_register_16(AX, 0);
        state.set_register_16(BX, 0);

        let result = OpKind::Cmp.execute_word(&mut state, 0, 0);

        assert_eq!(result, 0);

        assert_eq!(state.get_register_16(AX), 0);
        assert_eq!(state.get_register_16(BX), 0);
        assert_eq!(state.flags.zero, true);
        assert_eq!(state.flags.sign, false);
    }
}
