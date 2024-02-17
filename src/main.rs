use anyhow::Result;
use clap::{crate_version, Parser};

use timecalc_rs::{calculate_total_time, duration_to_str};

#[derive(Parser, Debug)]
#[clap(name = "Timecalc", version = crate_version!())]
struct Args {
    #[clap(required = true)]
    #[clap(help = "Time durations to calculate, e.g. 1w 2d 3h 4m -15m 08:00-12:00 12:30-16")]
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
