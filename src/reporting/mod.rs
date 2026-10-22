#![forbid(unsafe_code)]

use crate::domain::AnalysisReport;

pub fn json(report: &AnalysisReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

pub fn table(report: &AnalysisReport) -> String {
    let mut out = format!(
        "php2shell_checker: {}\n\nFindings: {}\n",
        report.root.display(),
        report.findings.len()
    );

    for f in &report.findings {
        let line = f
            .evidence
            .line
            .map_or_else(|| "?".to_string(), |value| value.to_string());
        out.push_str(&format!(
            "[{:?}] {} {}\n  {}:{}\n  {}\n",
            f.severity,
            f.code,
            f.title,
            f.evidence.file.display(),
            line,
            f.why_it_matters
        ));
    }

    out
}
