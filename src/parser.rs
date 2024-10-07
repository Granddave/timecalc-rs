use chrono::Duration;
use lazy_static::lazy_static;
use regex::Regex;

use crate::TimecalcError;

lazy_static! {
    /// Matches a durataion in the format `(-)N[wdhm]`
    static ref DURATION_RE: Regex = Regex::new(r"^(-?\d+)([wdhm])$").unwrap();
    /// Matches an interval part in the format `HH:MM`, `H:MM`, `HH`, or `H`
    static ref TIME_INTERVAL_PART_RE: Regex = Regex::new(r"^((?:[0-1]?\d)|(?:2[0-3]))(?::([0-5]\d))?$").unwrap();
}

/// Parse a duration in the format `(-)N[wdhm]` and return the duration represented by the
/// duration.
///
/// The duration can be positive or negative, and the unit can be weeks, days, hours, or minutes.
fn parse_duration_expression(arg: &str) -> Option<Duration> {
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

fn parse_interval_part(input: &str) -> Option<Duration> {
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

/// Parse an interval in the format `HH:MM-HH:MM` and return the duration between the start and end
/// time.
///
/// The start and end time can be in the format `HH:MM`, `HH`, `H:MM`, or `H`.
///
/// Example input:
/// - `08:00-12:00`
/// - `8:00-08:30`
/// - `8-12`
pub(crate) fn parse_interval_expression(input: &str) -> Option<Duration> {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 2 {
        return None;
    }

    let duration = match (parse_interval_part(parts[0]), parse_interval_part(parts[1])) {
        (Some(start), Some(end)) => Some(end - start),
        _ => None,
    };

    duration.filter(|duration| *duration >= Duration::zero())
}

/// Parse a duration expression or interval expression and return the [Duration] it represents.
pub fn parse_expression(arg: &str) -> Result<Duration, TimecalcError> {
    if let Some(duration) = parse_duration_expression(arg) {
        return Ok(duration);
    }

    if let Some(interval) = parse_interval_expression(arg) {
        return Ok(interval);
    }

    Err(TimecalcError::ParseError(arg.to_string()))
}

/// Parse a list of duration expressions or interval expressions and return the total [Duration]
/// they represent.
pub fn parse_expressions(args: &[&str]) -> Result<Duration, TimecalcError> {
    args.iter().try_fold(Duration::seconds(0), |acc, arg| {
        Ok(acc + parse_expression(arg)?)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    #[test]
    fn test_parse_durations() -> Result<()> {
        assert_eq!(parse_duration_expression("1w").unwrap(), Duration::weeks(1));
        assert_eq!(parse_duration_expression("2d").unwrap(), Duration::days(2));
        assert_eq!(parse_duration_expression("3h").unwrap(), Duration::hours(3));
        assert_eq!(
            parse_duration_expression("4m").unwrap(),
            Duration::minutes(4)
        );
        assert_eq!(
            parse_duration_expression("-1w").unwrap(),
            Duration::weeks(-1)
        );
        assert_eq!(
            parse_duration_expression("-2d").unwrap(),
            Duration::days(-2)
        );
        assert_eq!(
            parse_duration_expression("-3h").unwrap(),
            Duration::hours(-3)
        );
        assert!(parse_duration_expression("1x").is_none());
        Ok(())
    }

    #[test]
    fn test_parse_interval() -> Result<()> {
        assert_eq!(
            parse_interval_expression("08:00-12:00").unwrap(),
            Duration::hours(4)
        );
        assert_eq!(
            parse_interval_expression("08:00-08:30").unwrap(),
            Duration::minutes(30)
        );
        assert_eq!(
            parse_interval_expression("8-12").unwrap(),
            Duration::hours(4)
        );
        assert_eq!(
            parse_interval_expression("8:00-09").unwrap(),
            Duration::hours(1)
        );
        assert_eq!(
            parse_interval_expression("8-9").unwrap(),
            Duration::hours(1)
        );
        Ok(())
    }

    #[test]
    fn test_parse_expressions() -> Result<()> {
        let args_list = vec!["1w", "2d", "3h", "4m", "08:00-12:00", "08:00-08:30", "8-12"];
        assert_eq!(
            parse_expressions(&args_list)?,
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
    fn test_parse() -> Result<()> {
        assert_eq!(parse_expression("1w")?, Duration::weeks(1));
        assert_eq!(parse_expression("2d")?, Duration::days(2));
        assert_eq!(parse_expression("3h")?, Duration::hours(3));
        assert_eq!(parse_expression("4m")?, Duration::minutes(4));
        assert_eq!(parse_expression("08:00-12:00")?, Duration::hours(4));
        assert_eq!(parse_expression("08:00-08:30")?, Duration::minutes(30));
        assert_eq!(parse_expression("8-12")?, Duration::hours(4));
        Ok(())
    }
}
