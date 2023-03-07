use clap::Parser;
use pingpong_word;

/// Simple program to identify the longest word that
/// in a file that can be alternatively typed with your
/// right and left hand.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let max_pingpong = pingpong_word::check_file(args.path);
    // print max_pingpong
    println!("{:?}", max_pingpong);
}
