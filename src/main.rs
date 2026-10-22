#![forbid(unsafe_code)]

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use php2shell_checker::{application, infrastructure, reporting};
use std::path::PathBuf;

const EMBEDDED_PHP2SHELL: &str = include_str!("../advisories/php2shell.toml");

#[derive(Parser)]
#[command(
    name = "php2shell_checker",
    about = "Static PHP runtime exposure and misconfiguration scanner"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Scan {
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = Format::Table)]
        format: Format,
        #[arg(long)]
        advisories: Option<PathBuf>,
    },
    Explain {
        finding_code: String,
    },
    Advisories {
        #[command(subcommand)]
        command: AdvisoryCommand,
    },
    Detectors {
        #[command(subcommand)]
        command: DetectorCommand,
    },
    Version,
}

#[derive(Subcommand)]
enum AdvisoryCommand {
    List,
    Show { id: String },
}

#[derive(Subcommand)]
enum DetectorCommand {
    List,
}

#[derive(Clone, ValueEnum)]
enum Format {
    Table,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan {
            path,
            format,
            advisories,
        } => {
            let report = match advisories {
                Some(advisory) => application::analyze(&path, &advisory)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?,
                None => {
                    let versions = infrastructure::parsers::collect_versions(&path)?;
                    application::analyze_with_advisory_text(&path, EMBEDDED_PHP2SHELL, versions)
                        .map_err(|error| anyhow::anyhow!(error.to_string()))?
                }
            };

            let output = match format {
                Format::Table => reporting::table(&report),
                Format::Json => reporting::json(&report)?,
            };
            print!("{output}");

            if !report.findings.is_empty() {
                std::process::exit(1);
            }
        }
        Command::Explain { finding_code } => {
            println!("{finding_code}: use the finding remediation and references emitted by scan.");
        }
        Command::Advisories { command } => match command {
            AdvisoryCommand::List => println!("PHP2Shell (CVE-2026-109618, CVE-2026-109619)"),
            AdvisoryCommand::Show { id } => {
                println!("{id}: bundled advisory metadata is in advisories/php2shell.toml")
            }
        },
        Command::Detectors { command } => match command {
            DetectorCommand::List => println!("version-resolution\nphp2shell"),
        },
        Command::Version => println!("php2shell {}", env!("CARGO_PKG_VERSION")),
    }

    Ok(())
}
