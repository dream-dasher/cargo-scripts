#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap = { version = "4", features = ["derive", "env"] }
directories = "5.0.1"
dotenvy = "0.15.7"
owo-colors = "4.1.0"
rexpect = { version = "0.6.0", features = ["which"] }
toml = "0.8.19"
---
// config = "0.15.4"
// human-panic = "2.0.2"
//! # Cargo-Script: cli-accessories.rs
//! 
use std::{env, error::Error, result::Result};

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();
        println!("Hi from cli-accessories.rs.rs.  You said: {}", args.argument);
        if args.wink { println!(";)"); }
        Ok(())
}

/// cli-accessories.rs Cargo-Script
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// String Arg
        argument: String,
        /// Boolean Flag
        #[arg(short, long)]
        wink: bool
}
