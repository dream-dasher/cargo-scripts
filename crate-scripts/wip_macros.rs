#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
---
//! # Cargo-Script: wip_macro
//! 
//! ## Convenience Section
//!
//! ## relevant docs
//! - [Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/README.html)
//! - [Log Rocket macro tutorial](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#whatarerustmacros)
//! - [dtolnay's proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop)
//!   - doesn't really belong here, but leaving it here until/unless I make a proc-macro cscript
//! 
//! ## of Note
//! - `#[macro_escape]`
//!   - results in a macros definition escaping it's formal file, and being usable in a broader lexical scope
//!   - think of (?) `mod` as dropping files into one another
//!   - this means, if you want the macro to be generally accessible, you need to define it high in the 'single' effective file
//!     - e.g. in first mod referenced or at top of lib/rs/main.rs
//! - `#[macro_export]`
//!   - allows explicit exporting of macro *outside of crate*
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


        /// Matching with recursion.
        /// NOTE: expansion appears (?) to occur based on *specific* arguments in code.
        ///       which means that arbitrarily deep recursion like this is safe to define.
        macro_rules! add_rec {
            () => { 0 };
            ($a:expr) => { $a };
            ($a:expr,$($b:tt)*) => { $a + add_rec!($($b)*) }
        }
        dbg!(add_rec!());
        dbg!(add_rec!(1));
        dbg!(add_rec!(1,2));
        dbg!(add_rec!(1,2,3));
        dbg!(add_rec!(1,2,3,4));
        dbg!(add_rec!(1,2,3,4,5));

        /// Note this is recursion without clear terminal conditions...
        macro_rules! add_rec_also_works {
            () => { 0 };
            ($a:expr) => { $a };
            ($a:expr,$($b:tt)*) => { add_rec_also_works!($($b)*) + add_rec_also_works!($($b)*) }
        }
        dbg!(add_rec_also_works!());
        dbg!(add_rec_also_works!(1));
        dbg!(add_rec_also_works!(1,2));
        dbg!(add_rec_also_works!(1,2,3));
        dbg!(add_rec_also_works!(1,2,3,4));
        dbg!(add_rec_also_works!(1,2,3,4,5));

        /// Note this is recursion without clear terminal conditions...
        macro_rules! add_rec_with_diagnostic_print {
            () => {{ println!("0"); 0 }};
            ($a:expr) => {{println!("1"); $a}};
            ($a:expr,$b:expr) => {{println!("2"); add_rec_with_diagnostic_print!($a) + add_rec_with_diagnostic_print!($b) }};
            ($a:expr,$($b:tt)*) => {{println!("3"); $a + add_rec_with_diagnostic_print!($($b)*) }}
        }
        dbg!(add_rec_also_works!());
        dbg!(add_rec_also_works!(1));
        dbg!(add_rec_also_works!(1,2));

        Ok(())
}
