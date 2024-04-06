![ci](https://github.com/Granddave/timecalc-rs/actions/workflows/ci.yml/badge.svg)

# Timecalc-rs

Timecalc-rs is a simple CLI tool that calculates the total time spent on a project given intervals and durations.

This is the Rust implementation of my [previous tool written in Python](https://github.com/Granddave/timecalc)

## How to use

```bash
$ timecalc --help
Calculates the total duration of a given set of intervals and durations.

Usage: timecalc [OPTIONS] [DURATION_ARGS]...

Arguments:
  [DURATION_ARGS]...  Intervals or durations to sum

                      Intervals are specified as `start-end`, e.g. `9-12:30`.
                      Durations are specified as `+duration` or `-duration`, e.g. `1h` or `-30m`.

                      Valid duration units are:
                        w for weeks
                        d for days
                        h for hours
                        m for minutes

Options:
  -s, --stdin    Read arguments from stdin
  -h, --help     Print help
  -V, --version  Print version

```

### Example

Let's say you worked on a project from 7:00 to 11:30, including a 15 minute break, and then worked another 4 hours after lunch.
You can calculate the total time spent on the project like this:

```bash
$ timecalc 7-11:30 -15m 4h
8h 15m
# or via stdin
$ echo 7-11:30 -15m 4h | timecalc -s
8h 15m
```

## Installation

The easiest way to install timecalc-rs is using `cargo`:

```bash
cargo install --git https://github.com/Granddave/timecalc-rs
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

