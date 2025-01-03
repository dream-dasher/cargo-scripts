#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
profile.dev.opt-level=3
[dependencies]
clap = { version = "4", features = ["derive"] }
faer = "0.20.1"
indoc = "2.0.5"
owo-colors = "4.1.0"
rand = "0.8.5"
---
// rand = { version = "0.8.5", features = ["packed_simd", "simd_support"] }
//! # Cargo-Script: zlib-faer - Fast (nongpu) Matrix Operations
//! [Faer User Guide](https://faer-rs.github.io/dense.html)
//! 
use std::{error::Error, result::Result};

use clap::Parser;
// use faer::{prelude::*, sparse::FaerSparseMat, SparseColMat, Side};
use faer::prelude::*;
// use faer::sparse::FaerSparseMat;
use faer::Side;
use faer::sparse::SparseColMat;
use faer::{mat, scale, Mat};
use indoc::printdoc;

fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();

        let a = mat![
            [1.0, 5.0, 9.0],
            [2.0, 6.0, 10.0],
            [3.0, 7.0, 11.0],
            [4.0, 8.0, 12.0f64],
        ];
        println!("a: {:?}", a);

        let b = Mat::<f64>::from_fn(4, 3, |i, j| (i + j) as f64);
        println!("b: {:?}", b);
        
        let add = &a + &b;
        let sub = &a - &b;
        let scale = scale(3.0) * &a;
        let mul = &a * b.transpose();
        printdoc!("
                add: {add:?},
                sub: {sub:?},
                scale: {scale:?},
                mul: {mul:?}\n");
        
        let a00 = a[(0, 0)];
        println!("a00: {a00}");

        Ok(())
}

/// zlib-faer Cargo-Script
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
        /// String Arg
        argument: Option<u8>,
        /// Boolean Flag
        #[arg(short, long)]
        wink: bool
}
