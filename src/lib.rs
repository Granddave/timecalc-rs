use chrono::Duration;
use regex::Regex;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TimecalcError {
    #[error("Invalid time interval format: '{0}'")]
    InvalidIntervalFormat(String),
    #[error("Invalid time interval unit: '{0}'")]
    InvalidIntervalUnit(String),
    #[error("Invalid time range format: '{0}'")]
    InvalidTimeRangeFormat(String),
    #[error("Invalid time range start time: '{0}'")]
    InvalidTimeRangeStart(String),
    #[error("Invalid time range end time: '{0}'")]
    InvalidTimeRangeEnd(String),
    #[error("Invalid time range: end time must be greater than start time")]
    InvalidTimeRange,
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Parse a time interval in the format `(-)N[wdhm]` and return the duration
/// represented by the interval.
///
/// Example input:
/// - `1w` - 1 week
/// - `2d` - 2 days
/// - `3h` - 3 hours
/// - `4m` - 4 minutes
/// - `-1w` - -1 week
/// - `-2d` - -2 days
/// - `-3h` - -3 hours
/// - `-4m` - -4 minutes
fn parse_time_interval(input: &str) -> Result<Duration, TimecalcError> {
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
                _ => Err(TimecalcError::InvalidIntervalUnit(unit.to_string())),
            }
        }
        None => Err(TimecalcError::InvalidIntervalFormat(input.to_string())),
    }
}

/// Parse a time range in the format `HH:MM-HH:MM` and return the duration
/// between the start and end time.
///
/// The start and end time can be in the format `HH:MM`, `HH`, `H:MM`, or `H`.
///
/// Example input:
/// - `08:00-12:00`
/// - `08:00-08:30`
/// - `8-12`
fn parse_time_range(input: &str) -> Result<Duration, TimecalcError> {
    let split_input: Vec<&str> = input.split('-').collect();
    if split_input.len() != 2 {
        return Err(TimecalcError::InvalidTimeRangeFormat(input.to_string()));
    }

    let re = Regex::new(r"^((?:[0-1]?\d)|(?:2[0-3]))(?::([0-5]\d))?$").unwrap();
    let captures_start = re
        .captures(split_input[0])
        .ok_or_else(|| TimecalcError::InvalidTimeRangeStart(split_input[0].to_string()))?;

    let captures_end = re
        .captures(split_input[1])
        .ok_or_else(|| TimecalcError::InvalidTimeRangeEnd(split_input[1].to_string()))?;

    let start = Duration::hours(captures_start[1].parse::<i64>().unwrap())
        + Duration::minutes(
            captures_start
                .get(2)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap_or(0)),
        );
    let end = Duration::hours(captures_end[1].parse::<i64>().unwrap())
        + Duration::minutes(
            captures_end
                .get(2)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap_or(0)),
        );

    if end <= start {
        Err(TimecalcError::InvalidTimeRange)
    } else {
        Ok(end - start)
    }
}

fn parse(input: &str) -> Result<Duration, TimecalcError> {
    if let Ok(d) = parse_time_interval(input) {
        return Ok(d);
    }

    if let Ok(d) = parse_time_range(input) {
        return Ok(d);
    }

    return Err(TimecalcError::ParseError(input.to_string()));
}

pub fn calculate_total_time(args_list: &[&str]) -> Result<Duration, TimecalcError> {
    let mut total_duration = Duration::seconds(0);

    for input in args_list {
        let duration = parse(input)?;
        total_duration = total_duration + duration;
        continue;
    }

    return Ok(total_duration);
}

pub fn timedelta_to_str(duration: Duration) -> String {
    let weeks = duration.num_weeks();
    let days = duration.num_days() % 7;
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;

    let mut output = String::new();
    if weeks != 0 {
        output.push_str(&format!("{}w ", weeks));
    }
    if days != 0 {
        output.push_str(&format!("{}d ", days));
    }
    if hours != 0 {
        output.push_str(&format!("{}h ", hours));
    }
    if minutes != 0 {
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
        assert_eq!(parse_time_interval("1w")?, Duration::weeks(1));
        assert_eq!(parse_time_interval("2d")?, Duration::days(2));
        assert_eq!(parse_time_interval("3h")?, Duration::hours(3));
        assert_eq!(parse_time_interval("4m")?, Duration::minutes(4));
        assert_eq!(parse_time_interval("-1w")?, Duration::weeks(-1));
        assert_eq!(parse_time_interval("-2d")?, Duration::days(-2));
        assert_eq!(parse_time_interval("-3h")?, Duration::hours(-3));
        assert_eq!(parse_time_interval("-4m")?, Duration::minutes(-4));
        assert_eq!(
            parse_time_interval("1x").unwrap_err(),
            TimecalcError::InvalidIntervalFormat("1x".to_string())
        );
        Ok(())
    }

    #[test]
    fn test_parse_time_range() -> Result<()> {
        assert_eq!(parse_time_range("08:00-12:00")?, Duration::hours(4));
        assert_eq!(parse_time_range("08:00-08:30")?, Duration::minutes(30));
        assert_eq!(parse_time_range("8-12")?, Duration::hours(4));
        assert_eq!(parse_time_range("8:00-09")?, Duration::hours(1));
        assert_eq!(parse_time_range("8-9")?, Duration::hours(1));
        Ok(())
    }

    #[test]
    fn test_calculate_total_time() -> Result<()> {
        let args_list = &["1w", "2d", "3h", "4m", "08:00-12:00", "08:00-08:30", "8-12"];
        assert_eq!(
            calculate_total_time(args_list)?,
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
            timedelta_to_str(
                Duration::weeks(1) + Duration::days(2) + Duration::hours(3) + Duration::minutes(4)
            ),
            "1w 2d 3h 4m"
        );
        assert_eq!(
            timedelta_to_str(Duration::weeks(1) + Duration::days(2) + Duration::hours(3)),
            "1w 2d 3h"
        );
        assert_eq!(
            timedelta_to_str(Duration::weeks(1) + Duration::days(2)),
            "1w 2d"
        );
        assert_eq!(timedelta_to_str(Duration::weeks(1)), "1w");
        assert_eq!(timedelta_to_str(Duration::days(2)), "2d");
        assert_eq!(timedelta_to_str(Duration::hours(3)), "3h");
        assert_eq!(timedelta_to_str(Duration::minutes(4)), "4m");
    }

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(parse("1w")?, Duration::weeks(1));
        assert_eq!(parse("2d")?, Duration::days(2));
        assert_eq!(parse("3h")?, Duration::hours(3));
        assert_eq!(parse("4m")?, Duration::minutes(4));
        assert_eq!(parse("08:00-12:00")?, Duration::hours(4));
        assert_eq!(parse("08:00-08:30")?, Duration::minutes(30));
        assert_eq!(parse("8-12")?, Duration::hours(4));
        Ok(())
    }
}
