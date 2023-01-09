mod args;
mod file;
mod owners;
mod permisos;

mod list;

mod options {
    pub mod filter;
    pub mod format;
    pub mod options;
    pub mod sort;
}

use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::fs::read_dir;
use std::path::Path;
use list::list;

use crate::file::File;
use args::Args;
use clap::Parser;
use options::filter::*;
use options::format::*;
use options::options::*;
use options::sort::*;

pub fn get_options(args: &Args) -> (Filter, Sort, Format) {
    (
        if args.dirs {
            Filter::Dirs
        } else {
            if args.all {
                Filter::All
            } else {
                Filter::None
            }
        },
        if args.time {
            Sort::Time
        } else {
            if args.size {
                Sort::Size
            } else {
                Sort::None
            }
        },
        if args.long {
            Format::Long
        } else {
            Format::Short
        },
    )
}

fn print_files(dir: &Path) {
    for entry in read_dir(dir).unwrap() {
        let file_name: String = entry.unwrap().file_name().into_string().unwrap();

        println!("{}", file_name);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let options = get_options(&args);

    let path = Path::new(&args.path);

    if path.is_dir() {
        print!(
            "{}",
            list(
                path,
                match options.0 {
                    Filter::None => default,
                    Filter::All => all,
                    Filter::Dirs => dirs,
                },
                match options.1 {
                    Sort::None => no_order,
                    Sort::Size => size,
                    Sort::Time => time,
                },
                match options.2 {
                    Format::Short => short,
                    Format::Long => long,
                }
            )?
        );
    } else {
        print!("{}", args.path)
    }
    Ok(())
}
