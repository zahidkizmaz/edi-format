mod formatter;
mod io_helpers;
mod segments;
use clap::Parser;
use formatter::EDIFormatter;
use io_helpers::write_content_to_file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path to format
    #[arg(index = 1, default_value_t = format!("."))]
    file_name: String,

    /// Do not actually modify the file but show formatted content in stdout
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();

    let formatter = EDIFormatter::new(args.file_name.as_str());
    let formatted_content = formatter.format();
    if args.dry_run {
        println!("{formatted_content}");
        return;
    }
    write_content_to_file(args.file_name.as_str(), formatted_content);
}
