#!/usr/bin/env -S cargo +nightly --quiet -Zscript
---
[dependencies]
clap = { version = "4", features = ["derive"] }
markov = "1.1.0"
owo-colors = "4.1.0"
[profile.dev]
opt-level = 3
---
#![feature(let_chains)]
//! # Cargo-Script: markov-silly.rs
//! 
//! ## Convenience Section
//!
//! ### Shell Commands
//! - direct
//!   - `chmod u+x markov-silly.rs.rs`
//!   - `./markov-silly.rs.rs`
//! - via cargo
//!   - `cargo +nightly -Zsscript markov-silly.rs.rs`
//! - other cargo commands
//!   - `cargo +nightly -Zscript COMMAND *ARGS --manifest-path markov-silly.rs.rs`
//!
//! ### Links
//! - [Cargo Book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script)
//! - [Github: Cargo-Script Tracking](https://github.com/rust-lang/cargo/issues/12207)
use std::{error::Error, result::Result, fs, path::PathBuf};

use clap::Parser;
use markov::Chain;
use owo_colors::OwoColorize as _;
// use petgraph;
const NGRAM_SIZE_DEFAULT: usize = 1;
const NGRAM_SIZE_MAX: usize = 16;
const TEXT_DEFAULT: &str = "July loves Billy.  Billy loves July.  Sarah loves Bob.  Bob loves Margaret.  Margaret loves Bob.  Nobody loves Sarah.";
fn main() -> Result<(), Box<dyn Error>> {
        let args = Args::parse();

        let n_size = args.ngram_size.map_or(NGRAM_SIZE_DEFAULT, |s| (s as usize).max(NGRAM_SIZE_MAX));
        if let Some(size) = args.ngram_size && size as usize > NGRAM_SIZE_MAX {
                eprintln!("Ngram size was a bit large at {}.  We've {} it to {}.", size.blue(), "clipped".yellow(), NGRAM_SIZE_MAX.cyan());
        };
        let text = if let Some(path) = args.file && let Ok(read_string) = fs::read_to_string(path) {
                read_string
        } else { 
                eprintln!("Using {} text.", "default".cyan());
                TEXT_DEFAULT.to_string()
        };
        let mut chain = Chain::of_order(n_size);
        chain.feed_str(&text);
        if args.show_text {
                println!("\n{start}\n{:?}\n{end}", chain.generate_str(), start ="--START--".green(), end="-- END --".green());
        }
        if args.show_graph {
                // let graph = chain.graph();
                // println!("{:?}", petgraph::dot::Dot::new(&graph));
                println!("\nGraph:\n: {:?}", chain.graph());
        }
        Ok(())
}

// String words together with different orders of chaos
#[derive(Parser)]
#[command(version, about)]
struct Args {
        /// size of markov chain's ngram
        #[arg(short, long)]
        ngram_size: Option<u8>,
        /// path to the file to read
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// show generated graph
        #[arg(short='g', long)]
        show_graph: bool,
        /// show generated text
        #[arg(short='t', long, default_value_t=true)]
        show_text: bool,
}
