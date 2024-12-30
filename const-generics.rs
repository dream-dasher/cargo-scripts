#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
---
//! # Const Generics (super helpful!)
//! 
//! ## Links
//! [Rust Reference: const generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics)
//! 
//! 
//! ## Allowable types
//! - u8, u16, u32, u64, u128, usize, 
//! - i8, i16, i32, i64, i128, isize, 
//! - char
//! - bool

use std::fmt::Display;
const DIST_FROM_0: usize = 6;
const DIV_BY: u8 = 3;
fn main() {
        println!("Hello from a const generics rust script!");
        let arr_5 = [0,1,2,3,4];
        print_array(arr_5);
        let arr_hi = ['H', 'i', ',', 't', 'h', 'e', 'r', 'e', '.'];
        print_array(arr_hi);

        let arr_x: [i64; DIST_FROM_0 * 2 + 1] = 
                core::array::from_fn(|i| i as i64 - DIST_FROM_0 as i64);
        let arr_mod = mod_array(arr_x, DIV_BY);
        let arr_rem = rem_array(arr_x, DIV_BY);
        println!("raw array:    {:>3?}", arr_x);
        println!("-{l_2}..={l_2} mod 3: {:>3?}", arr_mod, l_2=DIST_FROM_0);
        println!("-{l_2}..={l_2}  %  3: {:>3?}", arr_rem, l_2=DIST_FROM_0);
}

/// Print contents of each item of an array.
fn print_array<D, const N: usize>(arr: [D; N]) 
where D: Display
{
        println!("The const value in this function `N` is: {}", N);
        for i in 0..N {
                println!("{i} element is : {}", arr[i]);
        }
}

/// Calculates values of i64 array mod u8 and returns same sized u8 array.
/// ## Note
/// "mod" is used here to refer to non-negative remainder
/// in rust this is `.rem_euclid`, even though `%` is sometimes called "mod"
fn mod_array<const N: usize>(arr: [i64; N], modulus: u8) -> [u8; N] {
        let mut out: [u8; N] = [0;N];
        for i in 0..N {
                out[i] = arr[i].rem_euclid(modulus as i64) as u8;
        }
        out
}

/// Calculates values of i64 array % i8 and returns same sized i8 array.
/// ## Note
/// `%` is 'signed remainder'
fn rem_array<const N: usize>(arr: [i64; N], modulus: u8) -> [i8; N] {
        use std::ops::Rem;
        let mut out: [i8; N] = [0;N];
        for i in 0..N {
                out[i] = arr[i].rem(modulus as i64) as i8;
        }
        out
}

// Used as a field of a struct.
struct Foo<const N: usize>([i32; N]);
impl<const N: usize> Foo<N> {
    // Used as an associated constant.
    const CONST: usize = N * 4;
}

trait Trait {
    type Output;
}
impl<const N: usize> Trait for Foo<N> {
    // Used as an associated type.
    type Output = [i32; N];
}

