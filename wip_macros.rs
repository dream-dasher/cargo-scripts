#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
---
//! # Cargo-Script: wip_macro
//! 
//! ## Convenience Section
//!
//! ### Shell Commands
//! - direct
//!   - `chmod u+x wip_macro.rs`
//!   - `./wip_macro.rs`
//! - via cargo
//!   - `cargo +nightly -Zsscript wip_macro.rs`
//! - other cargo commands
//!   - `cargo +nightly -Zscript COMMAND *ARGS --manifest-path wip_macro.rs`
//!
//! ### Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)
use std::{env, error::Error, result::Result};
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn Error>> {
        println!("Hello from wip_macro.rs!");

        /// Add two numbers, each first coerced to a type
        macro_rules! add_as {
            ($a:expr, $b:expr, $typ:ty) => { $a as $typ + $b as $typ }
        }

        dbg!(add_as!(1,1,u8));
        dbg!(add_as!(1,1,i8));
        dbg!(add_as!(1,1,f32));


        /// Add two numbers, each first coerced to a type, then give the name of the type as a string
        macro_rules! add_as_and_say_type {
            ($a:expr, $b:expr, $typ:ty) => { ( $a as $typ + $b as $typ, std::any::type_name::<$typ>() ) }
        }

        dbg!(add_as_and_say_type!(1,1,u8));
        dbg!(add_as_and_say_type!(1,1,i8));
        dbg!(add_as_and_say_type!(1,1,f32));

        Ok(())
}
