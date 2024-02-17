use anyhow::Result;
use clap::{crate_version, Parser};

use timecalc_rs::{calculate_total_time, duration_to_str};

#[derive(Parser, Debug)]
#[clap(name = "Timecalc", version = crate_version!())]
#[clap(
    about = "Calculates the total time of a given set of time ranges and intervals
A duration is either a time range or an interval.
Time ranges are specified as `start-end`, e.g. `9-12:30`.
Intervals are specified as `+duration` or `-duration`, e.g. `1h` or `-30m`.

Valid duration units are:
  `w` for weeks
  `d` for days
  `h` for hours
  `m` for minutes"
)]
struct Args {
    #[clap(required = true)]
    #[clap(allow_hyphen_values = true)]
    #[clap(
        help = "Time durations to calculate in time ranges or intervals, e.g. `9-12:30`, `1h` or `-30m`"
    )]
    durations: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let durations = args.durations;
    let total_duration = calculate_total_time(&durations)?;
    let output = duration_to_str(total_duration);
    println!("{}", output);

    Ok(())
}
