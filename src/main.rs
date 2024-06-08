use clap::Parser;

use crate::cli::Command;

mod decoder;
#![cfg(not(tarpaulin_include))]
mod cli;

fn main() {
    Command::parse().run()
}
