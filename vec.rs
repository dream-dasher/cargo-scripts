#!/usr/bin/env cargo +nightly -Zscript
---
[dependencies]
---
//! Vec : just playing with with some vector methods.
//!

fn main() {

        // Swap_Remove
        {
                println!("Hello from a rust script!");
                let mut vv = vec!['a','b','c'];
                println!("swap_remove(0): {}", vv.swap_remove(0));
                println!("last is first: {:?}", vv);
                println!("swap_remove(1): {}", vv.swap_remove(1));
                // println!("swap_remove(1): {}", vv.swap_remove(1)); <-- would panic, out of bounds}
                println!("swap_remove(0): {}", vv.swap_remove(0)); // <-- allowed despite lack of element to swap in
                // println!("swap_remove(0): {}", vv.swap_remove(0)); // <-- would panic, out of bounds}
        }

        // Leak
        {
                let mut ref_to_forever = {
                        let vv = vec!['a', 'b', 'g'];
                        vv.leak()
                };
                println!("leaked ref: {:?}", ref_to_forever);
                ref_to_forever.swap(0,2);
                println!("leaked ref: {:?}", ref_to_forever);
                std::thread::spawn(move|| {
                        for c in ref_to_forever {
                                eprintln!("from thread: {}", c);
                        }
                });

                // // Won't live long enough!
                // let mut regular_lifetime = {
                //         let mut vv = vec!['a', 'b', 'g'];
                //         &mut vv
                // };
                let mut vv = vec!['A', 'B', 'G'];
                let mut regular_lifetime = &mut vv;
                println!("leaked ref: {:?}", regular_lifetime);
                regular_lifetime.swap(0,2);
                println!("leaked ref: {:?}", regular_lifetime);
                // // Won't live long enough!
                // std::thread::spawn(|| {
                //         for c in regular_lifetime{
                //                 println!("from thread: {}", c);
                //         }
                // });
                std::thread::sleep(std::time::Duration::from_millis(10));
        }
}
