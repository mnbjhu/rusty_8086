use std::vec::IntoIter;

use crate::decoder::instr::{decode_instr, Instr};

pub mod instr;
pub mod loc;
pub mod mov;
pub mod op;

pub fn decode(bytes: &mut IntoIter<u8>) -> Vec<Instr> {
    let mut found = vec![];
    let mut count = 0;
    while let Some(byte) = bytes.next() {
        found.push(decode_instr(byte, bytes, count));
        count += 1;
    }
    found
}
