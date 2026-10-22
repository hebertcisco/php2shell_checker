# Homebrew distribution

`php2shell_checker` can be distributed as a Homebrew formula named `php2shell-checker`.

The formula installs the executable named `php2shell_checker`.

## End-user installation

After the tap is published:

```sh
brew tap hebertcisco/php2shell-checker
brew install php2shell-checker
```

Verify the installation:

```sh
php2shell_checker --version
php2shell_checker --help
php2shell_checker detectors list
```

Upgrade it with:

```sh
brew update
brew upgrade php2shell-checker
```

## Formula repository layout

The companion tap repository should use this layout:

```text
homebrew-php2shell-checker/
├── Formula/
│   └── php2shell-checker.rb
├── docs/
│   └── maintaining.md
├── README.md
└── LICENSE
```

The formula builds the tagged upstream source with Cargo:

```ruby
depends_on "rust" => :build

def install
  system "cargo", "install", *std_cargo_args(path: ".")
end
```

## Local formula testing

From the formula repository:

```sh
brew style ./Formula/php2shell-checker.rb
brew audit --formula ./Formula/php2shell-checker.rb
brew install --build-from-source ./Formula/php2shell-checker.rb
brew test php2shell-checker
```

The formula test should verify that the installed binary starts and exposes the expected CLI surface:

```ruby
test do
  assert_match "Static PHP runtime exposure", shell_output("#{bin}/php2shell_checker --help")
  assert_match "version", shell_output("#{bin}/php2shell_checker detectors list")
end
```

## Releasing a new formula version

1. Update `version` in `Cargo.toml`.
2. Run the project's formatting, test, and Clippy checks.
3. Create an upstream `vX.Y.Z` release tag.
4. Confirm that the source archive is publicly available.
5. Update the formula URL and version.
6. Use Homebrew to calculate the checksum from the exact archive.
7. Run `brew style`, `brew audit`, installation, and formula tests.

Example checksum workflow:

```sh
VERSION=0.2.0
brew fetch --build-from-source \
  "https://github.com/hebertcisco/php2shell_checker/archive/refs/tags/v${VERSION}.tar.gz"
```

Never invent or reuse a checksum from another archive. The checksum must match the exact immutable release URL used by the formula.

## Packaging policy

- The formula must build from a versioned upstream tag.
- The formula must not embed credentials or runtime secrets.
- The formula must not alter scanner safety defaults.
- Normal scans remain offline and non-executing.
- The formula license must match the upstream MIT license.
- A release should document user-visible changes and compatibility notes.

The tap repository should contain its own maintenance guide and CI job running `brew style`, `brew audit`, source installation, and `brew test` on macOS.
