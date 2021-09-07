pub mod cli;
pub mod config;

use std::error::Error;

pub struct Compiler {

}

impl Compiler {
    pub fn compile(_config: config::Config) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
