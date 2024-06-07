use std::path::PathBuf;

pub fn bytes(path: &PathBuf) {
    let bytes = std::fs::read(path).unwrap().into_iter();
    for byte in bytes {
        println!("{:#10b} ", byte);
    }
}
