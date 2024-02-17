# Timecalc-rs

Simple CLI tool that calculates the total time spent on a project given time ranges and intervals.

This is the Rust implementation of my [previous tool written in Python](https://github.com/Granddave/timecalc)

## How to use

```bash
$ ./timecalc --help
Calculates the total time of a given set of time ranges and intervals
A duration is either a time range or an interval.
Time ranges are specified as `start-end`, e.g. `9-12:30`.
Intervals are specified as `+duration` or `-duration`, e.g. `1h` or `-30m`.

Valid duration units are:
  `w` for weeks
  `d` for days
  `h` for hours
  `m` for minutes

Usage: timecalc <DURATIONS>...

Arguments:
  <DURATIONS>...  Time durations to calculate in time ranges or intervals, e.g. `9-12:30`, `1h` or `-30m`

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Let's say that you work the whole morning and have a daily standup for 15 minutes and a coffe break
for 30 minutes. In the afternoon you work two hours:

```bash
$ timecalc 7:00-12:00 -15m -30m 2h
6h 15m
```

## Installation

```bash
cargo install --git https://github.com/Granddave/timecalc-rs
```

