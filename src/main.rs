#![doc = include_str!("../README.md")]

use std::env::{args, var};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::Command;

#[doc(hidden)]
fn main() {
    let version = var("RUSTC_VERSION").unwrap_or("1.999.0".to_string());
    let host = var("RUSTC_HOST").unwrap_or("x86_64-unknown-linux-gnu".to_string());

    // parse command line flags for --verbose/-v and --version/-V
    let mut arg_version = false;
    let mut arg_verbose = false;
    for arg in args() {
        match arg.as_str() {
            arg if arg.starts_with("--") => match arg {
                "--version" => {
                    arg_version = true;
                }
                "--verbose" => {
                    arg_verbose = true;
                }
                _ => continue,
            },
            arg if arg.starts_with('-') => {
                if arg.contains('V') {
                    arg_version = true;
                }
                if arg.contains('v') {
                    arg_verbose = true;
                }
            }
            _ => continue,
        }
    }

    if arg_version {
        println!("rustc {version}");
        if arg_verbose {
            println!("binary: rustc-with-version");
            println!("commit-hash: unknown");
            println!("commit-date: unknown");
            println!("host: {host}");
            println!("release: {version}");
        }
    } else {
        let mut command = Command::new("rustc");
        command.args(args().skip(1));
        #[cfg(unix)]
        command.exec();
        #[cfg(not(unix))]
        unimplemented!();
        // .status()
        // .unwrap()
        // .code()
        // .unwrap_or(0)
    }
}
