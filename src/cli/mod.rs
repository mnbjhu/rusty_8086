use std::path::PathBuf;

use clap::{Args, Parser};

use crate::cli::disassemble::disassemble;
use bytes::bytes;

use self::sim::sim;

mod bytes;
mod disassemble;
mod sim;

#[derive(Parser)]
#[clap(name = "8085 Sim")]
pub enum Command {
    Disassemble { path: PathBuf },
    Bytes { path: PathBuf },
    Sim(SimArgs),
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Disassemble { path } => disassemble(path),
            Command::Bytes { path } => bytes(path),
            Command::Sim(SimArgs {
                path,
                output,
                trace,
            }) => sim(path, output, *trace),
        }
    }
}

#[derive(Args)]
pub struct SimArgs {
    #[clap(short, long)]
    pub path: PathBuf,
    #[clap(short, long)]
    pub output: Option<PathBuf>,
    #[clap(short, long)]
    pub trace: bool,
}
