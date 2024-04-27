mod formatter;
mod io_helpers;
mod segments;

use clap::Parser;
use formatter::EDIFormatter;
use io_helpers::{write_content_to_file, write_content_to_stdout};
use tracing::{debug, error, info, level_filters::LevelFilter, Level};
use tracing_subscriber::{fmt, prelude::*, Registry};
enum FormatResult {
    Format(String),
    Skip,
}

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
    debug!("Passed arguments: {:?}", args);
    init_logging(args.log_level);

    let file_path = args.path.as_str();
    match format_file(file_path) {
        Ok(FormatResult::Format(formatted_content)) => {
            if args.dry_run {
                info!("Running in dry-run mode");
                let _ = write_content_to_stdout(formatted_content);
            } else {
                let _ = write_content_to_file(file_path, formatted_content);
                info!("formatted {file_path}")
            }
        }
        Ok(FormatResult::Skip) => debug!("skipping formatting of {file_path}"),
        Err(()) => error!("error while formatting {file_path}"),
    }
}

fn format_file(file_path: &str) -> Result<FormatResult, ()> {
    let formatter = EDIFormatter::new(file_path);
    let formatted_content = formatter.format();

    if formatter.file_content == formatted_content {
        return Ok(FormatResult::Skip);
    }
    Ok(FormatResult::Format(formatted_content))
}
