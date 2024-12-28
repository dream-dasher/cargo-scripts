#!/usr/bin/env cargo +nightly -Zscript
---
[dependencies]
clap = { version = "4.5", features = ["derive"] }
---
//! Mod Doc
//!

use clap::Parser;

/// Description.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// String Arg
        argument: String,
}

fn main() -> core::result::Result<(), Box<dyn core::error::Error>>{
        let args = Args::parse();

        Ok(())
}
