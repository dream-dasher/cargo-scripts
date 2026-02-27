#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"
[dependencies]
indoc = "2.0.5"
json_to_table = "0.9.0"
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.134"
tabled = "0.17.0"
---
//! # Cargo-Script: zlib-tabled.rs
//! 
//! - Effort to effect ratio isn't drawing me.
//!   - Lots of bespoke syntax.
//! - BUT: *nested* tables are promising.
//!   - may be worth revisiting sometime for that.
use std::{collections::BTreeMap, error::Error, ops::AddAssign, result::Result};

use indoc::indoc;
use json_to_table::json_to_table;
use serde_json::json;
use tabled::{Table, Tabled,
             builder::Builder,
             settings::{Style, merge::Merge}};
const LYRICS: &str = indoc!(r#"
    … So, so you think you can tell heaven from hell?
    Blue skies from pain?
    Can you tell a green field from a cold steel rail?
    A smile from a veil?
    Do you think you can tell?
"#);
fn main() -> Result<(), Box<dyn Error>> {
        let word_fmap = {
                let mut fmap: BTreeMap<&str, usize> = BTreeMap::new();
                let string = LYRICS;
                for line in string.lines() {
                        for word in line.split_whitespace() {
                                fmap.entry(word).or_default().add_assign(1);
                        }
                }
                fmap
        };
        let word_info_list = {
                let mut list = Vec::new();
                for (word, count) in word_fmap.iter() {
                        let mut char_fmap: BTreeMap<char, usize> = BTreeMap::new();
                        for c in word.chars() {
                                char_fmap.entry(c).or_default().add_assign(1);
                        }
                        let most_common_char = char_fmap
                                .into_iter()
                                .max_by_key(|&(_, count)| count)
                                .map(|(c, _)| c)
                                .unwrap_or_default();
                        list.push(WordInfo {
                                word,
                                frequency: *count,
                                length: word.len(),
                                most_common_char,
                        });
                }
                list
        };
        println!("--------------------------------------------------------------------------");
        let mut table_from_fmap = Builder::from(word_fmap).build();
        table_from_fmap.with(Style::modern_rounded().remove_horizontal());
        println!("{}\n", table_from_fmap);

        let mut word_info_table = Table::new(&word_info_list);
        word_info_table.with(Style::psql());
        println!("{}\n", word_info_table);

        let data = [['A', 'B', 'B'], ['A', 'W', 'E'], ['Z', 'Z', 'Z']];
        let mut table_for_merge = Table::new(data);
        table_for_merge.with(Merge::horizontal()).with(Merge::vertical());
        println!("{}\n", table_for_merge);

        let mut data_ = [['Q', 'A', 'Z'], ['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'Z', 'Z']];
        for bttm in data_.iter_mut().flat_map(|row| row.iter_mut()) {
                if *bttm != 'A' {
                        *bttm = '.';
                }
        }
        let table_no_merge = Table::new(data_);
        println!("horizontal\n{}\n", table_no_merge);
        
        let data_h = [['Q', 'A', 'Z'], ['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'Z', 'Z']];
        let mut table_hor_merge = Table::new(data_h);
        table_hor_merge.with(Merge::horizontal());
        println!("horizontal\n{}\n", table_hor_merge);

        let data_v = [['Q', 'A', 'Z'], ['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'Z', 'Z']];
        let mut table_vert_merge = Table::new(data_v);
        table_vert_merge.with(Merge::vertical());
        println!("vertical\n{}\n", table_vert_merge);
        
        let data_m = [['Q', 'A', 'Z'], ['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'Z', 'Z']];
        let mut table_attempt_mix_merge = Table::new(data_m);
        table_attempt_mix_merge
                .with(Merge::vertical())
                .with(Merge::horizontal())
                .with(Merge::vertical());
        println!("multiple, to no effect\n{}\n", table_attempt_mix_merge);

        let combined = &[
                table_no_merge.clone().to_string(),
                table_hor_merge.clone().to_string(),
                table_vert_merge.clone().to_string(),
                table_attempt_mix_merge.clone().to_string(),
        ];
        let table_from_combined = Table::new(combined);
        println!("{}\n", table_from_combined);

        let combined_macro = tabled::col![
                tabled::row!["raw", "hor_merge", "vert_merge", "attempt_mix_merge"],
                tabled::row![table_no_merge, table_hor_merge, table_vert_merge, table_attempt_mix_merge,]
        ];
        println!("{}\n", combined_macro);

        /////////////////// JSON ///////////////////
        let serde_json_value = json!(
            [{"name": "Aleix Melon",
              "id": "E00245",
              "role": ["Dev", "DBA"],
              "age": 23,
              "doj": "11-12-2019",
              "married": false,
              "address": {
                  "street": "32, Laham St.",
                  "city": "Innsbruck",
                  "country": "Austria"
                  },
              "referred-by": "E0012"
              },]
        );
        let json_table = json_to_table(&serde_json_value).to_string();
        println!("JSON:!\n{}\n", serde_json_value);
        println!("JSON to Table!\n{}\n", json_table);

        Ok(())
}

/// Data struct
#[derive(Tabled)]
struct WordInfo<'a> {
        word:             &'a str,
        frequency:        usize,
        length:           usize,
        most_common_char: char,
}


