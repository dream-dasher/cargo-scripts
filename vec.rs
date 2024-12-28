#!/usr/bin/env cargo +nightly -Zscript
---
[dependencies]
---
//! Vec : just playing with with some vector methods.
//!

fn main() {
        println!("Hello from a rust script!");
        let mut vv = vec!['a','b','c'];
        println!("swap_remove(1): {}", vv.swap_remove(1));
        println!("swap_remove(1): {}", vv.swap_remove(1));
        // println!("swap_remove(1): {}", vv.swap_remove(1)); <-- would panic, out of bounds}
        println!("swap_remove(0): {}", vv.swap_remove(0)); // <-- allowed despite lack of element to swap in
        println!("swap_remove(0): {}", vv.swap_remove(0)); // <-- would panic, out of bounds}
}
