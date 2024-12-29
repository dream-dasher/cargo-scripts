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

use std::process::{Command, Stdio};
use std::io::*;
use std::io;
use std::io::IsTerminal;
use std::env;
use std::io::Write;

fn main() -> core::result::Result<(), Box<dyn core::error::Error>>{
        // let mut buffer = String::new();
        // if !io::stdin().is_terminal() {
        //     io::stdin().read_to_string(&mut buffer)?;
        //     println!("Piped input: {}", buffer);
        // }
        // let stdin = io::stdin();
        // if stdin.is_terminal() {
        //     println!("stdin is a terminal");
        // } else {
        //     println!("stdin is not a terminal");
        // }
        println!("-----------------------------");
        let path_val = env::var("PATH").expect(r#""PATH" not found."#);
        let path_vals: Vec<_> = path_val.split(':').collect();
        println!("pathval: {path_vals:#?}");

        // echo $PATH | sd : '\n' | xargs -I_ fd '.*' _ -t f | sort
        // let spwn_rg = std::process::Command::new("sd")

        let echo= Command::new("echo")
                .args(path_vals)
                .stdout(Stdio::piped())
                .spawn()?;
        let mut sd= Command::new("sd")
                .args([r#" "#, r#"\n"#])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
        let echo_out = echo.wait_with_output().unwrap();
        println!("\necho out: {:?}\n", &echo_out);
        sd.stdin.take().unwrap().write_all(&echo_out.stdout).unwrap();
        let sd_out = sd.wait_with_output().unwrap();
        println!("\nsd out: {:?}\n", &sd_out);

        println!("-----------------------------");
        // Piping from one process to another
        // - defining commands -
        let mut ls = Command::new("ls");
        let mut cat = Command::new("cat");
        ls.stdout(Stdio::piped());
        cat.stdin(Stdio::piped());
        cat.stdout(Stdio::piped());


        // - background spawning -
        let ls = ls.spawn().unwrap();
        let mut cat = cat.spawn().unwrap();

        // - get output 1 -
        let ls_out = ls.wait_with_output().unwrap();
        println!("\nls_out: {:?}", &ls_out);
        // - grab proc2's sdtin and write to it -
        cat.stdin.take().unwrap().write_all(&ls_out.stdout).unwrap();
        // - get output 2 -
        let cat_out = cat.wait_with_output().unwrap();

        println!("\ncat_out: {}", String::from_utf8(cat_out.stdout).unwrap());
        println!("-----------------------------");
        Ok(())
}
