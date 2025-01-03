#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap = { version = "4", features = ["derive", "env"] }
config = "0.15.4"
directories = "5.0.1"
dotenvy = "0.15.7"
owo-colors = "4.1.0"
rexpect = { version = "0.6.0", features = ["which"] }
toml = "0.8.19"
termtree = "0.5.1"
---
// human-panic = "2.0.2"
//! # Cargo-Script: cli-accessories.rs
//!
use std::{env, error::Error, fs::File, path::PathBuf, result::Result};

use std::collections::HashMap;
use config::Config;
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();
        if args.wink { println!(";)"); }
        let config_path = args.config_file.unwrap_or("rustfmt.toml".into());

        let settings = Config::builder()
            .add_source(config::File::with_name(config_path.to_str().expect("Path expected UTF-8")))
            .build()
            .unwrap();
        println!("settings: {:#?}", settings);

        Ok(())
}

/// cli-accessories.rs Cargo-Script
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// String Arg
        // config_file: Vec<PathBuf>,
        config_file: Option<PathBuf>,
        /// Boolean Flag
        #[arg(short, long)]
        wink: bool
}
