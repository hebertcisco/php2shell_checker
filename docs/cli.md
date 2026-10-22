# CLI reference

The executable is named `php2shell_checker`.

## `scan`

```sh
php2shell_checker scan PATH
```

| Option | Description |
| --- | --- |
| `--format table` | Human-readable output; the default. |
| `--format json` | Machine-readable `AnalysisReport` JSON. |
| `--advisories PATH` | Load advisory metadata from a TOML file instead of the embedded advisory. |

Examples:

```sh
php2shell_checker scan .
php2shell_checker scan ./service --format json
php2shell_checker scan ./service --advisories ./advisories/internal.toml
```

The scanner exits with `1` when findings are present and `0` when the report is clean.

## Other commands

```sh
php2shell_checker explain PHP-ADV-001
php2shell_checker advisories list
php2shell_checker advisories show PHP2Shell
php2shell_checker detectors list
php2shell_checker version
```

Detailed remediation and references are currently emitted with each finding.

## Safety guarantees

Normal commands do not execute PHP, invoke project tooling, or make network calls. A scan uses repository files and explicitly supplied advisory data only.
