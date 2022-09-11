use sub_fixer::Subtitle;

use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The subtitle file to fix
    file_name: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let input = read_to_string(&args.file_name)?;

    let subtitle = Subtitle::new(input);
    let output = subtitle.fix();
    let fixed_subtitle_file_name = args.file_name.with_extension("fixed.srt");

    std::fs::write(fixed_subtitle_file_name, output)
}
