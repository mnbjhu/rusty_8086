use std::vec::IntoIter;

use crate::decoder::{loc::Location, mov::decode_reg};

pub fn decode_imm_to_reg(first: u8, bytes: &mut IntoIter<u8>) -> (Location, Location) {
    let w = (first & 0b00001000) >> 3;
    let reg = first & 0b00000111;
    let reg = decode_reg(w, reg);
    if w == 0 {
        let second = bytes.next().unwrap();
        (Location::Reg(reg), Location::Immediate8(second))
    } else {
        let second = bytes.next().unwrap();
        let third = bytes.next().unwrap();
        let second = (third as u16) << 8 | second as u16;
        (Location::Reg(reg), Location::Immediate16(second))
    }
}
