use std::vec::IntoIter;

use crate::decoder::mov::{decode_reg, Location, MoveInstr};

pub fn decode_mov_im(first: u8, bytes: &mut IntoIter<u8>) -> MoveInstr {
    let w = (first & 0b00001000) >> 3;
    let reg = first & 0b00000111;
    let reg = decode_reg(w, reg);
    if w == 0 {
        let second = bytes.next().unwrap();
        MoveInstr {
            dest: Location::Reg(reg),
            src: Location::Immediate(second.into()),
        }
    } else {
        let second = bytes.next().unwrap();
        let third = bytes.next().unwrap();
        let second = (third as u16) << 8 | second as u16;
        MoveInstr {
            dest: Location::Reg(reg),
            src: Location::Immediate(second),
        }
    }
}
