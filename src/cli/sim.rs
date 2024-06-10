use std::path::PathBuf;

use crate::sim::SimState;

pub fn sim(path: &PathBuf, output: &Option<PathBuf>, trace: bool) {
    let bytes = std::fs::read(path).unwrap();
    let mut state = SimState::new(bytes);
    println!("start");
    println!("{}", state);
    if trace {
        state.run_trace();
    } else {
        state.run();
    }
    println!("end");
    println!("{}", state);
    if let Some(output) = output {
        println!("Writing memory to {}", output.to_string_lossy());
        state.write_memory(output);
    }
}
