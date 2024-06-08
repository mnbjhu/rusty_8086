use clap::Parser;

use crate::cli::Command;

#[cfg(not(tarpaulin_include))]
mod cli;
mod decoder;

#[cfg(not(tarpaulin_include))]
fn main() {
    Command::parse().run()
}
