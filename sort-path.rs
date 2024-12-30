#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
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

use std::env;
use std::io::{IsTerminal, Write, *};
use std::io;
use std::process::{Command, Stdio};

fn main() {
        println!("-----------------------------");
        let shell_path = env::var("PATH").expect(r#""PATH" not found."#);
        let path_vals: Vec<_> = shell_path.split(':').collect();
        println!("pathval: {path_vals:#?}");
}
