#!/usr/bin/env -S cargo +nightly -Zscript
--- 
[dependencies]
---
//! # Scope Thread

use std::thread;

fn main() {
        let mut a = vec![1, 2, 3];
        let mut x = 0;
        println!("vec a: {:?}", a);
        println!("vec x: {:?}", x);

        thread::scope(|s| {
                s.spawn(|| {
                        println!("hello from the first scoped thread");
                        // We can borrow `a` here.
                        dbg!(&a);
                });
                s.spawn(|| {
                        println!("hello from the second scoped thread");
                        // We can even mutably borrow `x` here,
                        // because no other threads are using it.
                        x += a[0] + a[2];
                });
                println!("hello from the main thread");
        });

        // After the scope, we can modify and access our variables again:
        a.push(4);
        println!("vec a: {:?}", a);
        println!("vec x: {:?}", x);
}
