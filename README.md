# php2shell_checker

`php2shell_checker` is a security-focused static analyzer for PHP runtime exposure and configuration risk.

It analyzes repository files only. It never executes scanned PHP code and makes no network requests during a normal scan.

## Status

This project is in early development. The current implementation provides:

- PHP version observations from `.php-version`, `.tool-versions`, `composer.json`, and `composer.lock`;
- exact, range, floating, and unknown version states;
- bundled, data-driven PHP2Shell advisory metadata;
- custom advisory files through `--advisories`;
- deterministic table and JSON output;
- a clean internal separation between domain, application, infrastructure, and reporting code.

Docker, CI providers, framework fingerprinting, PHP configuration, extension analysis, SARIF, baselines, and additional advisory sources are planned.

## Installation

### From source

```sh
git clone https://github.com/hebertcisco/php2shell_checker.git
cd php2shell_checker
cargo install --path .
```

The repository URL above is a placeholder until the project has an official remote.

### Run without installing

```sh
cargo run -- scan ./repository
cargo run -- scan ./repository --format json
cargo run -- scan ./repository --advisories ./advisories/custom.toml
```

## Exit codes

- `0`: no findings;
- `1`: findings were detected;
- `2`: usage or configuration error;
- `3`: internal error.

## Safety model

Scans are offline by default. The analyzer reads files, evaluates configuration, and emits findings. It does not run project code, invoke package managers, contact external services, or confirm the runtime artifact that was actually deployed.

The bundled advisory is stored in `advisories/php2shell.toml` and compiled into the binary with `include_str!`.

## Development

```sh
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines and [SECURITY.md](SECURITY.md) for vulnerability reporting.

## License

This project is released under the MIT License. See [LICENSE](LICENSE).
