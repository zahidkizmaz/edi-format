mod formatter;

mod segments;

use anyhow::{Context, Result};
use std::{fs::File, io::BufWriter, str::FromStr};

use clap::{crate_description, crate_version, value_parser, Arg, ArgAction, Command};
use tracing::{debug, info, level_filters::LevelFilter, Level};
use tracing_subscriber::{fmt, prelude::*, Registry};

fn cli() -> Command {
    let log_level = Arg::new("log_level")
        .short('l')
        .long("log-level")
        .help("Log level eg: trace, debug, info, warn, error.")
        .value_parser(Level::from_str)
        .default_value(Level::INFO.as_str());

    let dry_run = Arg::new("dry_run")
        .long("dry-run")
        .help("Do not modify the file but show formatted content in stdout.")
        .value_parser(value_parser!(bool))
        .action(ArgAction::SetTrue);

    let stdin = Arg::new("stdin")
        .long("stdin")
        .help("Use stdin as the input.")
        .value_parser(value_parser!(bool))
        .action(ArgAction::SetTrue);

    let path = Arg::new("path")
        .required_unless_present("stdin")
        .help("Path to format.")
        .index(1);

    Command::new("edi-format")
        .version(crate_version!())
        .about(crate_description!())
        .arg(path)
        .arg(log_level)
        .arg(dry_run)
        .arg(stdin)
}

fn init_logging(log_level: Level) {
    let level_filter = LevelFilter::from_level(log_level);
    let subscriber = Registry::default()
        .with(level_filter)
        .with(fmt::Layer::default());
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn main() -> Result<()> {
    let args = cli().get_matches();
    let log_level = args.get_one::<Level>("log_level").unwrap();
    let dry_run = args.get_flag("dry_run");
    let stdin = args.get_flag("stdin");

    init_logging(*log_level);
    debug!(?args, "Passed arguments");

    if stdin {
        format_stdin()?;
    } else {
        let file_path = args.get_one::<String>("path").unwrap();
        format_file(file_path, dry_run)?;
    }
    Ok(())
}

fn format_file(file_path: &str, dry_run: bool) -> Result<()> {
    let input = File::open(file_path).context("error opening file")?;

    if dry_run {
        info!("Running in dry-run mode");
        let output = BufWriter::new(std::io::stdout().lock());
        formatter::format(&input, output).context("error formatting")?;
        drop(input);
    } else {
        let mut output = BufWriter::new(tempfile::NamedTempFile::new_in("./")?);
        formatter::format(&input, &mut output).context("error formatting")?;
        drop(input);
        output
            .into_inner()
            .expect("This should never happen")
            .persist(file_path)?;
    };

    Ok(())
}

fn format_stdin() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();

    formatter::format(stdin, stdout)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::read, io::Write, process::Command};

    use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
    use predicates::prelude::predicate;
    use pretty_assertions_sorted::assert_eq;

    const EF: &str = "edi-format";

    #[test]
    fn run_help() {
        let mut cmd = Command::cargo_bin(EF).unwrap();
        cmd.arg("--help");

        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("--dry-run"));
        cmd.assert().stdout(predicate::str::contains("--log-level"));
        cmd.assert().stdout(predicate::str::contains("--stdin"));
        cmd.assert().stdout(predicate::str::contains("--version"));
        cmd.assert().stdout(predicate::str::contains("version"));
    }

    // #[test]
    // fn run_formatted_file() {
    //     let mut cmd = Command::cargo_bin(EF).unwrap();
    //     cmd.arg("--log-level");
    //     cmd.arg("debug");
    //     cmd.arg("tests/valid_formatted.edi");
    //     cmd.assert().success();
    //     cmd.assert().stdout(predicate::str::contains(
    //         "Already formatted skipping tests/valid_formatted.edi",
    //     ));
    // }

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

    #[test]
    fn format_unformatted_file() {
        let mut cmd = Command::cargo_bin(EF).unwrap();
        let mut temp = tempfile::NamedTempFile::new().unwrap();
        let temp_path = temp.path().to_str().unwrap().to_string();
        temp.write_all(include_bytes!("../tests/valid_not_formatted.edi"))
            .unwrap();

        cmd.arg(&temp_path);
        cmd.assert().success();
        let result = read(&temp_path).unwrap();

        drop(temp);
        assert_eq!(
            result,
            include_bytes!("../tests/valid_formatted.edi").to_vec(),
        );
    }
}
