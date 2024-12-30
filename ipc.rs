#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
---
//! # Cargo-Script: ipc.rs
//! 
//! ## Convenience Section
//!
//! ### Shell Commands
//! - run directly
//!   - `chmod u+x ipc.rs.rs`
//!   - `./ipc.rs.rs`
//! - run via cargo
//!   - `cargo +nightly -Zsscript ipc.rs.rs`
//! - run other commands on script
//!   - `cargo +nightly COMMAND *ARGS --manifest-path ipc.rs.rs -Zscript`
//!     - e.g. `cargo +nightly update --manifest-path ipc.rs.rs -Zscript`
//!
//! ### Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)

use std::{env, error::Error, io::Write, 
          process::{Command, Stdio}, 
          result::Result};

fn main() -> Result<(), Box<dyn Error>> {
        println!("Hello from ipc.rs.rs!");

        let shell_path = env::var("PATH").expect(r#""PATH" not found."#);
        let path_vals: Vec<_> = shell_path.split(':').collect();
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
