mod cli;

use anyhow::{Context, Result, bail};
use clap::Parser as ClapParser;

use std::io::Read;
use std::time::Instant;
use std::{fs::File, io::Write};

use libmql;

const EXTENSION: &str = "mql";

fn main() -> Result<()> {
    let start = Instant::now();

    let cli = cli::Args::parse();

    let input = cli.input();
    let output = cli.output();

    let Some(EXTENSION) = input.extension().and_then(|s| s.to_str()) else {
        bail!("the Major Query Language uses the .mql file extension")
    };

    if let Some(output_path) = output {
        let Some("json") = output_path.extension().and_then(|s| s.to_str()) else {
            bail!("the Major Query Language can only output to a .json file")
        };
    }

    let mut input_file = File::open(input).context("could not open file")?;

    let mut buf = String::new();

    input_file
        .read_to_string(&mut buf)
        .context("file was not UTF-8")?;

    let result = libmql::parse(&buf)?;

    if let Some(output_path) = cli.output() {
        let mut output_file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output_path)
            .context("could not get handle to output file")?;

        output_file
            .write(result.to_string_pretty()?.as_bytes())
            .context("could not write to output file")?;

        let elapsed_time = start.elapsed();

        println!(
            "output to {:?} in {}ms",
            output_path,
            elapsed_time.as_millis()
        )
    } else {
        println!("{}", result.to_string_pretty()?);
    }

    Ok(())
}
