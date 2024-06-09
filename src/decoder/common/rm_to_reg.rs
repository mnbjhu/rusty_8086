use std::vec::IntoIter;

use crate::decoder::{
    loc::{eac::decode_eac, Location},
    mov::decode_reg,
};

pub fn decode_rm_to_from_reg(first: u8, bytes: &mut IntoIter<u8>) -> (Location, Location) {
    let second = bytes.next().unwrap();
    let d = first & 0b00000010;
    let w = first & 0b00000001;
    let reg = (second & 0b000111000) >> 3;
    let reg = decode_reg(w, reg);
    let rm = decode_rm(first, second, bytes);

    if d == 0 {
        (rm, Location::Reg(reg))
    } else {
        (Location::Reg(reg), rm)
    }
}

pub fn decode_rm(first: u8, second: u8, bytes: &mut IntoIter<u8>) -> Location {
    let w = first & 0b00000001;
    let mod_ = (second & 0b11000000) >> 6;
    if mod_ != 0b11 {
        if (second & 0b00000111) == 0b110 && mod_ == 0 {
            if w == 0 {
                Location::Mem(bytes.next().unwrap() as u16)
            } else {
                let low = bytes.next().unwrap();
                let high = bytes.next().unwrap();
                Location::Mem((high as u16) << 8 | low as u16)
            }
        } else {
            Location::Eac(decode_eac(second, bytes))
        }
    } else {
        Location::Reg(decode_reg(w, second & 0b000000111))
    }
}

pub fn decode_rm_to_reg(first: u8, second: u8, bytes: &mut IntoIter<u8>) -> (Location, Location) {
    let w = first & 0b00000001;
    let sw = first & 0b00000011;

    let mod_ = (second & 0b11000000) >> 6;
    let rm = if mod_ != 0b11 {
        if (second & 0b00000111) == 0b110 && mod_ == 0 {
            if sw != 0b01 {
                Location::Mem(bytes.next().unwrap() as u16)
            } else {
                let low = bytes.next().unwrap();
                let high = bytes.next().unwrap();
                Location::Mem((high as u16) << 8 | low as u16)
            }
        } else {
            Location::Eac(decode_eac(second, bytes))
        }
    } else {
        Location::Reg(decode_reg(w, second & 0b000000111))
    };

    let imm = if sw == 0b01 {
        let low = bytes.next().unwrap();
        let high = bytes.next().unwrap();
        Location::Immediate16((high as u16) << 8 | low as u16)
    } else {
        let low = bytes.next().unwrap();
        Location::Immediate8(low)
    };

    (rm, imm)
}
