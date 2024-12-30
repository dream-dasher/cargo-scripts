//! # local cargo-xtask
//!
//! It's unclear to me that I have a use for this currently.
//! It overlaps with `justfile` functionality.
//! But, while it offers strengths in complex custom cases,
//! it it less quickly legible when the justfile commands are
//! almost entirely series of shell command calls.
//!
//! Calling shells commands from rust doesn't simplify the
//! call logic.  And doesn't seem like it would be abile to
//! assist with fragility much.
//!
//! Only in cases where I had complex behavior would xtask feel
//! like it brought something to the table. (or, perhaps, collaboration)
//!
//! Note: as long as  (1)`xtask/` is displayed prominently in root
//!       and (2) a clap interface with command descriptions is present
//!       then basic command discoverability should be on par with just

use clap::Parser;
use owo_colors::{self as _, OwoColorize};

/// xtasks, repo convenience tasks
#[derive(Parser, Debug)]
#[command(version, about, long_about, disable_help_subcommand = true, subcommand_help_heading = "input source")]
enum Args {
        /// say hello
        Hello,
        /// add two numbers
        Add { a: i32, b: i32 },
}

fn main() {
        match Args::parse() {
                Args::Hello => println!("Hello, world"),
                Args::Add { a, b } => {
                        let sum = a + b;
                        let sum = sum.green();
                        let a = a.red();
                        let b = b.blue();
                        println!("The (hex) sum of {a:>16x}  and {b:>16x} is {sum:>16x}");
                        println!("The (dec) sum of {a:>16}  and {b:>16} is {sum:>16}");
                        println!("The (oct) sum of {a:>16o}  and {b:>16o} is {sum:>16o}");
                        println!("The (bin) sum of {a:>16b}  and {b:>16b} is {sum:>16b}");
                }
        }
}
