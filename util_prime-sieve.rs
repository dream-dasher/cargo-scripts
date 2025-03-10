#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap = { version = "4", features = ["derive"] }
owo-colors = "4.1"
---
//! # Cargo-Script: util_prime-sieve
//!
//! Gets slow around 100 million in debug mode and 1 billion in release mode.
//! 100_000_000
//! 1_000_000_000
//! (note: doesn't seem to be a way to enter numbers with `_` separators via cli for clap)
//!
//! Faster than I'd have guessed given its rather ... direct approach.
//!
//! (I wonder if the index calculations, skipping indices with false and jumping to P*n indicces, have any special
//! performances advantages.)
//!
//! ## Convenience Section
//!
//! ### Shell Commands
//! - direct
//!   - `chmod u+x util_prime_sieve.rs`
//!   - `./util_prime-sieve.rs`
//! - via cargo
//!   - `cargo +nightly -Zscript util_prime-sieve.rs`
//! - other cargo commands
//!   - `cargo +nightly -Zscript COMMAND *ARGS --manifest-path util_prime-sieve.rs`
//!
//! ### Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)
use std::{error::Error, result::Result, time::Instant};

use clap::Parser;
use owo_colors::OwoColorize;

/// Very simple, almost hyper-'literal' eratosthenes-sieve.
///
/// For quick results in debug mode : stop around 100_million searched.
/// For quick Results in release mode: stop around 1_billion searched.
///
/// Mostly for simple play, but useful int hos ranges.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// Calculate all primes until some number (inclusive)
        primes_until: Option<usize>,

        /// Only show primes above this number
        #[arg(short='n', long="min")]
        primes_from: Option<usize>,

        /// Show all primes found
        #[arg(short, long)]
        show: bool,
        /// Show timing of core prime calculation and reaping. (Ignoring UI and display times.)
        #[arg(short, long = "time")]
        time_calc:    bool,
}
fn main() -> Result<(), Box<dyn Error>> {
const DEFAULT_PRIMES_TILL: usize = 12_345;
let args = Args::parse();
let primes_from_or_default = args.primes_from.unwrap_or(0);
let primes_till_or_default = match args.primes_until {
        None => {
                println!(
                        "No `{}` input given, defaulting to : {}",
                        "primes_until".green(),
                        DEFAULT_PRIMES_TILL.cyan()
                );
                DEFAULT_PRIMES_TILL
        }
        Some(p) => {
                println!("You requested primes up to: {}", p.blue());
                p
        }
};
println!(
        "Calculating primes from ({}..={})...",
        primes_from_or_default.blue(),
        primes_till_or_default.blue()
);
if primes_from_or_default > primes_till_or_default {
        Err("Error: your minimum is larger than your maximum.  Cancelling search.")?
};
let start_time = Instant::now();
let found_primes = prime_sieve(args.primes_from, primes_till_or_default);
let finish_duration = start_time.elapsed();
println!(
        "Number of primes found <= {}: {}",
        primes_till_or_default.blue(),
        found_primes.len().green().bold()
);
println!(
        "which makes the range ({}..={}) {:.1}% prime.",
        primes_from_or_default.blue(),
        primes_till_or_default.blue(),
        (100. * (found_primes.len() as f32)
                / ((primes_till_or_default - primes_from_or_default) as f32 + 2.))
                .cyan()
                .bold()
);
if args.show {
        println!("{:?}", found_primes.magenta());
}
if args.time_calc {
        println!("Time taken: {:?}", finish_duration.red());
}
Ok(())
}

/// I'll be surprised if this works efficiently as a mechanical, literal, procedure.
fn prime_sieve(min: Option<usize>, max: usize) -> Vec<usize> {
        // buncha default yes's
        let mut primes = vec![true; max + 1];
        primes[0] = false;
        primes[1] = false;
        // no need to go past sqrt(n).floor()
        for i in  2..=max.isqrt() {
                // skip if index was marked as multiple of preceding num
                if primes[i] {
                        // first value that's not been sieved would require p >= us, which would be us
                        let mut index = i * i;
                        // false for al p * n indices
                        while index <= max {
                                primes[index] = false;
                                index += i;
                        }
                }
        }
        let min = min.unwrap_or(0);
        // collect unsieved bits
        let mut result = vec![];
        for (i, b) in primes.iter().enumerate().skip(min) {
                if *b {
                        result.push(i);
                }
        }
        result
}
