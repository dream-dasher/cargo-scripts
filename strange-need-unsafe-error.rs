#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
clap={ version="4.5", features=["derive"] }
derive_more={version="1", features=["display"]}
tracing="0.1"
tracing-subscriber="0.3"
---
//! # Exploratory bin.
//!
//! 100_000 runs x LEN=5
//! > 2^5 32
//! > Recursive: 974ns
//! > Loop: 354ns
//!
//! 10 runs x LEN=500
//! > 2^20 1_048_576
//! > Recursive: 43.309095ms
//! > Loop: 6.701533ms
//!
//!
/*!
0   | * * * *  |
1a  | +        |

1b  |   +      |
2b  | + +      |

1c  |     +    |
2c. | +   +    |
2c: |   + +    |

1d  |       +  |
2d. | +     +  |
2d: |   +   +  |
2d; |     + +  |
**/
use std::{fmt::Debug, time::Instant};

use clap::Parser;
use dirty_terminal::{clear_screen_ansi, dirty_pause};
use tracing::{Level, event, instrument};

// use day07::Result;
type Result<T> = std::result::Result::<T, Box<dyn std::error::Error>>;

const LEN: usize = 5;
fn main() -> Result<()> {
        let args = Args::parse();
        tracing_subscriber::fmt::init();
        let b_arr = [Symbol::A; LEN];
        // let mut all = Vec::with_capacity(2_usize.pow(LEN as u32));
        let mut durations_recursive = Vec::new();
        // let mut durations_loop = Vec::new();
        for _ in 0..args.num_runs {
                {
                        // recursive
                        let mut all = Vec::with_capacity(2_usize.pow(LEN as u32));
                        let start = Instant::now();
                        // all.extend(vec![b_arr]);
                        // all.extend(vec![PatWithExe { pat: vec![b_arr], idx: 0, i: 0}]);
                        all.extend(combinatorial_ordered_tree_recursive(b_arr, 0, 0, args.manual_mode));
                        durations_recursive.push(start.elapsed());
                        {
                                let solution_len = all.len();
                                all.sort_unstable_by_key(|s| s.top_down_order);
                                for pat in all {
                                        println!("{}", pat);
                                }
                                println!("all length: {:?}", solution_len);
                                println!("2^{} {}", LEN, 2_usize.pow(LEN as u32));
                        }
                }
                // {
                //         // loop
                //         // let mut all = Vec::with_capacity(2_usize.pow(LEN as u32));
                //         let start = Instant::now();
                //         let all: Vec<[Symbol; LEN]> = combinatorial_ordered_tree_loop(args.manual_mode);
                //         durations_loop.push(start.elapsed());
                //         {
                //                 let solution_len = all.len();
                //                 for pat in all {
                //                         println!("{:?}", pat);
                //                 }
                //                 println!("all length: {:?}", solution_len);
                //                 println!("2^{} {}", LEN, 2_usize.pow(LEN as u32));
                //         }
                // }
        }

        // let avg_recursive =
        //         durations_recursive.iter().sum::<std::time::Duration>() / <u32>::try_from(args.num_runs).unwrap();
        // let avg_loop = durations_loop.iter().sum::<std::time::Duration>() / <u32>::try_from(args.num_runs).unwrap();
        // println!("Recursive: {:?}", avg_recursive);
        // println!("Loop: {:?}", avg_loop);
        Ok(())
}

/// For Debugging and visualizing flow
#[derive(Debug, derive_more::Display, Clone, Copy)]
#[display("{:>6} {:?}  -- {} {}", top_down_order, pat, local_idx, local_i)]
struct PatWithExe<const N: usize> {
        pub pat: [Symbol; N],
        pub top_down_order: usize,
        pub local_idx: usize,
        pub local_i: usize,
}
/// Generate (without repetition) all combinations of values
/// AND do so in a way that has a known ordering, which allows for branch pruning during generation.
/// This operates with the intuitive recursive approach.
#[instrument(ret(level = Level::TRACE))]
fn combinatorial_ordered_tree_recursive<const N: usize>(
        arr: [Symbol; N],
        mut top_down_order: usize,
        idx: usize,
        manual_mode: bool,
) -> Vec<PatWithExe<N>>
where
        Symbol: Copy,
{
        // let mut out: Vec<[Symbol; N]> = Vec::new();
        let mut out: Vec<PatWithExe<N>> = Vec::new();
        // out.push(arr);
        let to_do = idx..N;
        for i in to_do {
                let mut arr_alt = arr;
                arr_alt[i] = Symbol::B;
                if manual_mode {
                        event![Level::INFO, ?arr_alt, idx, i, "update"];
                        // println!("{:?}, {:?}, {:?}", arr_alt, idx, i);
                        dirty_pause().unwrap();
                }
                top_down_order+=1;
                // out.push(arr_alt);
                out.push(PatWithExe{pat: arr_alt, top_down_order, local_idx: idx, local_i: i});
                if i + 1 < N {
                        out.extend(combinatorial_ordered_tree_recursive(arr_alt, top_down_order+(N-idx), i + 1, manual_mode));
                }
        }
        if manual_mode { clear_screen_ansi() };
        out
}

/// Generate (without repetition) all combinations of values
/// AND do so in a way that has a known ordering, which allows for branch pruning during generation.
/// This operates using a single loop and a reference stack.
#[instrument(ret(level = Level::TRACE))]
fn combinatorial_ordered_tree_loop<const N: usize>(manual_mode: bool) -> Vec<[Symbol; N]> {
        let mut result = vec![[Symbol::A; N]];
        let mut stack = vec![(0, [Symbol::A; N])];

        while let Some((idx, curr_arr)) = stack.pop() {
                for i in idx..N {
                        let mut new_arr = curr_arr;
                        new_arr[i] = Symbol::B;

                        if manual_mode {
                                event!(Level::INFO, ?new_arr, idx, i, "update");
                                dirty_pause().unwrap();
                                clear_screen_ansi();
                        }

                        result.push(new_arr);
                        if i + 1 < N {
                                stack.push((i + 1, new_arr));
                        }
                }
        }

        result
}

#[derive(Clone, Copy, derive_more::Display)]
enum Symbol {
        #[display("_")]
        A,
        #[display("X")]
        B,
}
impl Debug for Symbol {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}

/// Checking our combinatorial value generation.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
        /// Number of runs to perform.
        num_runs:    usize,
        /// Whether each step should be paused. (For review with Tracing @ INFO level.)
        #[arg(long, short, value_enum)]
        manual_mode: bool,
}

mod dirty_terminal {
        use std::{io, io::Write as _};

        use super::*;
        /// Clear terminal screen using ANSI escape code.
        ///
        /// Not the most robust, but decent in a pinch.
        pub fn clear_screen_ansi() {
                // There are ANSI escape codes that can be used to clear the screen!
                const ANSI_CLEAR_SCREEN: &str = "\x1B[2J\x1B[H";
                print!("{}", ANSI_CLEAR_SCREEN);
                std::io::stdout().flush().unwrap();
        }
        /// Quick and dirty pause button so I can watch as program runs.
        pub fn dirty_pause() -> Result<()> {
                println!("Press Enter to continue...");
                let mut _input = String::new();
                let read_in = io::stdin().read_line(&mut _input)?;
                event![Level::DEBUG, ?read_in];
                Ok(())
        }
}

#[cfg(test)]
mod tests {}

