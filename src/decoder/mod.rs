use std::vec::IntoIter;

use crate::decoder::instr::Instr;

pub mod instr;
pub mod mov;

use mov::decode_mov;

pub fn dis(bytes: &mut IntoIter<u8>) -> Vec<Instr> {
    let mut found = vec![];
    while let Some(byte) = bytes.next() {
        if let 0b10001000 = byte & 0b11111100 {
            found.push(Instr::Mov(decode_mov(byte, bytes)))
        } else {
            println!("Unknown instruction: {:#10b} ", byte)
        }
    }
    found
}
