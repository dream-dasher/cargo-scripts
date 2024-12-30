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

- run directly
  - `chmod u+x {{sd_me: script_name}}.rs`
  - `./{{sd_me: script_name}}.rs`
- run via cargo
  - `cargo +nightly -Zsscript {{sd_me: script_name}}.rs`
- run other commands on script
  - `cargo +nightly COMMAND *ARGS --manifest-path {{sd_me: script_name}}.rs -Zscript`
    - e.g. `cargo +nightly update --manifest-path {{sd_me: script_name}}.rs -Zscript`

## Ergonomics

- The shebang-line can be modified to run in release or quiet modes.
  - e.g. `#!/usr/bin/env -S cargo +nightly --quiet -Zscript run --release --manifest-path`
- `watchexec` allows diagnostic to be easily run while waiting on rust-analyzer support.
  - e.g. `watchexec --filter {{file}} 'clear; ./file.rs'`

(see **justfile** groups for convenient access to both)

## Links
 - [Cargo book: Script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html?highlight=script#script)
 - [cargo-script + rustfmt tracking issue](https://github.com/rust-lang/rustfmt/issues/6388)
 - [cargo-script tracking issue](https://github.com/rust-lang/cargo/issues/12207)

## Accessory Scripts

Convenience commands created for working with single scripts & directory.
```just
available recipes:
    init                            # Ready all local `.rs` files.
    cargo-script command file *args # Cargo _ on script file.
    cargo-script-all command *args  # Cargo _ on ALL `.rs` files at current directory level.

    [create]
    new name                        # New script, with executable user privileges.
    new-clap name                   # New script, with executable user privileges.

    [general]
    check file                      # Linting, formatting, typo checking, etc.
    docs-gen                        # Show general use docs.
    docs file                       # Show docs for a script.
    perf-script file *args          # Run performance analysis on a package.

    [modify]
    simple-script file              # Modify shebang: run without flags. (default)
    quiet-script file               # Modify shebang: use`--quiet` when called directly.
    heavy-script file               # Modify shebang: use `--release` when called directly.
    stable-script file              # Modify shebang: use `--release` & `--quiet` when called directly.

    [watch]
    watch file                      # Run a file when it changes.
    watch-quiet file                # Run a file, without warnings, when it changes.
    watch-check file                # Lint & test a file when it changes.
    watch-check-run file            # Lint & test then run a file when it changes.
``` 

(Test & utility files for this accessory code are grouped under 'meta-tests'.
These help check that no template text is miset and that compilation style checks output a partition of appropriate files.)
