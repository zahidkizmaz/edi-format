mod formatter;
mod io_helpers;
mod segments;

use clap::Parser;
use formatter::EDIFormatter;
use io_helpers::{write_content_to_file, write_content_to_stdout};
use tracing::{debug, error, info, level_filters::LevelFilter, Level};
use tracing_subscriber::{fmt, prelude::*, Registry};

use crate::formatter::FormatResult;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to format.
    #[arg(index = 1)]
    path: String,

    /// Do not modify the file but show formatted content in stdout
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Log level eg: trace, debug, info, warn, error
    #[arg(short, long, default_value_t = Level::INFO)]
    log_level: Level,
}

fn init_logging(log_level: Level) {
    let level_filter = LevelFilter::from_level(log_level);
    let subscriber = Registry::default()
        .with(level_filter)
        .with(fmt::Layer::default());
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn main() {
    let args = Args::parse();
    init_logging(args.log_level);
    debug!("Passed arguments: {:?}", args);

    let file_path = args.path.as_str();
    let formatter = EDIFormatter::new(file_path);
    match formatter.format() {
        Ok(FormatResult::Format(formatted_content)) => {
            if args.dry_run {
                info!("Running in dry-run mode");
                let _ = write_content_to_stdout(formatted_content);
            } else {
                let _ = write_content_to_file(file_path, formatted_content);
                info!("formatted {file_path}")
            }
        }
        Ok(FormatResult::Skip) => debug!("Already formatted skipping {file_path}"),
        Err(()) => error!("error while formatting {file_path}"),
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
    use predicates::prelude::predicate;

    const EF: &str = "edi-format";

    #[test]
    fn run_help() {
        let mut cmd = Command::cargo_bin(EF).unwrap();
        cmd.arg("--help");

        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("--dry-run"));
        cmd.assert().stdout(predicate::str::contains("--log-level"));
        cmd.assert().stdout(predicate::str::contains("--version"));
        cmd.assert().stdout(predicate::str::contains("version"));
    }

    #[test]
    fn run_formatted_file() {
        let mut cmd = Command::cargo_bin(EF).unwrap();
        cmd.arg("--log-level");
        cmd.arg("debug");
        cmd.arg("tests/valid_formatted.edi");

        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains(
            "Already formatted skipping tests/valid_formatted.edi",
        ));
    }

    #[test]
    fn run_dry_run() {
        let mut cmd = Command::cargo_bin(EF).unwrap();

        cmd.arg("--dry-run").arg("tests/valid_not_formatted.edi");

        let formatted_content = "UNA:+.? '
UNB+IATB:1+6XPPC:ZZ+LHPPC:ZZ+940101:0950+1'
UNH+1+PAORES:93:1:IA'
MSG+1:45'
IFT+3+XYZCOMPANY AVAILABILITY'
ERC+A7V:1:AMD'
IFT+3+NO MORE FLIGHTS'
ODI'
TVL+240493:1000::1220+FRA+JFK+DL+400+C'
PDI++C:3+Y::3+F::1'
APD+74C:0:::6++++++6X'
TVL+240493:1740::2030+JFK+MIA+DL+081+C'
PDI++C:4'
APD+EM2:0:1630::6+++++++DA'
UNT+13+1'
UNZ+1+1'";

        cmd.assert().success();
        cmd.assert()
            .stdout(predicate::str::contains("Running in dry-run mode"));
        cmd.assert()
            .stdout(predicate::str::contains(formatted_content));
    }
}
