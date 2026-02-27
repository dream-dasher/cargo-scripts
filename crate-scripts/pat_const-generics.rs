#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
---
//! # Const Generics (super helpful!)
//!
//! 
//! ## Links
//! - [Const Generics (rust reference)](https://doc.rust-lang.org/reference/items/generics.html#const-generics)
//! - [Inline Const (rust blog)](https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html#inline-const-expressions)
//! - [panic! in Const (rust blog)](https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html#panic-in-const-contexts)
//!    - allows const assertion!
//! 
//! ## Allowable types
//! - u8, u16, u32, u64, u128, usize, 
//! - i8, i16, i32, i64, i128, isize, 
//! - char
//! - bool
use std::fmt::Display;

// const assert
const _: () = assert!(0 == 132_u32.rem_euclid(2));
const _: () = assert!(std::mem::size_of_val(_TEMP_HELLO_MESSAGE_STATIC) == 40);
const _: () = assert!(std::mem::size_of_val(HELLO_MESSAGE_CONST) == 40);
static _TEMP_HELLO_MESSAGE_STATIC: &str = "Hello from a const generics rust script!";
// const _: () = assert!(*_TEMP_HELLO_MESSAGE_CONST == *HELLO_MESSAGE_CONST);
const _: () = assert!(std::mem::size_of::<u64>() == 8);
const _: () = assert!(u8::MAX == 255);
const _: () = assert!(u16::MAX == 65_535);
const _: () = assert!(u32::MAX == 4_294_967_295);
const _: () = assert!(u64::MAX == 18_446_744_073_709_551_615);
const _: () = assert!(u128::MAX == 340_282_366_920_938_463_463_374_607_431_768_211_455);
// const store
const HELLO_MESSAGE_CONST: &str = "Hello from a const generics rust script!";
const DIST_FROM_0: usize = 6;
const DIV_BY: u8 = 3;
fn main() {
        println!("{HELLO_MESSAGE_CONST}");
        let arr_5 = [0,1,2,3,4];
        print_array(arr_5);
        let arr_hi = ['H', 'i', ',', 't', 'h', 'e', 'r', 'e', '.'];
        print_array(arr_hi);

        let arr_x: [i64; DIST_FROM_0 * 2 + 1] = 
                core::array::from_fn(|i| i as i64 - DIST_FROM_0 as i64);
        let arr_mod = mod_array(arr_x, DIV_BY);
        let arr_rem = rem_array(arr_x, DIV_BY);
        println!();
        println!("raw array:    {:>3?}", arr_x);
        println!("-{l_2}..={l_2} mod 3: {:>3?}", arr_mod, l_2=DIST_FROM_0);
        println!("-{l_2}..={l_2}  %  3: {:>3?}", arr_rem, l_2=DIST_FROM_0);
}

/// Print contents of each item of an array.
fn print_array<D, const N: usize>(arr: [D; N]) 
where D: Display
{
        println!("\nThe const value, `N`, in this function is: {}", N);
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
