#!/usr/bin/env cargo +nightly -Zscript
---
[dependencies]
---
//! # Cargo-Script: {{sd_me: script_name}}
//! 
//! ## Convenience Section
//!
//! ### Shell Commands
//! - run directly
//!   - `chmod u+x {{sd_me: script_name}}.rs`
//!   - `./{{sd_me: script_name}}.rs`
//! - run via cargo
//!   - `cargo +nightly -Zsscript {{sd_me: script_name}}.rs`
//! - run other commands on script
//!   - `cargo +nightly COMMAND *ARGS --manifest-path {{sd_me: script_name}}.rs -Zscript`
//!     - e.g. `cargo +nightly update --manifest-path {{sd_me: script_name}}.rs -Zscript`
//!
//! ### Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)

fn main() {
        println!("Hello from {{sd_me: script_name}}.rs!");
}
