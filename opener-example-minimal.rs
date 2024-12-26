#!/usr/bin/env cargo +nightly -Zscript
---
[dependencies]
opener = "0.7.2"
---
//! Minimal Example of Opener function.
//! example with `BROWSER` env var
//!
//! ## Run:
//! `./opener-example-minimal.rs`
//! or
//! `BROWSER='...' ./opener-example-minimal.rs`
//! 
//! ## Convenience note:
//! `chmod u+x opener-example-minimal.rs`

const DEFAULT_PATH: &str = "https://www.rust-lang.org";

fn main() {
    opener::open_browser(DEFAULT_PATH).unwrap();
}



