use std::path::PathBuf;

use crate::{decoder::decode, sim::SimState};

pub fn sim(path: &PathBuf) {
    let bytes = std::fs::read(path).unwrap();
    let found = decode(bytes);
    let mut state = SimState::default();
    println!("start");
    println!("{}", state);
    for instr in found {
        state.execute(&instr);
    }
    println!("end");
    println!("{}", state)
}
