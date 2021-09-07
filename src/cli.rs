use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    pub input_file: std::path::PathBuf,
}


impl Cli {
    pub fn init() -> Self {
        Cli::from_args()
    }
}
