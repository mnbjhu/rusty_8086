use std::path::PathBuf;

use clap::Parser;

use crate::cli::disassemble::disassemble;
use bytes::bytes;

mod bytes;
mod disassemble;

#[derive(Parser)]
#[clap(name = "8085 Sim")]
pub enum Command {
    Disassemble { path: PathBuf },
    Bytes { path: PathBuf },
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Disassemble { path } => disassemble(path),
            Command::Bytes { path } => bytes(path),
        }
    }
}
