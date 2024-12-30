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
use std::{collections::HashMap, env, fmt::{self, Display}, io, error::Error, path::{Path, PathBuf}, result::Result};

use clap::Parser;
use owo_colors::OwoColorize as _;
use walkdir::WalkDir;

const IGNORE: [&str; 2] = [".git", ".venv"];
fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();
        let min_d = args.min_depth.unwrap_or(0) as usize;
        let max_d = args.min_depth.map_or(usize::MAX, |d| d as usize);

        let shell_path = env::var("PATH").expect(r#""PATH" not found."#);
        let mut path_vals: Vec<_> = shell_path.split(':').collect();
        path_vals.sort_unstable_by_key(|k| k.len());

        let mut found_paths = Vec::new();
        let mut forbidden_map = HashMap::new();
        for uc_entry in path_vals.into_iter().flat_map(|p| gen_walkdir(p, min_d, max_d).into_iter()) {
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
                                forbidden_map.entry(io_err.kind()).or_insert_with(Vec::new).push(path.to_path_buf());
                        },
                }
        }
        // found_paths.sort_unstable_by_key(|(f, p)| f);
        // found_paths.sort_unstable_by_key(|pair| pair.0.clone());
        found_paths.sort_unstable();
        let found_paths = FoundPaths {found_paths};
        println!("forbidden_map: {:#?}", forbidden_map);
        println!("found_paths: {}", found_paths);

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
                        write!(f, "{}\n", path)?;
                }
                Ok(())
        }
}
impl Display for FoundPath {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:-<8}: {:->20}", self.file.green(), self.path.display())
        }
}



/// Generate a `WallkDir` directory representation which can be iterated on.
fn gen_walkdir(start_dir: &str, min_depth: usize, max_depth: usize) -> WalkDir {
        WalkDir::new(start_dir)
                .min_depth(min_depth)
                .max_depth(max_depth)
                // .into_iter()
                // .filter_entry(|ent|
                //         if args.show_all { true }
                //         else if ent.depth() != 0 { !is_const_ignore(ent) } else { true }
                // )
}

/// Sort-Path
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// String Arg
        dir: Option<String>,
        /// Show all files.
        #[arg(short, long)]
        show_all: bool,
        /// Max_D
        #[arg(short, long)]
        max_depth: Option<u8>,
        /// (start dir is '0')
        #[arg(long)]
        min_depth: Option<u8>,
}

/// Starts with a literal '.' and has len > 1
/// (so the local shorthand `./` is not ignored)
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
        entry.file_name().to_str().map(|s| s.starts_with('.') && (s.len() > 1)).unwrap_or(false)
}

/// Member of in-script ignore list.
fn is_const_ignore(entry: &walkdir::DirEntry) -> bool {
        entry.file_name().to_str().map(|s| IGNORE.contains(&s)).unwrap_or(false)
}
