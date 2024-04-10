[![ci](https://github.com/Granddave/timecalc-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Granddave/timecalc-rs/actions)
[![dependency status](https://deps.rs/repo/github/granddave/timecalc-rs/status.svg)](https://deps.rs/repo/github/granddave/timecalc-rs)
[![Crates.io Version](https://img.shields.io/crates/v/timecalc)](https://crates.io/crates/timecalc)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# timecalc

Timecalc is a simple CLI tool that calculates the total time spent on a project given intervals and durations.

This is the Rust implementation of my [previous tool written in Python](https://github.com/Granddave/timecalc).

Timecalc is available as both a [crate](https://crates.io/crates/timecalc) and as a standalone binary.
See the [installation instructions](#installation) on how to install it,
or the [documentation](https://docs.rs/timecalc) for more information about the crate.


## How to use

```bash
$ timecalc --help
Calculate the total duration of a given set of intervals and durations.

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

### Releases

The latest release can be downloaded as a standalone binary from the [releases page](https://github.com/Granddave/timecalc-rs/releases/latest).

### Using `cargo`

It can also be compiled and installed via `cargo`:

```bash
cargo install timecalc
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

