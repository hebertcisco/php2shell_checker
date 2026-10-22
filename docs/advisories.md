# Advisory data

Advisories are data, not detector code. The bundled PHP2Shell advisory is stored at `advisories/php2shell.toml` and embedded with `include_str!`. Users can provide a custom TOML file with `scan --advisories PATH`.

## Schema

```toml
id = "PHP2Shell"
aliases = ["CVE-2026-109618", "CVE-2026-109619"]
severity = "critical"
mitigations = ["Upgrade to a fixed release."]
references = ["https://example.invalid/advisory"]

[[affected]]
branch = "8.2"
range = "<8.2.35"
first_fixed = "8.2.35"

[[affected]]
branch = "8.1-or-older"
range = "*"
status = "unverified_eol"
```

Top-level fields are `id`, `aliases`, `severity`, `mitigations`, `references`, and `affected`. Each branch has `branch` and `range`; `first_fixed` is used for known fixes, while `status = "unverified_eol"` represents an unsupported branch without a known fixed release.

## Version interpretation

- Exact versions are compared with `first_fixed`.
- Composer constraints are reported as ranges because the source file does not prove the resolved runtime.
- `latest`, `*`, and incomplete major-only values are floating.
- Unsupported branches are unverified/EOL, never safe.
- Unknown versions remain distinct from safe versions.

Network retrieval belongs in a future explicit update command and must never happen implicitly during `scan`.
