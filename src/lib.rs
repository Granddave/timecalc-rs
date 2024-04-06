//! timecalc is a simple library to calculate duration expressions into a total [duration](chrono::Duration).
//!
//! # Duration expressions
//!
//! Duration expressions can be specified as [intervals](#intervals) and [durations](#durations).
//!
//! ## Intervals
//! Intervals are specified as `start-end`, e.g. `9-12:30`.
//!
//! ## Durations
//! Durations are specified as `+duration` or `-duration`, e.g. `1h` or `-30m`.
//!
//! Valid duration units are:
//! - `w` for weeks
//! - `d` for days
//! - `h` for hours
//! - `m` for minutes
//!
//! # Example
//!
//! ```
//! use timecalc::{formatter::duration_to_str, parser::parse_expressions};
//!
//! let args = vec!["1h", "2h", "-30m"];
//! let total_duration = parse_expressions(&args).unwrap();
//! let output = duration_to_str(total_duration);
//! assert_eq!(output, "2h 30m");
//! ```

/// Format a [duration](chrono::Duration) into a [String].
pub mod formatter;

/// Parse duration expressions into a total [duration](chrono::Duration).
///
/// ## Example
/// ```
/// use timecalc::parser::{parse_expression, parse_expressions};
///
/// let arg = "1h";
/// let duration = parse_expression(arg).unwrap();
/// assert_eq!(duration.num_minutes(), 60);
///
/// let args = vec!["2h", "-30m"];
/// let total_duration = parse_expressions(&args).unwrap();
/// assert_eq!(total_duration.num_minutes(), 90);
/// ```
pub mod parser;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TimecalcError {
    #[error("Failed to parse duration: {0}")]
    ParseError(String),
}
