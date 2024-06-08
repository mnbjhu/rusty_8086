use std::path::PathBuf;

use crate::{decoder::decode, sim::SimState};

pub fn sim(path: &PathBuf) {
    let mut bytes = std::fs::read(path).unwrap().into_iter();
    let found = decode(&mut bytes);
    let mut state = SimState::new();
    println!("start");
    println!("{}", state);
    for instr in found {
        state.execute(&instr);
    }
    println!("end");
    println!("{}", state)
}
