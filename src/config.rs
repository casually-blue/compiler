use crate::cli;

use std::fs::File;

pub struct Config {
    pub input_file: File,
    pub output_file: File,
}

impl Config {
    pub fn load(args: &cli::Cli) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            input_file: File::open(args.input_file.clone())?,
            output_file: {
                let mut out_name = args.input_file.clone();
                out_name.set_extension("s");
                File::create(out_name)?
            },
        })
    }
}
