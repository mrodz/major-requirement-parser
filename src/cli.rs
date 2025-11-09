use std::path::{Path, PathBuf};

use clap::Parser;

const CLI_LONG_ABOUT: &str = r#"

 /$$      /$$  /$$$$$$  /$$      
| $$$    /$$$ /$$__  $$| $$      
| $$$$  /$$$$| $$  \ $$| $$      
| $$ $$/$$ $$| $$  | $$| $$      
| $$  $$$| $$| $$  | $$| $$      
| $$\  $ | $$| $$/$$ $$| $$      
| $$ \/  | $$|  $$$$$$/| $$$$$$$$
|__/     |__/ \____ $$$|________/
                   \__/          
                                 
Transform a Major Requirements file (.mql) to its corresponding JSON.
This is useful when drawing up the requirements for the permutations 
of the ~80 Yale majors.

* You can find the .mql specification at 
  https://github.com/mrodz/major-requirement-parser/tree/main
* Check out MajorAudit at
  https://github.com/majoraudit"#;

#[derive(Parser, Debug)]
#[command(version, about, long_about = CLI_LONG_ABOUT)]
pub(crate) struct Args {
    /// Path to the input file
    input: PathBuf,
    /// Path to the output file [Default: output to `stdout`]
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
