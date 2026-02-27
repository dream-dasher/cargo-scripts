#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
---
//! # Cargo-Script: slice_generics.rs
//! 
//! On trait objects, slices, and what we can say vs what we can return.
//!
//! ## TLDR
//! (primitive type) slices can all be compared because they all have `.len()`, for example
//! However, slice is not a type. (And adjacent types don't have `.len()` or quite the like).
//! a req for `dyn` is that all the trait posessors can be pointed to 
//! with the same-size pointer.
//! This is (~) *immplementable* for slices, but not predefined.
//! And I say `~` as one would have to 'subslice' the slice methods.
//! Plenty of methods that return sized objects, for example.
use std::{error::Error, result::Result};
// use std::{slice::SliceIndex, ::any::Any};
fn main() -> Result<(), Box<dyn Error>> {
        static C_ARR: [char; 9] = ['H','i',',',' ','t','h','e','r','e',];
        let n_vec: Vec<u16> = (0..1000).collect();
        const B_ARR: [bool;2] = [true,false];
        let f_vec: Vec<f32> = vec![1.2,3.4,5.6,7.8];
        
        println!("{:?}", how_long_static(&n_vec, &f_vec));
        println!("{:?}", how_long_static(&C_ARR, &f_vec));
        println!("{:?}", how_long_static(&n_vec, &B_ARR));
        println!("{:?}", how_long_static(&C_ARR, &B_ARR));
        Ok(())
}

/// We can indirectly point out which is bigger but not return the reference.
fn how_long_static<A,B>(a: &[A], b:&[B]) -> bool {
        a.len() >= b.len() 
}



// fn bigger_slice<'a>(a: &'a dyn [Any], b: &'a dyn [Any]) -> &'a dyn [Any] {

//     if len_a >= len_b {
//         a
//     } else {
//         b
//     }
// }


