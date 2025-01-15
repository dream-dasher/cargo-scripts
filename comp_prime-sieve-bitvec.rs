#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap = { version = "4", features = ["derive"] }
bitvec = "1.0.1"
---
//! # Compare w/ `util_prime-sieve` 
//! bitvec replacement for Vec<bool> 
//!
//! ## Desc
//! using `bitvec`, with single bits representing bools, vs whole bytes.
//! As a raw replacement it performs much *slower* than the native byte implementation.
//! (both in debug and release modes)
//! Low times are dominated by the cargo call machinery.
//! But at 1billion in release we're looking at about 7 vs 10 seconds.
//!
//! ## Alt
//! Might pre-calculate per-byte setting.  Not that interested in pursuing this without setting up better assembly level visualization.
//! 
//! 
//! ### Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)
use std::{error::Error, result::Result};

use bitvec::prelude::*;
use clap::Parser;

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
fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();
        let primes_from = args.primes_from.unwrap_or(0);
        let primes_till = match args.primes_till {
                None => {
                        println!("Hi from scratch_prime.rs.  No primes_till given, defaulting to : 12_345");
                        12_345
                        
                }
                Some(p) => {
                        println!("Hi from scratch_prime.rs.  You requested primes through: {}", p);
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
        // let mut primes = vec![true; max + 1];
        let mut primes: BitVec = bitvec![1; max + 1];
        *primes.get_mut(0).unwrap() = false;
        *primes.get_mut(1).unwrap() = false;
        // no need to go past sqrt(n).floor()
        for i in  2..=max.isqrt() {
                // skip if index was marked as multiple of preceding num
                if primes[i] {
                        // first value that's not been sieved would require p >= us, which would be us
                        let mut index = i * i;
                        // false for al p * n indices
                        while index <= max {
                                *primes.get_mut(index).unwrap() = false;
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
