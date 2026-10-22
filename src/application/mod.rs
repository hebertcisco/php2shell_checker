use crate::domain::{
    Advisory, AnalysisReport, Evidence, Finding, Severity, VersionObservation, VersionValue,
};
use crate::infrastructure::parsers::collect_versions;
use std::{fs, path::Path};

pub fn analyze(
    root: &Path,
    advisory_path: &Path,
) -> Result<AnalysisReport, Box<dyn std::error::Error + Send + Sync>> {
    let versions = collect_versions(root)?;
    let text = fs::read_to_string(advisory_path)?;
    analyze_with_advisory_text(root, &text, versions)
}

pub fn analyze_with_advisory_text(
    root: &Path,
    text: &str,
    versions: Vec<VersionObservation>,
) -> Result<AnalysisReport, Box<dyn std::error::Error + Send + Sync>> {
    let advisory: Advisory = toml::from_str(text)?;
    let mut report = AnalysisReport {
        root: root.to_path_buf(),
        summary: crate::domain::ProjectSummary {
            versions: versions.clone(),
            topology: Vec::new(),
            drift: Vec::new(),
        },
        findings: Vec::new(),
    };

    for observation in versions {
        evaluate(&mut report, &advisory, &observation);
    }

    report.findings.sort_by(|a, b| {
        a.code
            .cmp(&b.code)
            .then(a.evidence.file.cmp(&b.evidence.file))
    });
    Ok(report)
}

fn evaluate(report: &mut AnalysisReport, advisory: &Advisory, observation: &VersionObservation) {
    let raw = match &observation.value {
        VersionValue::Exact(value) | VersionValue::Range(value) | VersionValue::Floating(value) => {
            value
        }
        VersionValue::Unknown => {
            add_unknown_finding(report, observation);
            return;
        }
    };
    let major_minor = raw
        .trim_start_matches(['^', '~', 'v'])
        .split(['.', '<', '>', '=', ' '])
        .take(2)
        .collect::<Vec<_>>()
        .join(".");

    if raw == "latest" || raw == "*" || raw.split('.').count() < 2 {
        report.findings.push(finding(
            "PHP-VER-002",
            Severity::High,
            "Floating PHP version",
            observation,
            "Floating tags can silently select vulnerable runtimes.",
            "Pin an exact PHP version and preferably an image digest.",
        ));
        return;
    }

    if let Some(branch) = advisory
        .affected
        .iter()
        .find(|branch| branch.branch == major_minor)
    {
        let unverified = branch.status.as_deref() == Some("unverified_eol")
            || version_tuple(&major_minor) <= (8, 1, 99);
        if unverified {
            report.findings.push(finding(
                "PHP-ADV-002",
                Severity::High,
                "Unverified/EOL PHP branch",
                observation,
                "No known fixed release is available for this branch.",
                "Upgrade to a supported PHP branch.",
            ));
        } else if matches!(&observation.value, VersionValue::Range(_)) {
            report.findings.push(finding(
                "PHP-ADV-003",
                Severity::High,
                "PHP version range may include PHP2Shell",
                observation,
                "A Composer constraint does not prove the resolved runtime is fixed.",
                "Resolve the lockfile/runtime to a fixed version and enforce it in CI.",
            ));
        } else if let Some(fixed) = &branch.first_fixed {
            if version_tuple(raw) < version_tuple(fixed) {
                report.findings.push(finding(
                    "PHP-ADV-001",
                    Severity::Critical,
                    "PHP runtime affected by PHP2Shell",
                    observation,
                    "The declared runtime precedes the first fixed release.",
                    &format!("Upgrade PHP {major_minor} to {fixed} or later."),
                ));
            }
        }
    }
}

#[allow(clippy::manual_unwrap_or, clippy::manual_unwrap_or_default)]
fn version_tuple(value: &str) -> (u64, u64, u64) {
    let parts = value
        .trim_start_matches(['^', '~', 'v'])
        .split('.')
        .collect::<Vec<_>>();
    let number = |index: usize| match parts.get(index).and_then(|part| part.parse::<u64>().ok()) {
        Some(value) => value,
        None => 0,
    };
    (number(0), number(1), number(2))
}

fn add_unknown_finding(report: &mut AnalysisReport, observation: &VersionObservation) {
    report.findings.push(finding(
        "PHP-VER-001",
        Severity::Medium,
        "PHP version unknown",
        observation,
        "The runtime cannot be assessed from the available files.",
        "Declare and pin PHP in project and deployment configuration.",
    ));
}

fn finding(
    code: &str,
    severity: Severity,
    title: &str,
    observation: &VersionObservation,
    why: &str,
    remediation: &str,
) -> Finding {
    Finding {
        code: code.to_string(),
        severity,
        title: title.to_string(),
        evidence: Evidence {
            file: observation.source.file.clone(),
            line: observation.source.line,
            snippet: Some(format!("{:?}", observation.value)),
        },
        why_it_matters: why.to_string(),
        remediation: remediation.to_string(),
        references: vec!["PHP2Shell: CVE-2026-109618, CVE-2026-109619".to_string()],
    }
}
