# Rust Artifact Dependency Example
Examples of unstable feature, [`artifact-dependencies`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies) to better understand feature to help in Build Scripts

## Run

For running, ensure that `Rustup` is installed, `nightly` toolchain is available. 

Then run

```shell
rustup run nightly cargo run -Z bindeps --verbose
```

## Structure

The folder `libhello` contains a library, and `rust_app` is an application that uses `libhello`.

* `libhello` contains a binary artifact as well, whose source code is at `libhello/src/bin/side.rs`