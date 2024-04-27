mod formatter;
mod io_helpers;
mod segments;
use clap::Parser;
use formatter::EDIFormatter;
use io_helpers::{write_content_to_file, write_content_to_stdout};
use tracing::{debug, error, info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
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
}

fn init_logging() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("EDI_FORMAT_LOG")
        .try_from_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer())
        .init();
}

fn main() {
    init_logging();
    let args = Args::parse();
    debug!("Passed arguments: {:?}", args);

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
