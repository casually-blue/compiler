#![feature(box_syntax)]

use std::error::Error;

/// Main Program
fn main() -> Result<(), Box<dyn Error>> {
    let args = compiler::cli::Cli::init();

    let config = compiler::config::Config::load(&args)?;

    compiler::Compiler::compile(config)
}
