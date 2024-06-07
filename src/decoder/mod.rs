use std::vec::IntoIter;

use crate::decoder::instr::{decode_instr, Instr};

pub mod instr;
pub mod mov;

pub fn decode(bytes: &mut IntoIter<u8>) -> Vec<Instr> {
    let mut found = vec![];
    while let Some(byte) = bytes.next() {
        found.push(decode_instr(byte, bytes));
    }
    found
}
