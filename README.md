# Cargo-Script

```rust
#!/usr/bin/env cargo +nightly -Zscript
---
[dependencies]
dep = { version = "x.y.z", features = ["f1", "f2"] }
---
// then just regular rust
```


 - [cargo-script + rustfmt](https://github.com/rust-lang/rustfmt/issues/6388)
 - [cargo-script tracking issue](https://github.com/rust-lang/cargo/issues/12207)

Will commonly require executable permissions, to operate 'standalone'.
 `chmod u+x`
