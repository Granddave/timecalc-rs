use chrono::Duration;
use regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    /// Matches a time interval in the format `(-)N[wdhm]`
    static ref DURATION_RE: Regex = Regex::new(r"^(-?\d+)([wdhm])$").unwrap();
    /// Matches a range part in the format `HH:MM`, `H:MM`, `HH`, or `H`
    static ref TIME_INTERVAL_PART_RE: Regex = Regex::new(r"^((?:[0-1]?\d)|(?:2[0-3]))(?::([0-5]\d))?$").unwrap();
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TimecalcError {
    #[error("Failed to parse duration: {0}")]
    ParseError(String),
}

/// Parse an interval in the format `(-)N[wdhm]` and return the duration
/// represented by the interval.
///
/// The interval can be positive or negative, and the unit can be weeks, days,
/// hours, or minutes.
fn parse_duration_arg(arg: &str) -> Option<Duration> {
    match DURATION_RE.captures(arg) {
        Some(captures) => {
            let value: i64 = captures[1]
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse duration: {}", arg));
            let unit = &captures[2];

            match unit {
                "w" => Some(Duration::weeks(value)),
                "d" => Some(Duration::days(value)),
                "h" => Some(Duration::hours(value)),
                "m" => Some(Duration::minutes(value)),
                _ => None,
            }
        }
        None => None,
    }
}

fn parse_time_interval_part(input: &str) -> Option<Duration> {
    match TIME_INTERVAL_PART_RE.captures(input) {
        Some(captures) => {
            let hour = captures[1].parse::<i64>().unwrap();
            let minute = captures
                .get(2)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap_or(0));
            Some(Duration::hours(hour) + Duration::minutes(minute))
        }
        None => None,
    }
}

/// Parse a time interval in the format `HH:MM-HH:MM` and return the duration
/// between the start and end time.
///
/// The start and end time can be in the format `HH:MM`, `HH`, `H:MM`, or `H`.
///
/// Example input:
/// - `08:00-12:00`
/// - `8:00-08:30`
/// - `8-12`
fn parse_time_interval_arg(input: &str) -> Option<Duration> {
    let splits: Vec<&str> = input.split('-').collect();
    if splits.len() != 2 {
        return None;
    }

    let duration = match (
        parse_time_interval_part(splits[0]),
        parse_time_interval_part(splits[1]),
    ) {
        (Some(start), Some(end)) => Some(end - start),
        _ => None,
    };

    duration.filter(|duration| *duration >= Duration::zero())
}

fn parse_arg(arg: &str) -> Result<Duration, TimecalcError> {
    if let Some(interval) = parse_duration_arg(arg) {
        return Ok(interval);
    }

    if let Some(range) = parse_time_interval_arg(arg) {
        return Ok(range);
    }

    Err(TimecalcError::ParseError(arg.to_string()))
}

pub fn parse_args(args: &[&str]) -> Result<Duration, TimecalcError> {
    args.iter()
        .try_fold(Duration::seconds(0), |acc, arg| Ok(acc + parse_arg(arg)?))
}

// TODO: Rename to Jira time format
pub fn duration_to_str(duration: Duration) -> String {
    let weeks = duration.num_weeks();
    let days = duration.num_days() % 7;
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;

    let mut output = String::new();
    if weeks > 0 {
        output.push_str(&format!("{}w ", weeks));
    }
    if days > 0 {
        output.push_str(&format!("{}d ", days));
    }
    if hours > 0 {
        output.push_str(&format!("{}h ", hours));
    }
    if minutes > 0 {
        output.push_str(&format!("{}m", minutes));
    }

    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    #[test]
    fn test_parse_time_interval() -> Result<()> {
        assert_eq!(parse_duration_arg("1w").unwrap(), Duration::weeks(1));
        assert_eq!(parse_duration_arg("2d").unwrap(), Duration::days(2));
        assert_eq!(parse_duration_arg("3h").unwrap(), Duration::hours(3));
        assert_eq!(parse_duration_arg("4m").unwrap(), Duration::minutes(4));
        assert_eq!(parse_duration_arg("-1w").unwrap(), Duration::weeks(-1));
        assert_eq!(parse_duration_arg("-2d").unwrap(), Duration::days(-2));
        assert_eq!(parse_duration_arg("-3h").unwrap(), Duration::hours(-3));
        assert!(parse_duration_arg("1x").is_none());
        Ok(())
    }

    #[test]
    fn test_parse_time_range() -> Result<()> {
        assert_eq!(
            parse_time_interval_arg("08:00-12:00").unwrap(),
            Duration::hours(4)
        );
        assert_eq!(
            parse_time_interval_arg("08:00-08:30").unwrap(),
            Duration::minutes(30)
        );
        assert_eq!(parse_time_interval_arg("8-12").unwrap(), Duration::hours(4));
        assert_eq!(
            parse_time_interval_arg("8:00-09").unwrap(),
            Duration::hours(1)
        );
        assert_eq!(parse_time_interval_arg("8-9").unwrap(), Duration::hours(1));
        Ok(())
    }

    #[test]
    fn test_parse_args() -> Result<()> {
        let args_list = vec!["1w", "2d", "3h", "4m", "08:00-12:00", "08:00-08:30", "8-12"];
        assert_eq!(
            parse_args(&args_list)?,
            Duration::weeks(1)
                + Duration::days(2)
                + Duration::hours(3)
                + Duration::minutes(4)
                + Duration::hours(4)
                + Duration::minutes(30)
                + Duration::hours(4)
        );
        Ok(())
    }

    #[test]
    fn test_timedelta_to_str() {
        assert_eq!(
            duration_to_str(
                Duration::weeks(1) + Duration::days(2) + Duration::hours(3) + Duration::minutes(4)
            ),
            "1w 2d 3h 4m"
        );
        assert_eq!(
            duration_to_str(Duration::weeks(1) + Duration::days(2) + Duration::hours(3)),
            "1w 2d 3h"
        );
        assert_eq!(
            duration_to_str(Duration::weeks(1) + Duration::days(2)),
            "1w 2d"
        );
        assert_eq!(duration_to_str(Duration::weeks(1)), "1w");
        assert_eq!(duration_to_str(Duration::days(2)), "2d");
        assert_eq!(duration_to_str(Duration::hours(3)), "3h");
        assert_eq!(duration_to_str(Duration::minutes(4)), "4m");
    }

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(parse_arg("1w")?, Duration::weeks(1));
        assert_eq!(parse_arg("2d")?, Duration::days(2));
        assert_eq!(parse_arg("3h")?, Duration::hours(3));
        assert_eq!(parse_arg("4m")?, Duration::minutes(4));
        assert_eq!(parse_arg("08:00-12:00")?, Duration::hours(4));
        assert_eq!(parse_arg("08:00-08:30")?, Duration::minutes(30));
        assert_eq!(parse_arg("8-12")?, Duration::hours(4));
        Ok(())
    }
}
