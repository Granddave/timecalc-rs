use anyhow::Result;
use clap::{crate_version, Parser};
use std::io::{self, BufRead};

use timecalc_rs::{duration_to_str, parse_args};

#[derive(Parser, Debug)]
#[clap(
    name = "Timecalc",
    version = crate_version!(),
    about = "Calculates the total duration of a given set of time intervals and durations
Intervals are specified as `start-end`, e.g. `9-12:30`.
Durations are specified as `+duration` or `-duration`, e.g. `1h` or `-30m`.

Valid duration units are:
  w for weeks
  d for days
  h for hours
  m for minutes"
)]
struct Cli {
    #[clap(
        help = "Durations to calculate in time intervals or durations, e.g. 9-12:30, 1h or -30m",
        allow_hyphen_values = true,
        required_unless_present = "stdin"
    )]
    duration_args: Vec<String>,
    #[clap(
        help = "Read duration arguments from stdin",
        short,
        long,
        conflicts_with = "duration_args"
    )]
    stdin: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let duration_args = if cli.stdin {
        let input = io::stdin()
            .lock()
            .lines()
            .next()
            .expect("Read from stdin failed")?;
        input.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        cli.duration_args
    };
    let duration_args: Vec<&str> = duration_args.iter().map(|s| s.as_str()).collect();
    let total_duration = parse_args(&duration_args)?;
    let output = duration_to_str(total_duration);
    println!("{}", output);

    Ok(())
}
