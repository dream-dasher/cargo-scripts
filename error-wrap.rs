#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
derive_more = { version="1.0.0", features=["display", "error", "from"] }
tracing-error = "0.2.1"
tracing-subscriber = "0.3.19"
tracing = "0.1.41"
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
//!   - allows auto-derivation of errors with backtrace; and generally useful
//!   - [this_error](https://docs.rs/thiserror/latest/thiserror/) works similarly
//! - [backtrace](https://doc.rust-lang.org/std/backtrace/index.html)
//!   - requires nightly
//! - [tracing_error](https://docs.rs/tracing-error/latest/tracing_error/)
//!   - tracing is amazing, but has crazy-making docs and api design
//!   - this sub-crate is no exception; but tldr: you can have it auto-grab
//!   - any active tracing spans.  (note: tracing spans are thread-local)
//! 
//! ## Cargo-Script Notes
//! ### Call with Cargo
//! ```shell
//! clear
//! cargo +nightly -Zscript error-wrap.rs
//! ````
//! add `RUST_BACKTRACE=1` to see backtrace
//! 
//! ### Cargo-Script Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)
//! 
#![feature(error_generic_member_access)]

use std::{backtrace, io};

use derive_more::{Display, Error, From};
use tracing_error;

pub type OurResult<A> = std::result::Result<A, ErrWrapper>;


// // // // // // // // // // Demonstration-of-use Code // // // // // // // // // //
use tracing::{trace, debug, info, info_span, debug_span, instrument};
use tracing_subscriber::{prelude::*, filter::LevelFilter};

fn main() -> OurResult<()> {
        let _writer = tracing_subscriber_shortened_boilerplate()?;
        let _entered = debug_span!("Main springs.").entered();
        info!("A friendly fluff message.");
        info!("Hello from error-wrap.rs!");

        // using a custom-error
        {
                let now = std::time::Instant::now();
                let _entered = info_span!("Measuring duration", ?now).entered();
                loop {
                        let time_so_far=now.elapsed();
                        trace!(?time_so_far);
                        if time_so_far.as_micros().rem_euclid(7) == 0 {
                                let source_dur = time_so_far;
                                // We defined this custom error in our enum
                                Err(ErrKind::SuperstitiousConcern {source_dur})?
                        }
                        if time_so_far.as_micros().rem_euclid(5) == 0 { break }
                }
        }

        // using an outside-error (ParseIntError from std)
        {
                let strs_to_parse: [&str; 6] = ["1","22","333","4444","55555","sixsix"];
                let _entered = debug_span!("parsing", items=strs_to_parse.len()).entered();
                for s in strs_to_parse {
                        let n = trim_double_parse_nest(s)?;
                        debug!("{}", n);
                }
        }
        Ok(())
}
/// Just something nesty to give `spantrace` & `backtrace` more to do
/// 1/3
#[instrument(skip_all)]
fn trim_double_parse_nest(s: impl AsRef<str>) -> OurResult<u64> {
        let so_clean = s.as_ref().trim();
        double_parse_nest(so_clean.to_string())
}
/// Just something nesty to give `spantrace` & `backtrace` more to do
/// 2/3
#[instrument]
fn double_parse_nest(s: String) -> OurResult<u64> {
        let so_long = format!("{0}{0}", s);
        parse_nest(so_long)
}
/// Just something nesty to give `spantrace` & `backtrace` more to do
/// 3/3
#[instrument]
fn parse_nest(s: String) -> OurResult<u64> {
        Ok(s.parse()?)
}

/// Shortened version of an actual tracing_subscriber boilerplate function.
pub fn tracing_subscriber_shortened_boilerplate() -> OurResult<()> {
        const OUTPUT_LOGGING_LEVEL: LevelFilter = LevelFilter::TRACE;
        const ERROR_LOGGING_LEVEL: LevelFilter = LevelFilter::TRACE;

        let error_layer = tracing_error::ErrorLayer::default()
                .with_filter(ERROR_LOGGING_LEVEL);
        let fmt_layer = tracing_subscriber::fmt::Layer::default()
                .with_filter(OUTPUT_LOGGING_LEVEL);

        let subscriber = tracing_subscriber::Registry::default()
                .with(error_layer)
                .with(fmt_layer);

        tracing::subscriber::set_global_default(subscriber)?;
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
        #[display("Kismet, weird, chance better not to taken.  Tiny son of seven: {:?}", source_dur)]
        SuperstitiousConcern { source_dur: std::time::Duration},

        // `repackaged` errors
        #[display("io error: {}", source)]
        Io { source: io::Error },

        #[display("parse error: {}", source)]
        ParseInt { source: std::num::ParseIntError },

        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: tracing::subscriber::SetGlobalDefaultError },

        // `other` errors
        #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
        #[display("Uncategorized Error (dyn error object): {}", source)]
        OtherErrorDyn { source: Box<dyn std::error::Error + Send + Sync> },

        #[display(r#"Uncategorized string err: "{}""#, source_string)]
        OtherErrorString { source_string: String },
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





////\\\\\\|||||\\\\\\//// Example Convenience Extensions ////\\\\\\|||||\\\\\\//// 

// Neither of these are necessary and they are redundant in terms of content.
// But these are examples of how to add easy convenience functions for
// grabbing errors dynamically.
// (Which can be helpful ergonomics when exploring code, debugging, etc.)
// 
// One is simply an impl for our error kind, which can then be passed up.
// The other is a trait that auto applies to anything that we could wrap in
// a regular dyn error, but wrapped up nicely in our errorwrapper


pub trait ToOther {
        fn to_other(self) -> ErrWrapper;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        /// This is another convenience function like the one above.
        fn to_other(self) -> ErrWrapper {
                ErrKind::OtherErrorDyn { source: self.into() }.into()
        }
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
