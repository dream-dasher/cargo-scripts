#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
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

use std::{error::Error, result::Result};

fn main() -> Result<(), Box<dyn Error>> {
        let v5 = vec![0, 1, 2, 3, 4];
        let v5r = reverse(&v5);
        println!("orig. {:?}", v5);
        println!("revr. {:?}", v5r);

        Ok(())
}

/// Intentionaly drops an element
fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
        let mut rev = vec![];
        for i in 1..xs.len() {
                println!("{i}");
                rev.insert(0, xs[i].clone())
        }
        rev
}
