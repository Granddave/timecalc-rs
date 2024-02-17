use anyhow::Result;
use std::env;

use timecalc_rs::{calculate_total_time, timedelta_to_str};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: tc <time range or interval> ...");
        return Err(anyhow::anyhow!("Invalid arguments"));
    }

    let args_list: Vec<&str> = args.iter().map(AsRef::as_ref).collect();
    let total_duration = calculate_total_time(&args_list)?;
    let output = timedelta_to_str(total_duration);
    println!("{}", output);

    Ok(())
}
