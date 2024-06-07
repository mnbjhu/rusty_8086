use std::path::PathBuf;

use crate::decoder::dis;

pub fn disassemble(path: &PathBuf) {
    let mut bytes = std::fs::read(path).unwrap().into_iter();
    let found = dis(&mut bytes);

    println!("bits 16");
    for instr in found {
        println!("{}", instr);
    }
}
