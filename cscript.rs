#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap = { version = "4", features = ["derive"] }
---
//! # cscript.rs
//! Just an example script.
//!
//! run with:
//! ```zsh
//! SCRIPT=cscript.rs
//! ARGS_S=('hi there, friend' -w -t)
//! CMD=run
//! ARGS_C=()
//! cargo +nightly $CMD $ARGS_C -Zscript --manifest-path $SCRIPT -- $ARGS_S
//! ```

use std::{error::Error, process::Command, result::Result};

use clap::Parser;

/// cscript.rs (example cargo script)
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// String Arg
        argument: String,
        /// Boolean Flag
        #[arg(short, long)]
        wink:     bool,
        #[arg(short, long)]
        tongue:   bool,
        #[arg(short, long)]
        ashen:    bool,
}

fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();
        if args.argument != "list" {
                println!("Hi from cscript.rs.  You said: {}", args.argument);
        } else {
                println!("You said: {}. So we're running `ls`:\n", args.argument);
                Command::new("ls").status().expect("ls command should run");
                if args.wink || args.tongue || args.ashen {
                        println!();
                }
        }

        if args.wink {
                println!(";)");
        }
        if args.tongue {
                println!(":P");
        }
        if args.ashen {
                println!("8|");
        }
        Ok(())
}
