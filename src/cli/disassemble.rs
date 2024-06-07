use std::path::PathBuf;

use crate::decoder::decode;

pub fn disassemble(path: &PathBuf) {
    let mut bytes = std::fs::read(path).unwrap().into_iter();
    let found = decode(&mut bytes);

    println!("bits 16");
    for instr in found {
        println!("{}", instr);
    }
}
