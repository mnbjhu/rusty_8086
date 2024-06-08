use std::vec::IntoIter;

use crate::decoder::loc::{eac::decode_eac, Location};

pub fn decode_imm_to_rm(first: u8, second: u8, bytes: &mut IntoIter<u8>) -> (Location, Location) {
    let w = first & 0b00000001;
    let eac = decode_eac(second, bytes);
    let src = if w == 0 {
        let second = bytes.next().unwrap();
        Location::Immediate8(second)
    } else {
        let second = bytes.next().unwrap();
        let third = bytes.next().unwrap();
        let second = (third as u16) << 8 | second as u16;
        Location::Immediate16(second)
    };
    (Location::Eac(eac), src)
}
