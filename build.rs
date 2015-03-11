use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("make").arg("-e").spawn().and_then(|mut p| p.wait()).unwrap();
    println!("cargo:rustc-flags=-L native={} -l static=wiringpi", out_dir);
}