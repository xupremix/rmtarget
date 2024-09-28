mod args;

use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use std::process::exit;

use args::Args;
use clap::Parser;

const EXIT_FAILURE: i32 = 1;

fn main() {
    let args = Args::parse();

    let mut cleared = 0;
    let mut weight = 0;

    if !args.dir.is_dir() {
        eprintln!(
            "Path \"{}\" is not a directory. (exiting)",
            args.dir.display()
        );
        exit(EXIT_FAILURE);
    }

    rm_targets(
        &args.dir,
        &args.targets,
        args.quiet,
        &mut cleared,
        &mut weight,
    );

    if !args.quiet {
        println!("Cleared {} target directories", cleared);
        let (freed, suff) = match weight {
            0..1024 => (weight as f64, "bytes"),
            1024..1048576 => (weight as f64 / 1024., "kb"),
            1048576..1073741824 => (weight as f64 / 1048576., "Mb"),
            _ => (weight as f64 / 1073741824., "Gb"),
        };
        println!("Freed {:.2}{}", freed, suff);
    }
}

pub(crate) fn rm_targets(
    dir: &Path,
    targets: &[PathBuf],
    quiet: bool,
    cleared: &mut u32,
    weight: &mut u64,
) {
    let entries = match dir.read_dir() {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}. (exiting)", e);
            exit(EXIT_FAILURE);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading entry: {}. (exiting)", e);
                exit(EXIT_FAILURE);
            }
        };
        let filetype = match entry.file_type() {
            Ok(filetype) => filetype,
            Err(e) => {
                eprintln!("Could not get file type: {}", e);
                exit(EXIT_FAILURE);
            }
        };
        if filetype.is_dir() {
            if targets.iter().any(|t| {
                t.file_name().expect("Could not get file name (exiting): ") == entry.file_name()
            }) {
                let w = dir_weight(&entry.path());
                if let Err(e) = remove_dir_all(entry.path()) {
                    eprintln!(
                        "Error removing {}: {}. (exiting)",
                        entry.path().display(),
                        e
                    );
                    exit(EXIT_FAILURE);
                } else {
                    if !quiet {
                        println!("Removed {}", entry.path().display());
                    }
                    *weight += w;
                    *cleared += 1;
                };
            } else {
                rm_targets(&entry.path(), targets, quiet, cleared, weight);
            }
        }
    }
}

pub(crate) fn dir_weight(path: &Path) -> u64 {
    let mut w = 0;
    if path.is_dir() {
        let entries = match path.read_dir() {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Error reading {}: {}. (exiting)", path.display(), e);
                exit(EXIT_FAILURE);
            }
        };
        for entry in entries {
            match entry {
                Ok(entry) => {
                    w += dir_weight(&entry.path());
                }
                Err(e) => {
                    eprintln!("Error reading directory: {}. (exiting)", e);
                    exit(EXIT_FAILURE);
                }
            }
        }
    } else {
        w += match path.metadata() {
            Ok(metadata) => metadata.len(),
            Err(e) => {
                eprintln!(
                    "Could not get metadata for {}: {}. (exiting)",
                    path.display(),
                    e
                );
                exit(EXIT_FAILURE);
            }
        };
    }
    w
}
