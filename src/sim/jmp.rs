use super::SimState;

impl SimState {
    pub fn execute_je(&mut self, offset: i8) {
        if self.flags.zero {
            self.decoder.offset = self.decoder.offset.wrapping_add_signed(offset as isize);
        }
    }

    pub fn execute_jne(&mut self, offset: i8) {
        if !self.flags.zero {
            self.decoder.offset = self.decoder.offset.wrapping_add_signed(offset as isize);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{decoder::mov::BX, sim::SimState};

    #[test]
    fn test_jne() {
        let mut state = SimState::new(vec![
            0b10111001, 0b11, 0b0, 0b10111011, 0b11101000, 0b11, 0b10000011, 0b11000011, 0b1010,
            0b10000011, 0b11101001, 0b1, 0b1110101, 0b11111000,
        ]);
        state.run();
        assert_eq!(state.get_register_16(BX), 1030);
        assert_eq!(state.decoder.offset, 14);
    }
}
