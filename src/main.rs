use anyhow::Result;
use clap::{crate_version, Parser};
use std::io::{self, BufRead};

use timecalc::{formatter::duration_to_str, parser::parse_args};

#[derive(Parser, Debug)]
#[clap(
    name = "timecalc",
    version = crate_version!(),
    about = "Calculate the total duration of a given set of intervals and durations."
)]
struct Cli {
    #[clap(
        help = "Intervals or durations to sum

Intervals are specified as `start-end`, e.g. `9-12:30`.
Durations are specified as `+duration` or `-duration`, e.g. `1h` or `-30m`.

Valid duration units are:
  w for weeks
  d for days
  h for hours
  m for minutes",
        allow_hyphen_values = true,
        required_unless_present = "stdin"
    )]
    duration_args: Vec<String>,
    #[clap(
        help = "Read arguments from stdin",
        short,
        long,
        conflicts_with = "duration_args"
    )]
    stdin: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let args = if cli.stdin {
        let input = io::stdin()
            .lock()
            .lines()
            .next()
            .expect("Read from stdin failed")?;
        input.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        cli.duration_args
    };
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let total_duration = parse_args(&args)?;
    let output = duration_to_str(total_duration);
    println!("{}", output);

    Ok(())
}
