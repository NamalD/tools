use std::{fs, io};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let dir = match fs::read_dir(args.path) {
        Ok(dir) => dir,
        Err(e) => {
            println!("Cannot access directory: {}", e.to_string());
            return;
        }
    };

    let paths = dir
        .map(|entry_result| entry_result.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    match paths {
        // TODO: Just display path without dir in args
        // TODO: Skip path if hidden and not in `--all`
        Ok(paths) => {
            println!("Length {}", paths.len());
            for path in paths {
                print_path(path);
            }
        }
        Err(e) => {
            println!("{}", e.to_string())
        }
    }
}

fn print_path(path: PathBuf) {
    // TODO: Handle errors properly
    let file_name = path.as_path().file_name().expect("Not a valid file name")
        .to_str().expect("Cannot convert into string")
        .to_owned();

    let dir_suffix = if path.is_dir() { "/" } else { "" };

    // TODO: Colorize output
    println!("{}{}", file_name, dir_suffix);
}
