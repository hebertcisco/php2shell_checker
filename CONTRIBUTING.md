# Contributing

Thank you for contributing to `php2shell_checker`.

## Development requirements

- Rust stable with the repository MSRV of 1.80;
- `cargo`, `rustfmt`, and Clippy;
- no network access during scanner execution;
- no execution of scanned PHP or project code.

## Before opening a change

Run:

```sh
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

## Adding a detector

1. Define or extend domain types in `src/domain`.
2. Put orchestration and detector logic in `src/application`.
3. Keep filesystem and parser concerns in `src/infrastructure`.
4. Return deterministic findings with stable codes and evidence.
5. Add fixtures and tests for positive, negative, malformed, and conflicting inputs.
6. Document static-analysis limitations.

## Security-sensitive changes

Do not add exploit payloads, network access to normal scans, or execution of repository code. Report vulnerabilities privately using the process in [SECURITY.md](SECURITY.md).

## Pull requests

Keep changes focused, explain the security impact, and include test evidence. Do not commit generated build artifacts.
