#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
derive_more = { version="1", features=["display", "error", "from"] }
tracing-error = "0.2.1"
---
//! # Cargo-Script: error-wrap
//! 
//! ## Convenience Section
//!
//! ### Shell Commands
//! - direct
//!   - `chmod u+x error-wrap.rs`
//!   - `./error-wrap.rs`
//! - via cargo
//!   - `cargo +nightly -Zsscript error-wrap.rs`
//! - other cargo commands
//!   - `cargo +nightly -Zscript COMMAND *ARGS --manifest-path error-wrap.rs`
//!
//! ### Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)
#![feature(error_generic_member_access)]

use std::{backtrace, io};
use derive_more::{Display, Error, From};
type OurResult<A> = std::result::Result<A, ErrWrapper>;

fn main() -> OurResult<()> {
        println!("Hello from error-wrap.rs!");
        Ok(())
}


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
        pub fn make_dyn_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherErrorDyn { source: error.into() }
        }
}

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
// Using custom display as debug so we can get SpanTrace auto printed.
impl std::fmt::Debug for ErrWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}
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
