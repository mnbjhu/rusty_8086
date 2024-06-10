use crate::decoder::{
    loc::{eac::decode_eac, Location},
    mov::decode_reg,
    state::Decoder,
};

pub fn decode_rm_to_from_reg<T: Decoder>(state: &mut T) -> (Location, Location) {
    let first = state.get_byte(0);
    let second = state.get_byte(1);
    state.add_len(2);
    let d = first & 0b00000010;
    let w = first & 0b00000001;
    let reg = (second & 0b000111000) >> 3;
    let reg = decode_reg(w, reg);
    let rm = decode_rm(state);
    if d == 0 {
        (rm, Location::Reg(reg))
    } else {
        (Location::Reg(reg), rm)
    }
}

pub fn decode_rm_to_reg<T: Decoder>(state: &mut T) -> (Location, Location) {
    let first = state.get_byte(0);
    state.add_len(2);
    let sw = first & 0b00000011;
    let rm = decode_rm(state);
    let len = state.get_instr_len();
    let imm = if sw == 0b01 {
        let low = state.get_byte(len);
        let high = state.get_byte(len + 1);
        state.add_len(2);
        Location::Immediate16((high as u16) << 8 | low as u16)
    } else {
        let low = state.get_byte(len);
        state.add_len(1);
        Location::Immediate8(low)
    };

    (rm, imm)
}

pub fn decode_rm<T: Decoder>(state: &mut T) -> Location {
    let first = state.get_byte(0);
    let w = first & 0b00000001;
    let second = state.get_byte(1);
    let mod_ = (second & 0b11000000) >> 6;
    if mod_ != 0b11 {
        if (second & 0b00000111) == 0b110 && mod_ == 0 {
            let low = state.get_byte(2);
            let high = state.get_byte(3);
            state.add_len(2);
            Location::Mem((high as u16) << 8 | low as u16)
        } else {
            Location::Eac(decode_eac(state))
        }
    } else {
        Location::Reg(decode_reg(w, second & 0b000000111))
    }
}
