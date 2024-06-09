use std::vec::IntoIter;

use crate::decoder::loc::Location;

use super::rm_to_reg::decode_rm;

pub fn decode_imm_to_rm(first: u8, second: u8, bytes: &mut IntoIter<u8>) -> (Location, Location) {
    let w = first & 1;
    let rm = decode_rm(first, second, bytes);
    let src = if w == 0 {
        Location::Immediate8(second)
    } else {
        let third = bytes.next().unwrap();
        let data = (third as u16) << 8 | second as u16;
        Location::Immediate16(data)
    };
    (rm, src)
}
