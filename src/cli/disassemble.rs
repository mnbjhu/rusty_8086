use std::path::PathBuf;

use crate::decoder::decode;

pub fn disassemble(path: &PathBuf) {
    let bytes = std::fs::read(path).unwrap();
    let found = decode(bytes);

    println!("bits 16");
    for instr in found {
        println!("{}", instr);
    }
}
