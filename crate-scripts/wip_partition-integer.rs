#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap = { version = "4.5", features = ["derive"] }
itertools = "0.13"
---
//! # Number of ways to sum an integer in parts, ignoring order
//! [wikipedia link: Integer Partition](https://en.wikipedia.org/wiki/Integer_partition)
//!
//!
//! 5  
//! 4 1        - 4
//! 3 2  - 3     3 1  - 3
//! 2 3    2 1   2 1    2 1  - 2
//! 1 4                         1 1
//!
//!
//! ## Run:
//! `./partition-integer.rs`
//! 
//! ## Convenience note:
//! `chmod u+x partition-integer.rs`
use clap::Parser;
use itertools::Itertools;
fn main() {
        let args = Args::parse();
        let sol_vec = partition_single_int(args.integer);
        println!("sollution with redundancies: {:?}", sol_vec);
        todo!("Logic error: redundant solutions.  Perf error: lots of repeated work.")
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Integer to partition.
    integer: u8,
}

pub fn partition_single_int(int: u8) -> Vec<Vec<u8>> {
        if int == 0 {
                return vec![vec![0]];
        }
        if int == 1 {
                return vec![vec![1]];
        }
        if int == 2 {
                return vec![vec![2], vec![1, 1]];
        }

        let mut sol = vec![vec![int]];
        for i in int.div_ceil(2)..int {
                let g_sol = partition_single_int(int - i);
                let l_sol = partition_single_int(i);
                for (mut gv, mut lv) in g_sol.into_iter().cartesian_product(l_sol.into_iter()) {
                        gv.append(&mut lv);
                        sol.push(gv);
                }
        }
        sol
}
