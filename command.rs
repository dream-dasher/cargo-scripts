#!/usr/bin/env cargo +nightly -Zscript
---
[dependencies]
---
//! Command xp
//! use with `BROWSER` env var
//!
//! ## Run:
//! ```zsh
//! clear; echo "hi there" | ./command.rs
//! clear; ./command.rs
//! ```
//!
//! ## Convenience note:
//! `chmod u+x opener-example.rs`

use std::process::{Command};
use std::io::*;
use std::io;
use std::io::IsTerminal;

fn main() -> core::result::Result<(), Box<dyn core::error::Error>>{

        let mut buffer = String::new();
        if !io::stdin().is_terminal() {
            io::stdin().read_to_string(&mut buffer)?;
            println!("Piped input: {}", buffer);
        }

        let stdin = io::stdin();
        if stdin.is_terminal() {
            println!("stdin is a terminal");
        } else {
            println!("stdin is not a terminal");
        }

        let status = Command::new("echo")
            .arg(r#"`hi"b.l;1\n'23'""#)
            .arg("PATH")
            .arg("$PATH")
            // .env("PATH")
            .spawn()?;

        println!("status: {:?}", status);


        Ok(())
}
