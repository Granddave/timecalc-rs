# Release process

## Prerequisites

Install Cargo release

```bash
cargo install cargo-release
```

## Actions

1. Start of with a clean repo
2. Dry-run the release for the next version
  - `cargo release patch`
  - `cargo release minor`
  - `cargo release major`
3. Release the next version by providing the `-x` flag
  - `cargo release patch -x`
  - `cargo release minor -x`
  - `cargo release major -x`
4. Wait for the CI to build the binaries and create a release draft on GitHub
5. Fill out the release notes
6. Publish the release on GitHub
  - This will trigger the CD-pipeline to publish the crate to crates.io
