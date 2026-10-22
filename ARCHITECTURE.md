# Architecture

`php2shell_checker` is a single Rust package organized into four internal layers.

## Domain

`src/domain` contains the stable vocabulary of the analyzer: observations, findings, advisories, severities, confidence levels, and public contracts. It does not access the filesystem or perform reporting.

## Application

`src/application` coordinates analysis. It consumes observations, evaluates advisory rules, creates findings, and keeps detector behavior deterministic.

## Infrastructure

`src/infrastructure` contains adapters for repository files and parsers. Filesystem access and syntax-specific parsing belong here so the application layer can remain easy to test.

## Reporting

`src/reporting` converts the domain report into user-facing formats. Reporters must preserve stable finding codes, evidence, ordering, and remediation details.

## Safety boundaries

The scanner is designed for offline, non-executing analysis. New code must not execute scanned project code, invoke package managers, follow paths outside the scan root, or make network requests during a normal scan.

## Extension workflow

New detectors should add domain types only when necessary, parse source data through infrastructure adapters, produce stable findings in the application layer, and include fixtures covering both safe and unsafe cases.
