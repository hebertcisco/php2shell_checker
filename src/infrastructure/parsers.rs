#![forbid(unsafe_code)]

use crate::domain::{Confidence, SourceLocation, VersionObservation, VersionValue};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("cannot read {path}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("invalid JSON in {path}: {source}")]
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
}

pub fn collect_versions(root: &Path) -> Result<Vec<VersionObservation>, ParseError> {
    let mut out = Vec::new();
    let candidates = [
        ".php-version",
        ".tool-versions",
        "composer.json",
        "composer.lock",
    ];
    for name in candidates {
        let path = root.join(name);
        if path.is_file() {
            parse_file(&path, &mut out)?;
        }
    }
    Ok(out)
}

fn parse_file(path: &Path, out: &mut Vec<VersionObservation>) -> Result<(), ParseError> {
    let text = fs::read_to_string(path).map_err(|source| ParseError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    let file = path
        .file_name()
        .and_then(|n| n.to_str())
        .map_or("", |name| name);
    match file {
        ".php-version" => add(out, path, text.trim(), Confidence::Exact, "dev"),
        ".tool-versions" => {
            if let Some(v) = text.lines().find_map(|l| {
                l.strip_prefix("php ")
                    .and_then(|x| x.split_whitespace().next())
            }) {
                add(out, path, v, Confidence::Exact, "dev");
            }
        }
        "composer.json" => parse_composer_json(path, &text, out)?,
        "composer.lock" => parse_composer_lock(path, &text, out)?,
        _ => {}
    }
    Ok(())
}

fn parse_composer_json(
    path: &Path,
    text: &str,
    out: &mut Vec<VersionObservation>,
) -> Result<(), ParseError> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|source| ParseError::Json {
            path: path.to_path_buf(),
            source,
        })?;
    if let Some(v) = value.pointer("/require/php").and_then(|v| v.as_str()) {
        add(out, path, v, Confidence::Range, "build");
    }
    if let Some(v) = value
        .pointer("/config/platform/php")
        .and_then(|v| v.as_str())
    {
        add(out, path, v, Confidence::Exact, "build");
    }
    Ok(())
}

fn parse_composer_lock(
    path: &Path,
    text: &str,
    out: &mut Vec<VersionObservation>,
) -> Result<(), ParseError> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|source| ParseError::Json {
            path: path.to_path_buf(),
            source,
        })?;
    if let Some(v) = value.pointer("/platform/php").and_then(|v| v.as_str()) {
        add(out, path, v, Confidence::Exact, "build");
    }
    Ok(())
}

fn add(
    out: &mut Vec<VersionObservation>,
    path: &Path,
    value: &str,
    confidence: Confidence,
    environment: &str,
) {
    let kind = if value == "latest" || value == "*" || value.split('.').count() == 1 {
        VersionValue::Floating(value.to_string())
    } else if confidence == Confidence::Exact {
        VersionValue::Exact(value.to_string())
    } else {
        VersionValue::Range(value.to_string())
    };
    out.push(VersionObservation {
        value: kind,
        source: SourceLocation {
            file: path.to_path_buf(),
            line: Some(1),
            column: Some(1),
        },
        confidence,
        environment: environment.to_string(),
    });
}
