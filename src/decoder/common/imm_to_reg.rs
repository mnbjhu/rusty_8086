use crate::decoder::{loc::Location, mov::decode_reg, state::DecoderState};

pub fn decode_imm_to_reg(state: &mut DecoderState) -> (Location, Location) {
    let first = state.get_byte(0);
    state.add_len(1);
    let w = (first & 0b00001000) >> 3;
    let reg = first & 0b00000111;
    let reg = decode_reg(w, reg);
    if w == 0 {
        let second = state.get_byte(1);
        state.add_len(1);
        (Location::Reg(reg), Location::Immediate8(second))
    } else {
        let second = state.get_byte(1);
        let third = state.get_byte(2);
        state.add_len(2);
        let second = (third as u16) << 8 | second as u16;
        (Location::Reg(reg), Location::Immediate16(second))
    }
}
