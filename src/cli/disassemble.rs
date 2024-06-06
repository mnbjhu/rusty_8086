use std::path::PathBuf;

use crate::decoder::{instr::Instr, mov::decode_mov};

pub fn disassemble(path: &PathBuf) {
    let bytes = std::fs::read(path).unwrap();
    let instrs = bytes.chunks(2);
    let mut found = vec![];
    for instr in instrs {
        let first = instr.first().unwrap();
        let second = instr.get(1).unwrap();
        match first & second {
            0b10001000 => found.push(Instr::Mov(decode_mov(*first, *second))),
            _ => {}
        }
    }

    println!("bits 16");
    for instr in found {
        println!("{}", instr);
    }
}
