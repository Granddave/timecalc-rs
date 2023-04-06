use chrono::Duration;
use regex::Regex;
use std::env;

fn parse_time_interval(input: &str) -> Result<Duration, String> {
    let re = Regex::new(r"^(-?\d+)([wdhm])$").unwrap();
    match re.captures(input) {
        Some(captures) => {
            let value: i64 = captures[1].parse().unwrap();
            let unit = &captures[2];

            match unit {
                "w" => Ok(Duration::weeks(value)),
                "d" => Ok(Duration::days(value)),
                "h" => Ok(Duration::hours(value)),
                "m" => Ok(Duration::minutes(value)),
                _ => Err(format!("Invalid unit: '{}'", unit)),
            }
        }
        None => Err(format!("Invalid time interval format: '{}'", input)),
    }
}

fn parse_time_range(input: &str) -> Result<Duration, String> {
    let split_input: Vec<&str> = input.split('-').collect();
    if split_input.len() != 2 {
        return Err(format!("Invalid time range format: {}", input));
    }

    let re = Regex::new(r"^((?:[0-1]?\d)|(?:2[0-3]))(?::([0-5]\d))?$").unwrap();
    let captures_start = re
        .captures(split_input[0])
        .ok_or_else(|| format!("Invalid start time format: {}", split_input[0]))?;
    let captures_end = re
        .captures(split_input[1])
        .ok_or_else(|| format!("Invalid end time format: {}", split_input[1]))?;

    // println!("{:?}", captures_start);
    // println!("{:?}", captures_end);

    let start = Duration::hours(captures_start[1].parse::<i64>().unwrap())
        + Duration::minutes(
            captures_start
                .get(2)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap_or(0)),
        );
    let end = Duration::hours(captures_end[1].parse::<i64>().unwrap())
        + Duration::minutes(
            captures_start
                .get(2)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap_or(0)),
        );

    if end <= start {
        Err(format!(
            "Invalid time range: end time must be greater than start time"
        ))
    } else {
        Ok(end - start)
    }
}

fn parse(input: &str) -> Result<Duration, String> {
    if let Ok(d) = parse_time_interval(input) {
        return Ok(d);
    }

    if let Ok(d) = parse_time_range(input) {
        return Ok(d);
    }

    return Err(format!("Failed to parse: '{}'", input));
}

fn calculate_total_time(args_list: &[&str]) -> Result<Duration, String> {
    let mut total_duration = Duration::seconds(0);

    for input in args_list {
        let duration = parse(input)?;
        total_duration = total_duration + duration;
        continue;
    }

    return Ok(total_duration);
}

fn timedelta_to_str(duration: Duration) -> String {
    let weeks = duration.num_weeks();
    let days = duration.num_days() % 7;
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;

    format!("{}w {}d {}h {}m", weeks, days, hours, minutes)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Usage: tc <time range or interval> ...");
        return;
    }

    let args_list: Vec<&str> = args.iter().map(AsRef::as_ref).collect();
    match calculate_total_time(&args_list) {
        Ok(total_duration) => {
            let output = timedelta_to_str(total_duration);
            println!("{}", output);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
