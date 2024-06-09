use std::path::PathBuf;

use crate::sim::SimState;

pub fn sim(path: &PathBuf) {
    let bytes = std::fs::read(path).unwrap();
    let mut state = SimState::new(bytes);
    println!("start");
    println!("{}", state);
    state.run();
    println!("end");
    println!("{}", state)
}
