use std::{fmt::Display, vec::IntoIter};

use crate::decoder::mov::{decode_mov, immediate::decode_mov_im, MoveInstr};

use super::mov::acc::{decode_acc_to_mem, decode_mem_to_acc};

#[derive(Debug, PartialEq)]
pub enum Instr {
    Mov(MoveInstr),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Instr::Mov(instr) = self;
        write!(f, "{}", instr)
    }
}

pub fn decode_instr(byte: u8, bytes: &mut IntoIter<u8>) -> Instr {
    match byte {
        _ if 0b10001000 == byte & 0b11111100 => Instr::Mov(decode_mov(byte, bytes)),
        _ if 0b10110000 == byte & 0b11110000 => Instr::Mov(decode_mov_im(byte, bytes)),
        _ if 0b10100000 == byte & 0b11111110 => Instr::Mov(decode_mem_to_acc(byte, bytes)),
        _ if 0b10100010 == byte & 0b11111110 => Instr::Mov(decode_acc_to_mem(byte, bytes)),
        _ => panic!("Unknown instruction: {:#10b} ", byte),
    }
}
