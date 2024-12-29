#!/usr/bin/env cargo +nightly -Zscript
---
[dependencies]
clap = { version = "4.5", features = ["derive"] }
opener = "0.7.2"
---
//! # Opener example that takes a custom string.
//! example with `BROWSER` env var
//!
//! ## Run:
//! `./opener-example.rs`
//! or
//! `BROWSER='...' ./opener-example.rs`
//! 
//! ## Convenience note:
//! `chmod u+x opener-example.rs`

use clap::Parser;

/// Use `BROWSER` envvar to test `opener` crate function.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Optional path to open. (will be prepended with `https://` if not present.)
    opt_path: Option<String>,
}

const REQ_PREFIX: &str = "https://";
const DEFAULT_PATH: &str = "https://www.rust-lang.org";

fn main() {
    let args = Args::parse();
    let mut path = args.opt_path.unwrap_or(DEFAULT_PATH.to_string());
    if !path.starts_with(REQ_PREFIX) {
        println!("prepending required prefix: {:?}", REQ_PREFIX);
        path = format!("{}{}",REQ_PREFIX, path);
    }
    println!("path to take: {:?}", path);

    opener::open_browser(path).unwrap();
}
