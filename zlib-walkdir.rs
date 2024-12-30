#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
clap = { version = "4", features = ["derive"] }
walkdir = "2.5.0"
---
#![feature(let_chains)]
//! # Cargo-Script: scratchlib-walkdir
//! Familiarization with **walkdir** crate. 
//! [walkdir documentation](https://docs.rs/walkdir/latest/walkdir/index.html)
//!
//! ## TLDR
//! 1. Generate `WalkDir` from str
//!  - set options at creation
//! 2. Choose a filter that blocks descent
//! 3. Iterate through entries. (`DirEntries` are entries *IN* dirs, not necesarilly dirs)
//! 
//! ## Caveat
//! - WalkDir's `into_iter()` does not expose standard iterator methods
//! - WalkDir's iter-combos change type in a way that will shape branching options
//!
use std::{error::Error, result::Result};

use clap::Parser;
use walkdir::WalkDir;

const IGNORE: [&str; 2] = [".git", ".venv"];

fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();


        let start_dir = args.dir.unwrap_or(".".to_string());
        let walkdir = WalkDir::new(start_dir)
                .min_depth(args.min_depth.unwrap_or(0) as usize) 
                .max_depth(args.min_depth.map_or(usize::MAX, |d| d as usize))
                .into_iter()
                .filter_entry(|ent|
                        if args.show_all { true }
                        else if ent.depth() != 0 { !is_const_ignore(ent) } else { true }
                ); 
        // AWKwARD: `filter_entry` changes walkdir type
        //           we cannot conditonally run the method as walkdir's type would change
        // For performance: branches would need to terminate past the walkdir type
        // For convenience: adding conditional logic in filter entry

        for uc_entry in walkdir {
                println!("{:?}", uc_entry?.path().display());
        }
        Ok(())
}

/// scratchlib-walkdir Cargo-Script
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
