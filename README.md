There are some crates ([autocfg](https://docs.rs/autocfg), [rustc_version](https://docs.rs/rustc_version), [rustversion](https://docs.rs/rustversion)) that examine the output of `rustc --version [--verbose]` to determine the capabilities of your compiler, such as nightliness. This determination can be used by other crates in their build.rs to conditionally enable these capabilities. Sometimes this is not what you want.

This crate allows you to specify a custom version string, such that you are in control of your own destiny...

# Examples
Simple usage:
 - `RUSTC=rustc-with-version cargo run`

With custom version:
 - `RUSTC_VERSION=1.0.0 RUSTC=rustc-with-version cargo check`
