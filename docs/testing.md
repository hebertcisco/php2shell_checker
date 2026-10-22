# Testing guide

## Local checks

```sh
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

## Test layers

Parser tests should cover valid, malformed, missing, and conflicting source files, including confidence and environment metadata.

Detector tests should cover vulnerable and fixed versions, Composer ranges, floating tags, unknown versions, EOL branches, and conflicts.

Report tests should verify stable ordering, finding codes, evidence locations, JSON serialization, and exit-code semantics.

Repository fixtures belong under `tests/fixtures/`. They must be deterministic, safe to inspect, and free of exploit payloads or secrets.

Future parallel walking and benchmarking must preserve deterministic output ordering and enforce file-size, symlink, and path-boundary limits.
