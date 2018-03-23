extern crate chrono;
use chrono::prelude::*;

use std::process::Command;
fn main() {
    // note: add error checking yourself.
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    
    let build_date = Local::now();
    let build_date = build_date.format("%d-%m-%y %H:%M:%S");
    
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
}