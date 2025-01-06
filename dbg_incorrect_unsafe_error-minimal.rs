#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
derive_more = { version = "1.0", features = ["display"] }
---
//! # Debug Example
//!  rust-analyzer incorrect report
//!
//! rust-analyzer 1.86.0-nightly (b3b368a1 2025-01-05)
//! rustc 1.86.0-nightly (b3b368a18 2025-01-05)
//! Zed 0.167.1 – /Applications/Zed.app
//! helix 25.1 (dabfb6ce)
//!
//! rust-analyzer incorrectly reports that there is an unsafe function call - BUT only within ide.
//! There's no error when using `cargo check --example` or `cargo clippy --example`
//! The error is reported for the call to create an array with a fixed value.
//! The same call, in another function, does not create an issue.
//!

fn main() {
        let an_array: [bool; 3] = [false; 3];
        let vec_of_arrays = vec![an_array];
        print!("{:?}", vec_of_arrays);
}
