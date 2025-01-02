#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap = { version = "4.5", features = ["derive"] }
owo-colors = "4.1.0"
walkdir = "2.5.0"
---
//! Various things, including
//! looking at everything in path:
//!
//! ```zsh
//! clear;
//! echo $PATH | sd : '\n' | xargs -I_ fd '.*' _ -t f | sort
//! ```
//!
//! ## Run:
//! ```zsh
//! clear; ./sort-path.rs
//! ```
//!
//! ## Convenience note:
//! `chmod u+x sort-path.rs`
use std::{collections::HashMap, env, fmt::{self, Display}, error::Error, path::{Path, PathBuf}, result::Result};

use clap::Parser;
use owo_colors::OwoColorize as _;

fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();

        let shell_path = env::var("PATH").expect(r#""PATH" not found."#);
        let mut path_vals: Vec<_> = shell_path.split(':').collect();
        path_vals.sort_unstable_by_key(|k| k.len());
        if args.raw_paths {
                println!("Raw {} paths:", "$PATH".green());
                for (i, p) in path_vals.into_iter().enumerate() {
                        let sep =  if i % 4 == 0 {"> "} else {"| "};
                        println!("{:>3}{} {:<5}", i.blue(), sep.black(), p.cyan());
                }
                return Ok(())
        }

        // std::fs
        use std::fs;
        // let mut found_paths = Vec::new();
        // let mut forbidden_map = HashMap::new();
        for uc_entry in path_vals.clone().into_iter().flat_map(|p| fs::read_dir(p)) {
                println!("std::fs {:?}", &uc_entry.blue());
                // match uc_entry {
                //         Ok(entry) => {
                //                 // println!("std::fs {:?}", &entry.green());
                //                 // let file = entry.file_name().to_string_lossy().into_owned();
                //                 // let path = entry.path().to_path_buf();
                //                 // found_paths.push(FoundPath{ file, path });
                //         },
                //         Err(err) => {
                //                 println!("std::fs {:?}", &err.red());
                //                 // let depth = err.depth();
                //                 // let path = err.path().unwrap_or(Path::new(""));
                //                 // let io_err = err.io_error().expect("walkdir error not wrapped io-error");
                //                 // forbidden_map.entry(io_err.kind()).or_insert_with(Vec::new).push((depth, path.to_path_buf()));
                //         },
                // }
        }

        // WalkDir
        use walkdir::WalkDir;
        // let mut found_paths = Vec::new();
        // let mut forbidden_map = HashMap::new();
        for uc_entry in path_vals.into_iter().flat_map(|p| WalkDir::new(p).into_iter()) {
                // dbg!(&uc_entry);
        //         match uc_entry {
        //                 Ok(entry) => {
        //                         let file = entry.file_name().to_string_lossy().into_owned();
        //                         let path = entry.path().to_path_buf();
        //                         found_paths.push(FoundPath{ file, path });
        //                 },
        //                 Err(err) => {
        //                         let depth = err.depth();
        //                         let path = err.path().unwrap_or(Path::new(""));
        //                         let io_err = err.io_error().expect("walkdir error not wrapped io-error");
        //                         forbidden_map.entry(io_err.kind()).or_insert_with(Vec::new).push((depth, path.to_path_buf()));
        //                 },
        //         }
        }
        // found_paths.sort_unstable();
        // let found_paths = FoundPaths {found_paths};
        // if !args.found_paths_only { println!("{}:", "Found paths".blue()); }
        // println!("{}", found_paths); // Just doing formatting here would probably have been slightly better organizationally. (vs newtype)
        // if args.show_errors {
        //         println!("--------------- errors ---------------");
        //         for key in forbidden_map.keys() {
        //                 println!("{:?}", key.red());
        //                 for (depth, path) in forbidden_map.get(key).unwrap() {
        //                         println!("      at depth {:<-2}: {:->20}", depth.blue(), path.display().purple());
        //                 }
        //         }
        // } else if !forbidden_map.is_empty()  && !args.found_paths_only {
        //        println!("Some paths could not be fully processed.");
        //        println!("{} errors were recorded during directory walk.", forbidden_map.len().red());
        //        println!("Use the `{}` flag for greater visibility.", "--show-errors".cyan());
        // }

        Ok(())
}

/// NewType to enable Display and Comparison
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FoundPath {
        file: String,
        path: PathBuf,
}
/// NewType to enable Display
#[derive(Debug, Clone)]
struct FoundPaths {
        found_paths: Vec<FoundPath>,
}
impl Display for FoundPaths {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for path in self.found_paths.iter() {
                        writeln!(f, "{}", path)?;
                }
                Ok(())
        }
}
impl Display for FoundPath {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:-<8}: {:->20}", self.file.green(), self.path.display())
        }
}



/// Sort-Path - Displays files findable via $PATH
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// Show the explicit $PATH paths.
        #[arg(short, long)]
        raw_paths: bool,
        /// Show only the errors that occur.
        #[arg(short, long)]
        show_errors: bool,
        /// Only show found-paths. (useful for piping, e.g. into `wc -l`)
        #[arg(short, long)]
        found_paths_only: bool,
}
