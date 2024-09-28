use std::path::PathBuf;

use clap::Parser;

// Argument parser, for semplicity sake we're using clap
// instead of the std::env::collect approach
#[derive(Parser, Debug)]
#[command(
    name = "rmtarget", 
    version, about,
    long_about = None,
)]
pub(crate) struct Args {
    #[arg(
        short,
        long,
        help = "Do not print anything to stdout",
        default_value_t = false
    )]
    pub(crate) quiet: bool,
    #[arg(short, long, help = "Directory to remove targets from")]
    pub(crate) dir: PathBuf,
    #[arg(
        short,
        long,
        help = "Target directories to remove",
        use_value_delimiter = true,
        value_delimiter = ',',
        default_value = "target"
    )]
    pub(crate) targets: Vec<PathBuf>,
}
