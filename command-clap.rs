#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
clap = { version = "4.5", features = ["derive"] }
---
//! # Command xp w/ clap

use clap::Parser;
use std::process::{Command, Stdio};

/// Use with `BROWSER` envar
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// Optional string to print
        opt_argument: Option<String>,
        opt_runner: Option<String>,
}

const DEFAULT_ARGUMENT: &str = "---Hi there, from default string.---\n";
const DEFAULT_RUNNER: &str = "echo";
fn main() -> core::result::Result<(), Box<dyn core::error::Error>>{
        let args = Args::parse();
        let runner = args.opt_runner.unwrap_or(DEFAULT_RUNNER.to_string());
        let argument = args.opt_argument.unwrap_or(DEFAULT_ARGUMENT.to_string());
        println!("rust: Argument: {}", argument);
        println!("rust: Runner: {}", runner);
        println!();
        let proc_handle = Command::new(runner)
                .arg(argument)
                .stdin(Stdio::null())
                // .stdout(Stdio::piped())
                .stdout(Stdio::inherit())
                .stderr(Stdio::piped())
                .spawn()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        let out = proc_handle.wait_with_output()?;
        println!("Spawned process output:\n{:#?}", out);
        Ok(())
}
