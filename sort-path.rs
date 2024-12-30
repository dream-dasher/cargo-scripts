#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
clap = { version = "4.5", features = ["derive"] }
owo-colors = "4.1.0"
walkdir = "2.5.0"
---
//! Various things, including
//! looking at everything in path:
//! ```zsh
//! clear;
//! echo $PATH | sd : '\n' | xargs -I_ fd '.*' _ -t f | sort
//! ```
//!
//! issues with BROWSER, which gives a value to `Command::new(_)`, which doesn't jive nicely
//! with mac systems (.app, won't work.)  .../Content/Macos/firefox  sort of works.
//! Also, samply just gobbles up the error.
//! ```zsh
//! clear;
//! echo "--safari-- ";           BROWSER=safari                      ./opener-example-minimal.rs;
//! echo "--firefox--";           BROWSER=firefox                     ./opener-example-minimal.rs;
//! echo "--/App..Firefox.app--"; BROWSER='/Applications/Firefox.app' ./opener-example-minimal.rs;
//! echo "--null--";              BROWSER=''                          ./opener-example-minimal.rs;
//! echo "--unset--";                                                 ./opener-example-minimal.rs;
//!
//! clear;
//! echo "--safari-- ";           BROWSER=safari                      ./opener-example.rs github.com;
//! echo "--firefox--";           BROWSER=firefox                     ./opener-example.rs github.com;
//! echo "--/App..Firefox.app--"; BROWSER='/Applications/Firefox.app' ./opener-example.rs github.com;
//! echo "--null--";              BROWSER=''                          ./opener-example.rs github.com;
//! echo "--unset--";                                                 ./opener-example.rs github.com;
//!
//! clear;
//! RUST_BACKTRACE=1 BROWSER='/Applications/Firefox.app' ./opener-test.rs https://www.github.com
//!
//! ~/coding_dirs/rust/Scripts_rust on  main [?] via 🦀 v1.83.0
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
use walkdir::WalkDir;

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

        let mut found_paths = Vec::new();
        let mut forbidden_map = HashMap::new();
        for uc_entry in path_vals.into_iter().flat_map(|p| WalkDir::new(p).into_iter()) {
                match uc_entry {
                        Ok(entry) => {
                                let file = entry.file_name().to_string_lossy().into_owned();
                                let path = entry.path().to_path_buf();
                                found_paths.push(FoundPath{ file, path });
                        },
                        Err(err) => {
                                let depth = err.depth();
                                let path = err.path().unwrap_or(Path::new(""));
                                let io_err = err.io_error().expect("walkdir error not wrapped io-error");
                                forbidden_map.entry(io_err.kind()).or_insert_with(Vec::new).push((depth, path.to_path_buf()));
                        },
                }
        }
        found_paths.sort_unstable();
        let found_paths = FoundPaths {found_paths};
        if !args.found_paths_only { println!("{}:", "Found paths".blue()); }
        println!("{}", found_paths); // Just doing formatting here would probably have been slightly better organizationally. (vs newtype)
        if args.show_errors {
                println!("--------------- errors ---------------");
                for key in forbidden_map.keys() {
                        println!("{:?}", key.red());
                        for (depth, path) in forbidden_map.get(key).unwrap() {
                                println!("      at depth {:<-2}: {:->20}", depth.blue(), path.display().purple());
                        }
                }
        } else if !forbidden_map.is_empty()  && !args.found_paths_only {
               println!("Some paths could not be fully processed.");
               println!("{} errors were recorded during directory walk.", forbidden_map.len().red());
               println!("Use the `{}` flag for greater visibility.", "--show-errors".cyan());
        }

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
