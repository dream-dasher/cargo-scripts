#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
derive_more = { version="1", features=["display", "error", "from"] }
tracing-error = "0.2.1"
---
//! # Cargo-Script: error-wrap
//! 
//! ## Purpose
//! Example of Error-Wrapper pattern w/ custom `From<E>` to populate fields.
//! Both the nightly `backtrace` and 3rd party `spantrace` are good candidates
//! for wrapper auto-fills; capturing local context.
//!
//! ### Crate links
//! - [derive_more](https://docs.rs/derive_more/latest/derive_more/)
//!   - allows auto-derivation of errors with backtrace; and generall useful
//!   - [this_error](https://docs.rs/thiserror/latest/thiserror/) works similarly
//! - [backtrace](https://doc.rust-lang.org/std/backtrace/index.html)
//!   - requires nightly
//! - [tracing_error](https://docs.rs/tracing-error/latest/tracing_error/)
//!   - tracing is amazing, but has crazy-makign docs and api design
//!   - this sub-crate is no exception; but tldr: you can have it auto-grab
//!   - any active tracing spans.  (note: tracing spans are thread-local)
//! 
//! ## Cargo-Script Notes
//! ### Shell Commands
//! - direct
//!   - `chmod u+x error-wrap.rs`
//!   - `./error-wrap.rs`
//! - via cargo
//!   - `cargo +nightly -Zsscript error-wrap.rs`
//! - other cargo commands
//!   - `cargo +nightly -Zscript COMMAND *ARGS --manifest-path error-wrap.rs`
//!
//! ### Cargo-Script Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)
#![feature(error_generic_member_access)]

use std::{backtrace, io};
use derive_more::{Display, Error, From};
type OurResult<A> = std::result::Result<A, ErrWrapper>;

// // // // // // // // // // Demonstration-of-use Code // // // // // // // // // //

fn main() -> OurResult<()> {
        println!("Hello from error-wrap.rs!");

        Ok(())
}



///////////////////////// ErrKing & ErrWrap Pattern Examples ///////////////////////// 

/// "Core" Error Enum
/// enumerates the types (and sources) of errors we expect to see
///  `derive_more::From` will auto generate code to bring corresponding types into variants
///  we use `#[from(ignore)]` for variants that couldn't be unamibiguasly coerced 
/// from their inner member. (e.g. many custom errors)
/// 
/// We also defined two `Other...` error kinds.
/// These are helpful when developing or experimenting so that we don't have to trace
/// and add every new error source as we come upon it.
#[derive(Debug, Display, From, Error)]
pub enum ErrKind {
        // `custom` errors
        #[from(ignore)]
        #[display("Unparsable character: {}", source_char)]
        ParseOther { source_char: char },
        // `repackaged` errors
        #[display("io error: {}", source)]
        Io { source: io::Error },
        // `other` errors
        #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
        #[display("Uncategorized Error (dyn error object): {}", source)]
        OtherErrorDyn {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
        #[display(r#"Uncategorized string err: "{}""#, source_string)]
        OtherErrorString { source_string: String },
}
impl ErrKind {
        /// This is a convenience function for creating `OtherErrorDyn`
        pub fn make_dyn_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherErrorDyn { source: error.into() }
        }
}

/// This is our Error Wrapper.  A struct, unlike the earlier enum.
/// The heart of the error is `ErrKind` above.
/// We can add additional fields for manual or automatic enrichment.
/// Below we've added a `backtrace` and `spantrace` field.
#[derive(Display, Error, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}\n\n\nbacktrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
        backtrace,
)]
pub struct ErrWrapper {
        pub source:    ErrKind,
        pub spantrace: tracing_error::SpanTrace,
        pub backtrace: backtrace::Backtrace,
}
// Using custom display as debug so we can get SpanTrace auto preetty-printed.
impl std::fmt::Debug for ErrWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}
// This is what makes the Error wrapper pattern practical.
// Whenever we write
// ```rust
// fallible_thing()?
// ```
// We're saying that if there's an error then run `.into()` on it
// and return it.
//
// So we customize our `.into()` to auto-populate with information we
// consider useful.
impl<T> From<T> for ErrWrapper
where
        T: Into<ErrKind>,
{
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        backtrace: backtrace::Backtrace::capture(),
                }
        }
}

pub trait ToOther {
        fn to_other(self) -> ErrWrapper;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> ErrWrapper {
                ErrKind::OtherErrorDyn { source: self.into() }.into()
        }
}
