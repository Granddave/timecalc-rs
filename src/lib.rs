//! timecalc is a simple library to calculate time expressions.
//!
//! # Example
//!
//! ```
//! use timecalc::{formatter::duration_to_str, parser::parse_args};
//! use chrono::Duration;
//!
//! let args = vec!["1h", "2h", "3h"];
//! let total_duration = parse_args(&args).unwrap();
//! let output = duration_to_str(total_duration);
//! assert_eq!(output, "6h");
//! ```

/// Formatter module
///
/// Contains functions to format durations.
pub mod formatter;

/// Parser module
///
/// Contains functions to parse arguments.
pub mod parser;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TimecalcError {
    #[error("Failed to parse duration: {0}")]
    ParseError(String),
}
