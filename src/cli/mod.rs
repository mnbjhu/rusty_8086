use std::path::PathBuf;

use clap::Parser;

use crate::cli::disassemble::disassemble;
use bytes::bytes;

mod bytes;
mod disassemble;
mod sim;

#[derive(Parser)]
#[clap(name = "8085 Sim")]
pub enum Command {
    Disassemble { path: PathBuf },
    Bytes { path: PathBuf },
    Sim { path: PathBuf },
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Disassemble { path } => disassemble(path),
            Command::Bytes { path } => bytes(path),
            Command::Sim { path } => bytes(path),
        }
    }
}
