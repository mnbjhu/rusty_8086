use clap::Parser;

use crate::cli::Command;

mod cli;
mod decoder;

fn main() {
    Command::parse().run()
}
