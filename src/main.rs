mod formatter;
mod io_helpers;
mod segments;
use clap::Parser;
use formatter::EDIFormatter;
use io_helpers::write_content_to_file;
use tracing::{debug, error, info};

enum FormatResult {
    Formatted,
    Skipped,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to format.
    /// Can be a single file or a directory
    #[arg(index = 1, default_value_t = format!("."))]
    path: String,

    /// Do not actually modify the file but show formatted content in stdout
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    let file_path = args.path;
    match format_file(file_path.as_str()) {
        Ok(FormatResult::Formatted) => info!("formatted {file_path}"),
        Ok(FormatResult::Skipped) => debug!("skipping formatting of{file_path}"),
        Err(()) => error!("error while formatting {file_path}"),
    }
}

fn format_file(file_path: &str) -> Result<FormatResult, ()> {
    let formatter = EDIFormatter::new(file_path);
    let formatted_content = formatter.format();

    if formatter.file_content == formatted_content {
        return Ok(FormatResult::Skipped);
    }

    match write_content_to_file(file_path, formatted_content) {
        Ok(()) => Ok(FormatResult::Formatted),
        Err(()) => Err(()),
    }
}
