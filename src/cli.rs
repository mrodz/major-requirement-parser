use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to the input file
    input: PathBuf,
    /// Path to the output file
    #[arg(long, short)]
    output: Option<PathBuf>,
}

impl Args {
    pub fn input(&self) -> &Path {
        self.input.as_path()
    }

    pub fn output(&self) -> Option<&Path> {
        self.output.as_deref()
    }
}
