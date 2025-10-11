# Cargo-Script


## TLDR

```rust
#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
dep = { version = "x.y.z", features = ["f1", "f2"] }
---
//! # comments

//then just regular rust
```


```zsh
FILE=script_name.rs`
```
- run directly
  - ```zsh
    chmod u+x $FILE
    ./$FILE
    ```
- run via cargo
  - ```zsh
    cargo +nightly -Zscript $FILE
    ```
- run other commands on script
  - ```zsh
    COMMAND=clippy
    cargo +nightly $COMMAND *ARGS --manifest-path $FILE -Zscript
    ```
    - e.g.
    - ```zsh
      cargo +nightly add derive_more --manifest-path $FILE -Zscript
      ```

## Ergonomics
- `watchexec` allows diagnostic to be easily run while waiting on rust-analyzer support.
  - e.g.
  - ```zsh
    watchexec --filter $FILE "clear; ./$FILE"
    ```
- Compilation specs can be added in header.
  - Currently prefer: modifying debug mode runtime.
    - alternate: modify shebang line to force release or adjust flags to cargo
      - less discoverable; and complicates non-run actions that aren't reading from sheband line
  - e.g.
  - ```rust
    ---
    package.edition = "2024"
    profile.dev.opt-level = 2
    profile.dev.package."*".opt-level = 2
    [dependencies]
    egui = "0.30.0"
    eframe = {version="0.30.0", default-features=false, features=["glow", "wayland"]}
    ---
    ```

(see **justfile** groups for convenient access to both)

## Links
 - [Cargo book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html?highlight=script#script)
 - [cargo-script tracking issue](https://github.com/rust-lang/cargo/issues/12207)
 - [cargo-script + rustfmt tracking issue](https://github.com/rust-lang/rustfmt/issues/6388)
 - [cargo-script + rust-analyzer tracking issue](https://github.com/rust-lang/rust-analyzer/issues/15318)

## Accessory Scripts

Convenience commands created for working with single scripts & directory.
<img width="709" alt="just-comms-cargo-scripts" src="https://github.com/user-attachments/assets/221acf0c-ae5d-4dec-8c2c-c4a31cb6dd26" />

(Test & utility files for this accessory code are grouped under 'meta-tests'.
These help check that no template text is miset and that compilation style checks output a partition of appropriate files.)
