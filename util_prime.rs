#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap = { version = "4", features = ["derive"] }
---
//! # Cargo-Script: scratch_prime
//! 
//! ## Convenience Section
//!
//! ### Shell Commands
//! - direct
//!   - `chmod u+x scratch_prime.rs`
//!   - `./scratch_prime.rs`
//! - via cargo
//!   - `cargo +nightly -Zsscript scratch_prime.rs`
//! - other cargo commands
//!   - `cargo +nightly -Zscript COMMAND *ARGS --manifest-path scratch_prime.rs`
//!
//! ### Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)
use std::{error::Error, result::Result};

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();
        let primes_from = args.primes_from.unwrap_or(0);
        let primes_till = match args.primes_till {
                None => {
                        println!("Hi from scratch_prime.rs.  No primes_till given, defaulting to : 12_345");
                        12_345
                        
                }
                Some(p) => {
                        println!("Hi from scratch_prime.rs.  You requested primes for said: {}", p);
                        p
                }
        };
        println!("Calculating primes from ({primes_from}..={primes_till})...");
        if primes_from > primes_till { Err("Error: your minimum is larger than your maximum.  Cancelling search.")? };

        let found_primes = prime_sieve(args.primes_from, primes_till);
        println!("Number of primes found <= {primes_till}: {}", found_primes.len());
        println!("which makes the range ({primes_from}..={primes_till}) {:.1}% prime.", 100.*(found_primes.len() as f32)/(primes_till as f32 + 2.));
        if args.show {
                println!("{:?}", found_primes);
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


/// scratch_prime Cargo-Script
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// Calculate all primes till some number
        primes_till: Option<usize>,
        /// Only show primes above this number
        #[arg(short='n', long="min")]
        primes_from: Option<usize>,
        /// Show all primes found
        #[arg(short, long)]
        show: bool
}