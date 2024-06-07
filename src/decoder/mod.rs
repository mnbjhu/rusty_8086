use std::vec::IntoIter;

use crate::decoder::{instr::Instr, mov_im::decode_mov_im};

pub mod instr;
pub mod mov;
pub mod mov_im;

use mov::decode_mov;

pub fn dis(bytes: &mut IntoIter<u8>) -> Vec<Instr> {
    let mut found = vec![];
    while let Some(byte) = bytes.next() {
        found.push(decode_instr(byte, bytes));
    }
    found
}

pub fn decode_instr(byte: u8, bytes: &mut IntoIter<u8>) -> Instr {
    if 0b10001000 == byte & 0b11111100 {
        Instr::Mov(decode_mov(byte, bytes))
    } else if 0b10110000 == byte & 0b11110000 {
        Instr::Mov(decode_mov_im(byte, bytes))
    } else {
        panic!("Unknown instruction: {:#10b} ", byte)
    }
}
